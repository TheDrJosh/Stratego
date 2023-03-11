use std::{collections::HashMap};

use common::GameInfo;
use common::GameState;
use common::InitState;
use common::Piece;
use common::PieceMove;
use common::UserToken;
use rocket::{serde::json::Json, tokio::sync::Mutex, Route, State};
use uuid::Uuid;
use crate::util::UuidGard;

pub fn api() -> Vec<Route> {
    routes![create_game, game_exists, join_game, get_game_state, move_piece, init_setup]
}

#[derive(Default)]
pub struct GameStoreState {
    games: Mutex<HashMap<Uuid, GameState>>,
    bot_games: Mutex<Vec<Uuid>>,
}

#[post("/create_game", format = "json", data = "<game_info>")]
async fn create_game(game_states: &State<GameStoreState>, game_info: Json<GameInfo>) -> String {
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

    id.to_string()
}



#[get("/game_exists/<id>")]
async fn game_exists(game_states: &State<GameStoreState>, id: UuidGard) -> String {
    let id = id.0;
    if game_states.games.lock().await.contains_key(&id) {
        true
    } else {
        false
    }
    .to_string()
}

#[get("/join/<id>")]
async fn join_game(game_states: &State<GameStoreState>, id: UuidGard) -> String {
    let id = id.0;

    let games = game_states.games.lock().await;
    let bot_games = game_states.bot_games.lock().await;

    let game = games.get(&id).unwrap();

    let join_primary = game
        .clients
        .iter()
        .find(|&x| {
            if let Some(side) = &x.side {
                if side == &game.primary_side {
                    return true;
                }
            }
            false
        })
        .is_some();

    let mut user_token = UserToken {
        access_toket: Uuid::new_v4(),
        side: None,
    };

    if join_primary {
        user_token.side = Some(game.primary_side.clone());
    } else {
        let join_secondary = game
            .clients
            .iter()
            .find(|&x| {
                if let Some(side) = &x.side {
                    if side == &game.primary_side.not() {
                        return true;
                    }
                }
                false
            })
            .is_some();

        if join_secondary && bot_games.contains(&id) {
            user_token.side = Some(game.primary_side.not());
        }
    }

    rocket::serde::json::to_string(&user_token).unwrap()

}




#[get("/game_state/<id>")]
async fn get_game_state(game_states: &State<GameStoreState>, id: UuidGard) -> String {
    let id = id.0;

    let games = game_states.games.lock().await;
    let game = {
        let game = games.get(&id);
        if game.is_none() {
            return rocket::serde::json::to_string(&None::<Vec<Piece>>).unwrap();
        }
        game.unwrap()
    };

    
    let board = Vec::from(game.board.clone());

    rocket::serde::json::to_string(&Some(board)).unwrap()
}



#[put("/move_piece", format = "json", data = "<piece_move>")]
fn move_piece(game_states: &State<GameStoreState>, piece_move: Json<PieceMove>) -> String {
    let piece_move = piece_move.0;


    



    todo!()
}

#[post("/init_setup", format = "json", data = "<init_state>")]
fn init_setup(game_states: &State<GameStoreState>, init_state: Json<InitState>) -> String {
    let init_state = init_state.0;

    
    



    todo!()
}




