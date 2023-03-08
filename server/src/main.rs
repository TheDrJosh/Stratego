use std::str::FromStr;

use common::{GameInfo, Side};
use rocket::{
    fs::{FileServer, NamedFile, Options},
    request::{FromParam, FromRequest},
    response::status,
};
use strum::{EnumString, ParseError};
use rocket::serde::json::Json;

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
fn create_game(game_info: Json<GameInfo>) -> String {
    let game_info = game_info.0;

    

    123.to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![app, app_page])
        .mount(
            "/api",
            routes![create_game],
        )
        .mount("/static", FileServer::new("../web/dist", Options::None))
}
