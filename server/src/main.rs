use std::{collections::HashMap, str::FromStr};

use common::{GameInfo, GameState, Side};
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::{
    fs::{FileServer, NamedFile, Options},
    request::{FromParam, FromRequest},
    response::status,
    State,
};
use strum::{EnumString, ParseError};
use uuid::Uuid;

#[macro_use]
extern crate rocket;

#[derive(EnumString)]
enum Route {
    WebApp,
}

impl<'a> FromParam<'a> for Route {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        if param != "api" && param != "static" {
            Ok(Route::WebApp)
        } else {
            Err(param)
        }
    }
}

#[get("/<_page>/<_..>", rank = 1)]
async fn app_page(_page: Route) -> NamedFile {
    NamedFile::open("../web/dist/index.html")
        .await
        .expect("could not find index.html")
}
#[get("/")]
async fn app() -> NamedFile {
    NamedFile::open("../web/dist/index.html")
        .await
        .expect("could not find index.html")
}

// api
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

struct UuidGard(Uuid);

impl<'a> FromParam<'a> for UuidGard {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match Uuid::from_str(param) {
            Ok(id) => Ok(UuidGard(id)),
            Err(_) => Err(param),
        }
    }
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

#[derive(Default)]
struct GameStoreState {
    games: Mutex<HashMap<Uuid, GameState>>,
    bot_games: Mutex<Vec<Uuid>>,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(GameStoreState::default())
        .mount("/", routes![app, app_page])
        .mount("/api", routes![create_game, game_exists])
        .mount("/static", FileServer::new("../web/dist", Options::None))
}
