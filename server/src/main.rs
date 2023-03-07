use std::str::FromStr;

use common::{GameInfo, Side};
use rocket::{
    fs::{FileServer, NamedFile, Options},
    request::{FromParam, FromRequest},
    response::status,
};
use strum::{EnumString, ParseError};

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
#[get("/create_game")]
fn create_game() -> String {
    123.to_string()
}
#[get("/join_random_game/<team>")]
fn join_random_game(team: &str) -> Result<String, &str> {
    match Side::from_str(team) {
        Ok(team) => match team {
            Side::Red => Ok(12.to_string()),
            Side::Blue => Ok(21.to_string()),
        },
        Err(_err) => Err("unable to parse team"),
    }
}
#[get("/create_bot_game/<team>")]
fn create_bot_game(team: &str) -> Result<String, &str> {
    match Side::from_str(team) {
        Ok(team) => match team {
            Side::Red => Ok(12.to_string()),
            Side::Blue => Ok(21.to_string()),
        },
        Err(_err) => Err("unable to parse team"),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![app, app_page])
        .mount(
            "/api",
            routes![create_game, join_random_game, create_bot_game],
        )
        .mount("/static", FileServer::new("../web/dist", Options::None))
}
