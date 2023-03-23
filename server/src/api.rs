use std::collections::HashMap;

use common::game_logic;
use common::game_logic::MoveError;
use common::game_logic::MoveResult;
use common::Board;
use common::BoardState;
use common::GameInfo;
use common::InitSetupError;
use common::InitState;
use common::Piece;
use common::PieceMove;
use common::PieceType;
use common::Side;
use common::UserToken;
use rocket::http::Status;
use rocket::response::status;
use rocket::tokio::sync::broadcast;
use rocket::tokio::sync::broadcast::Receiver;
use rocket::tokio::sync::broadcast::Sender;
use rocket::{serde::json::Json, tokio::sync::Mutex, Route, State};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::util::SideGard;
use crate::util::UuidGard;

pub fn api() -> Vec<Route> {
    routes![
        create_game,
        game_exists,
        join_game,
        get_game_state,
        get_game_state_changed,
        move_piece,
        init_setup,
        join_random_game
    ]
}

pub struct GameStoreState {
    games: Mutex<HashMap<Uuid, GameState>>,
    bot_games: Mutex<Vec<Uuid>>,
    changed_games: Sender<Uuid>,
    _cgr: Receiver<Uuid>,
}

pub struct GameState {
    pub board: Board,
    pub primary_side: Side,
    pub clients: HashMap<Uuid, (Option<Side>, Receiver<Uuid>)>,
    pub active_side: Side,
    pub ready: HashMap<Side, bool>,
}

impl GameState {
    pub fn new(primary_side: Side) -> Self {
        Self {
            board: Board::new(),
            primary_side: primary_side.clone(),
            clients: HashMap::new(),
            active_side: primary_side,
            ready: HashMap::new(),
        }
    }
    pub fn has_primary(&self) -> bool {
        let primary_side = self.primary_side.clone();
        self.clients
            .values()
            .find(move |&(side, _)| side == &Some(primary_side.clone()))
            .is_some()
    }
    pub fn has_secondary(&self) -> bool {
        let secondary_side = !self.primary_side.clone();
        self.clients
            .values()
            .find(move |&(side, _)| side == &Some(secondary_side.clone()))
            .is_some()
    }
    pub fn ready(&self) -> bool {
        self.ready.get(&Side::Red).unwrap_or(&false)
            == self.ready.get(&Side::Blue).unwrap_or(&false)
    }
}

impl Default for GameStoreState {
    fn default() -> Self {
        let (send, recv) = broadcast::channel(16);
        Self {
            games: Default::default(),
            bot_games: Default::default(),
            changed_games: send,
            _cgr: recv,
        }
    }
}

#[post("/create_game", format = "json", data = "<game_info>")]
async fn create_game(game_states: &State<GameStoreState>, game_info: Json<GameInfo>) -> Json<Uuid> {
    let game_info = game_info.0;

    let id = Uuid::new_v4();

    game_states
        .games
        .lock()
        .await
        .insert(id, GameState::new(game_info.primary_side));
    if game_info.vs_bot {
        game_states.bot_games.lock().await.push(id);
    }

    id.into()
}

#[get("/<id>/game_exists", format = "json")]
async fn game_exists(game_states: &State<GameStoreState>, id: UuidGard) -> Json<bool> {
    let id = id.0;
    if game_states.games.lock().await.contains_key(&id) {
        true
    } else {
        false
    }
    .into()
}

#[get("/<id>/join", format = "json")]
async fn join_game(
    game_states: &State<GameStoreState>,
    id: UuidGard,
) -> Result<Json<UserToken>, status::NotFound<String>> {
    let id = id.0;

    let mut games = game_states.games.lock().await;
    let bot_games = game_states.bot_games.lock().await;

    let game = games
        .get_mut(&id)
        .ok_or(status::NotFound("Game does not exist".to_owned()))?;

    let mut join_side = None;

    if !game.has_primary() {
        join_side = Some(game.primary_side.clone());
    } else {
        if !game.has_secondary() && !bot_games.contains(&id) {
            join_side = Some(!game.primary_side.clone());
        }
    }
    let user_id = Uuid::new_v4();
    game.clients.insert(
        user_id,
        (join_side.clone(), game_states.changed_games.subscribe()),
    );

    Ok(UserToken {
        access_toket: user_id,
        side: join_side,
    }
    .into())
}

