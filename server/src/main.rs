use api::GameStoreState;
use rocket::fs::{FileServer, Options};

#[macro_use]
extern crate rocket;

mod api;
mod util;
mod web_app;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(GameStoreState::default())
        .mount("/", web_app::web_app())
        .mount("/api", api::api())
        .mount(
            "/static",
            FileServer::new("../web/dist", Options::None).rank(2),
        )
        .mount(
            "/static",
            FileServer::new("../web/static", Options::None).rank(1),
        )
}
