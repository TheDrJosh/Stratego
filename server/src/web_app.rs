use rocket::{fs::NamedFile, request::FromParam};

pub fn web_app() -> Vec<rocket::Route> {
    routes![app, app_page]
}

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

#[get("/<_page>/<_..>")]
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
