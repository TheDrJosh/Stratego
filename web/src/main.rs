use common::*;
use home::Home;
use select_game::SelectGame;
use yew::prelude::*;
use yew_router::prelude::*;
mod common;
mod home;
mod select_game;

//consider removing yew routing and just use static web pages with rocket for everything but the game





fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{
            <Home/>
        },
        Route::GameSelect => html!{
            html! {
                <SelectGame/>
            }
        },
        Route::Game => todo!(),
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
    yew::Renderer::<App>::new().render();
}
