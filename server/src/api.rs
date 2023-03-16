use std::collections::HashMap;

use crate::util::UuidGard;
use common::GameInfo;
use common::GameState;
use common::InitSetupReturn;
use common::InitState;
use common::Piece;
use common::PieceMove;
use common::PieceType;
use common::UserToken;
use rocket::{serde::json::Json, tokio::sync::Mutex, Route, State};
use uuid::Uuid;

pub fn api() -> Vec<Route> {
    routes![
        create_game,
        game_exists,
        join_game,
        get_game_state,
        move_piece,
        init_setup
    ]
}

#[derive(Default)]
pub struct GameStoreState {
    games: Mutex<HashMap<Uuid, GameState>>,
    bot_games: Mutex<Vec<Uuid>>,
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
async fn join_game(game_states: &State<GameStoreState>, id: UuidGard) -> Json<UserToken> {
    let id = id.0;

    let mut games = game_states.games.lock().await;
    let bot_games = game_states.bot_games.lock().await;

    let mut game = games.get_mut(&id).unwrap();

    let mut join_side = None;

    if !game.has_primary {
        join_side = Some(game.primary_side.clone());
        game.has_primary = true;
    } else {
        if !game.has_secondary && bot_games.contains(&id) {
            join_side = Some(game.primary_side.not());
            game.has_secondary = true;
        }
    }
    let user_id = Uuid::new_v4();
    games
        .get_mut(&id)
        .unwrap()
        .clients
        .insert(user_id, join_side.clone());

    UserToken {
        access_toket: user_id,
        side: join_side,
    }
    .into()
}

#[get("/<id>/game_state", format = "json")]
async fn get_game_state(
    game_states: &State<GameStoreState>,
    id: UuidGard,
) -> Json<Option<Vec<Option<Piece>>>> {
    let id = id.0;

    let games = game_states.games.lock().await;
    let game = {
        let game = games.get(&id);
        if game.is_none() {
            return Json::from(None);
        }
        game.unwrap()
    };

    let board = Vec::from(game.board.clone());

    Some(board).into()
}

//wait_for_game_state

#[put("/<id>/move_piece", format = "json", data = "<piece_move>")]
async fn move_piece(
    game_states: &State<GameStoreState>,
    id: UuidGard,
    piece_move: Json<PieceMove>,
) -> String {
    let id = id.0;
    let piece_move = piece_move.0;

    if let Some(game) = game_states.games.lock().await.get_mut(&id) {
        if let Some(Some(side)) = game.clients.get(&piece_move.access_token) {
            if let Some(piece) = game.board.iter().enumerate().find(|piece| {
                if let Some(piece) = piece.1 {
                    &piece.owner == side && piece.id == piece_move.piece_id
                } else {
                    false
                }
            }) {
                let x = piece.0 % 10;
                let y = piece.0 / 10;
                let piece = piece.1.clone().unwrap();

                //if x,y to piece_move.x,piece_move.y is valid 
                {
                    
                }

            }
        }
    }

    todo!()
}

#[post("/<id>/init_setup", format = "json", data = "<init_state>")]
async fn init_setup(
    game_states: &State<GameStoreState>,
    id: UuidGard,
    init_state: Json<InitState>,
) -> Json<InitSetupReturn> {
    let id = id.0;
    let init_state = init_state.0;

    let mut piece_count = HashMap::new();

    for i in 0..30 {
        let t = &init_state.pieces[i];
        let c = piece_count.get(&t).unwrap_or(&0);
        piece_count.insert(t, *c + 1);
    }

    let correct_piece_count = piece_count[&PieceType::Bomb] == 6
        && piece_count[&PieceType::Captain] == 4
        && piece_count[&PieceType::Colonel] == 2
        && piece_count[&PieceType::Flag] == 1
        && piece_count[&PieceType::General] == 1
        && piece_count[&PieceType::Lieutenant] == 4
        && piece_count[&PieceType::Major] == 3
        && piece_count[&PieceType::Marshal] == 1
        && piece_count[&PieceType::Miner] == 5
        && piece_count[&PieceType::Scout] == 8
        && piece_count[&PieceType::Sergeant] == 4
        && piece_count[&PieceType::Spy] == 1;

    if !correct_piece_count {
        return Json::from(InitSetupReturn::IncorrectPieceCount);
    }

    if let Some(game) = game_states.games.lock().await.get_mut(&id) {
        if let Some(Some(side)) = game.clients.get(&init_state.access_token) {
            for i in 0..30 {
                let index = if side == &game.primary_side {
                    60 + i
                } else {
                    39 - i
                };
                game.board[index] = Some(Piece {
                    id: Uuid::new_v4(),
                    owner: side.clone(),
                    piece_type: init_state.pieces[i].clone(),
                });
            }
        } else {
            return Json::from(InitSetupReturn::InvalidAccess);
        }
    } else {
        return Json::from(InitSetupReturn::UnknownFail);
    }

    Json::from(InitSetupReturn::UnknownFail)
}