#[get("/<id>/game_state/<user_token>", format = "json")]
async fn get_game_state(
    game_states: &State<GameStoreState>,
    id: UuidGard,
    user_token: UuidGard,
) -> Result<Json<BoardState>, status::NotFound<String>> {
    //TODO! filp for other side
    let id = id.0;
    let user_token = user_token.0;

    let games = game_states.games.lock().await;
    let game = {
        let game = games.get(&id);
        game.ok_or(status::NotFound("Game does not exist!".to_owned()))
    }?;

    let board_state = BoardState {
        board: game.board.clone(),
        active_side: game.active_side.clone(),
        ready: game.ready(),
    };

    Ok(board_state.into())
}

#[get("/<id>/game_state_changed/<user_token>", format = "json")]
async fn get_game_state_changed(
    game_states: &State<GameStoreState>,
    id: UuidGard,
    user_token: UuidGard,
) -> Result<Json<bool>, status::Custom<String>> {
    let id = id.0;
    let user_token = user_token.0;

    let mut games = game_states.games.lock().await;
    let game = games.get_mut(&id).ok_or(status::Custom(
        Status::NotFound,
        "Game does not exist!".to_owned(),
    ))?;

    let (_, recv) = game.clients.get_mut(&user_token).ok_or(status::Custom(
        Status::Unauthorized,
        "Not an active user".to_owned(),
    ))?;

    let mut changed = false;

    while !recv.is_empty() {
        changed |= recv.recv().await.unwrap() == id;
    }

    Ok(changed.into())
}

#[put("/<id>/move_piece", format = "json", data = "<piece_move>")]
async fn move_piece(
    game_states: &State<GameStoreState>,
    id: UuidGard,
    piece_move: Json<PieceMove>,
) -> Json<MoveResult> {
    let id = id.0;
    let piece_move = piece_move.0;

    if let Some(game) = game_states.games.lock().await.get_mut(&id) {
        if let Some((Some(side), _)) = game.clients.get(&piece_move.access_token) {
            if let Some(_piece) = game.board.0 .0.iter().find(|piece| {
                if let Some(piece) = piece {
                    &piece.owner == side && piece.id == piece_move.piece_id
                } else {
                    false
                }
            }) {
                return Json::from(game_logic::move_piece(
                    &mut game.board,
                    piece_move.piece_id,
                    piece_move.x,
                    piece_move.y,
                ));
            }
        }
    }

    game_states.changed_games.send(id).unwrap();

    Json::from(Err(MoveError::PieceDoesNotExist(piece_move.piece_id)))
}

#[post("/<id>/init_setup", format = "json", data = "<init_state>")]
async fn init_setup(
    game_states: &State<GameStoreState>,
    id: UuidGard,
    init_state: Json<InitState>,
) -> Result<(), status::BadRequest<Json<InitSetupError>>> {
    let id = id.0;
    let init_state = init_state.0;

    let mut piece_count = HashMap::new();

    for i in 0..40 {
        let t = &init_state.pieces[i];
        let c = piece_count.get(&t).unwrap_or(&0);
        piece_count.insert(t, *c + 1);
    }

    let mut correct_piece_count = true;

    for piece_type in PieceType::iter() {
        correct_piece_count &=
            *piece_count.get(&piece_type).unwrap_or(&0) == piece_type.starting_count();
    }

    if !correct_piece_count {
        return Err(status::BadRequest(Some(Json::from(
            InitSetupError::IncorrectPieceCount,
        ))))?;
    }

    if let Some(game) = game_states.games.lock().await.get_mut(&id) {
        if let Some((Some(side), _)) = game.clients.get(&init_state.access_token) {
            for i in 0..40 {
                let index = if side == &game.primary_side {
                    60 + i
                } else {
                    39 - i
                };
                game.board.0[index] = Some(Piece {
                    id: Uuid::new_v4(),
                    owner: side.clone(),
                    piece_type: init_state.pieces[i].clone(),
                });
            }
            game_states.changed_games.send(id).unwrap();
        } else {
            return Err(status::BadRequest(Some(Json::from(
                InitSetupError::InvalidAccess,
            ))))?;
        }
    } else {
        return Err(status::BadRequest(Some(Json::from(
            InitSetupError::GameDoesNotExist,
        ))))?;
    }
    Ok(())
}

#[get("/join_random/<side>", format = "json", rank = 1)]
async fn join_random_game(
    game_states: &State<GameStoreState>,
    side: SideGard,
) -> Json<(Uuid, UserToken)> {
    todo!()
}
