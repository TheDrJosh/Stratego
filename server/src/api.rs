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



#[get("/game_exists/<id>")]
async fn game_exists(game_states: &State<GameStoreState>, id: UuidGard) -> Json<bool> {
    let id = id.0;
    if game_states.games.lock().await.contains_key(&id) {
        true
    } else {
        false
    }.into()
}

#[get("/join/<id>")]
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
    games.get_mut(&id).unwrap().clients.insert(user_id, join_side.clone());


    UserToken {
        access_toket: user_id,
        side: join_side,
    }.into()

}




#[get("/game_state/<id>")]
async fn get_game_state(game_states: &State<GameStoreState>, id: UuidGard) -> Json<Option<Vec<Option<Piece>>>> {
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



#[put("/move_piece/<id>", format = "json", data = "<piece_move>")]
fn move_piece(game_states: &State<GameStoreState>, id: UuidGard, piece_move: Json<PieceMove>) -> String {
    let piece_move = piece_move.0;

    

    



    todo!()
}





#[post("/init_setup/<id>", format = "json", data = "<init_state>")]
async fn init_setup(game_states: &State<GameStoreState>, id: UuidGard, init_state: Json<InitState>) -> String {
    let id = id.0;
    let init_state = init_state.0;


    //correct piece count


    if let Some(game) = game_states.games.lock().await.get_mut(&id) {
        if let Some(Some(side)) = game.clients.get(&init_state.access_token) {
            for i in 0..30 {
                let index = if side == &game.primary_side { 70 + i } else { 29 - i };
                game.board[index] = Some(Piece {
                    id: Uuid::new_v4(),
                    owner: side.clone(),
                    piece_type: init_state.pieces[i].clone(),
                });
            }
        }
    }


    
    
    



    todo!()
}


#[cfg(test)]
mod test {
    use crate::rocket;
    use common::GameInfo;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use rocket::serde::json::Json;
    use uuid::Uuid;

    #[test]
    fn game_connect() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let game_info = rocket::serde::json::to_string(&GameInfo {
            vs_bot: false,
            primary_side: common::Side::Red,
        }).unwrap();

        let mut response = client.post(uri!("/api", super::create_game)).body(game_info).dispatch();


        assert_eq!(response.status(), Status::Ok);

        let game_id: Uuid = response.into_json().unwrap();
        //UserToken

       // let mut response = client.get(uri!("/api", super::join_game: id = game_id)).body(game_info).dispatch();


    }
}

