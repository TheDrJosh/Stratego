use common_comps::*;
use game::Game;
use home::Home;
use select_game::SelectGame;
use yew::prelude::*;
use yew_router::prelude::*;
mod common_comps;
mod game;
mod home;
mod select_game;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home/>
        },
        Route::GameSelect => html! {
            html! {
                <SelectGame/>
            }
        },
        Route::Game { id } => html! {
            <Game {id} />
        },
        Route::NotFound => html! {
            <h1>{"404"}</h1>
        },
    }
}
#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();

    yew::Renderer::<App>::new().render();
}
