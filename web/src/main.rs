use yew::prelude::*;
use yew_router::prelude::*;

//consider removing yew routing and just use static web pages with rocket for everything but the game


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/select")]
    GameSelect,
    #[at("/Game/:id")]
    Game,
}

#[function_component(Acount)]
fn acount() -> Html {

    
    html! {
        <acount>
            <imgt>{"acount image"}</imgt>
            <name>{"acount name"}</name>
        </acount>
    };

    html! {
        <acount>
            <login>{"Login"}</login>
        </acount>
    }
}


#[function_component(Header)]
fn header() -> Html {
    html! {
        <header>
            <Link<Route> to={Route::Home} classes={"title"}>
                {"Stratego"}
            </Link<Route>>
            <spacer/>
            <Acount/>
        </header>
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! { 
        <home>
            <Link<Route> to={Route::GameSelect} classes={"play"}>{"Play Now!"}</Link<Route>>
            <p>{"this is a description"}</p>
        </home> 
    }
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{
            <Home/>
        },
        Route::GameSelect => html!{
            html! {
                {"select"}
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
