use crate::common_comps::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <home>
            <Link<Route> to={Route::GameSelect} classes={"play"}>{"Play Now!"}</Link<Route>>
            <p>{"this is a description"}</p>
        </home>
    }
}
