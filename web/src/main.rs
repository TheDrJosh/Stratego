use yew::prelude::*;
use yew_router::prelude::*;


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

#[function_component(Header)]
fn other() -> Html {
    html! {
        <header>
            <h1>
                {"Stratego"}
            </h1>
        </header>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
    }
}
#[function_component]
fn App() -> Html {


    html! {
        <>
            <Header />
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}