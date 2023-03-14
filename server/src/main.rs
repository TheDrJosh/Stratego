use api::GameStoreState;
use rocket::fs::{FileServer, Options};

#[macro_use]
extern crate rocket;

mod api;
mod web_app;
mod util;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(GameStoreState::default())
        .mount("/", web_app::web_app())
        .mount("/api", api::api())
        .mount("/static", FileServer::new("../web/dist", Options::None))
}
