use yew::prelude::*;
use yew_router::prelude::*;

pub enum MenuState {
    GameSelect,
    FriendSelect,
    TeamSelect(GameType),
    JoinSelect,
    Wait(GameType),
}

pub enum GameType {
    Random,
    Friend,
    Computer,
}

#[function_component(SelectGame)]
pub fn select_game() -> Html {
    let menu_state = use_state(|| MenuState::GameSelect);

    let change_state = {
        let menu_state = menu_state.clone();
        Callback::from(move |state| menu_state.set(state))
    };

    match *menu_state {
        MenuState::GameSelect => html! {
            <GameSelect {change_state}/>
        },
        MenuState::FriendSelect => html! {
            <FriendSelect/>
        },
        MenuState::TeamSelect(_) => html! {
            <TeamSelect/>
        },
        MenuState::JoinSelect => todo!(),
        MenuState::Wait(_) => todo!(),
    }
    
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub change_state: Callback<MenuState>,
}

#[function_component(GameSelect)]
fn game_select(props: &Props) -> Html {

    let onclick1 = {
        let change_state = props.change_state.clone();
        Callback::from(move |_| change_state.emit(MenuState::FriendSelect))
    };
    let onclick2 = {
        let change_state = props.change_state.clone();
        Callback::from(move |_| change_state.emit(MenuState::TeamSelect(GameType::Random)))
    };
    let onclick3 = {
        let change_state = props.change_state.clone();
        Callback::from(move |_| change_state.emit(MenuState::TeamSelect(GameType::Computer)))
    };

    html! {
        <select_game>
            <h1>{"Pick who to fight!"}</h1>
            <button onclick={onclick1}>{"vs. Friend"}</button>
            <button onclick={onclick2}>{"vs. Random"}</button>
            <button onclick={onclick3}>{"vs. Computer"}</button>
        </select_game>
    }
}

#[function_component(TeamSelect)]
fn team_select() -> Html {
    html! {
        <select_game>
            <h1>{"Pick a Team"}</h1>
            <button>{"Red"}</button>
            <button>{"Blue"}</button>
        </select_game>
    }
}

#[function_component(FriendSelect)]
fn freind_select() -> Html {
    html! {
        <select_game>
            <button>{"Create Game"}</button>
            <button>{"Join Game"}</button>
        </select_game>
    }
}
fn join_select() -> Html {
    html! {
        <select_game>
            <input type={"text"}/>
            <button>{"Join Game"}</button>
        </select_game>
    }
}

/*
1. game_select -> 1. freind_select | 2. team_select(rand) | 3. team_select(comp)
2. freind_select -> 1. team_select(friend) | 2. join_select
3. team_select(rand) -> wait(rand)
4. team_select(comp) -> game
5. team_select(friend) -> wait(friend)
6. join_select -> game
*/
