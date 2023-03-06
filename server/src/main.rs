use std::str::FromStr;

use rocket::{fs::{FileServer, Options, NamedFile}, request::FromParam};
use strum::{EnumString, ParseError};

#[macro_use] extern crate rocket;

#[derive(EnumString)]
enum Route {
    #[strum(serialize = "select")]
    GameSelect,
}

impl<'a> FromParam<'a> for Route {
    type Error = ParseError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Route::from_str(param)
    }
}

#[get("/<page>")]
async fn app_page(page: Route) -> NamedFile {

    NamedFile::open("../web/dist/index.html").await.expect("could not find index.html")
}
#[get("/")]
async fn app() -> NamedFile {
    NamedFile::open("../web/dist/index.html").await.expect("could not find index.html")
}


#[get("/helloworld")]
fn hello_world() -> &'static str {
    "Hello, world!"

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![hello_world]).mount("/static", FileServer::new("../web/dist", Options::None)).mount("/", routes![app, app_page])
}