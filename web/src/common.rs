use yew::prelude::*;
use yew_router::prelude::*;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/select")]
    GameSelect,
    #[at("/Game")]
    Game,
}


#[function_component(Acount)]
pub fn acount() -> Html {
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
pub fn header() -> Html {
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
