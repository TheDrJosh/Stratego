use common::Side;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;


use crate::common_comps::Route;

#[derive(PartialEq, Clone, Debug)]
pub enum MenuState {
    GameSelect,
    FriendSelect,
    TeamSelect(GameType),
    JoinSelect,
    GameRandom(Side),
    GameComputer(Side),
    NewGameFriend(Side),
    JoinGameFriend(u64),
}
#[derive(PartialEq, Clone, Debug)]
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

    let navigator = use_navigator().unwrap();

    match &*menu_state {
        MenuState::GameSelect => html! {
            <GameSelect {change_state}/>
        },
        MenuState::FriendSelect => html! {
            <FriendSelect {change_state}/>
        },
        MenuState::TeamSelect(game_type) => html! {
            <TeamSelect {change_state} game_type={game_type.clone()}/>
        },
        MenuState::JoinSelect => html! {
            <JoinSelect {change_state}/>
        },
        MenuState::GameRandom(side) => {
            join_random_game(&navigator, side);
            html! {
                <Wait game_type={GameType::Random} />
            }
        }
        MenuState::GameComputer(side) => {
            create_bot_game(&navigator, side);
            html! {
                <Wait game_type={GameType::Computer} />
            }
        }
        MenuState::NewGameFriend(side) => {
            create_game(&navigator);
            html! {
                <Wait game_type={GameType::Friend} joining={false}/>
            }
        }
        MenuState::JoinGameFriend(id) => {
            navigator.push(&Route::Game { id: *id });
            html! {
                <Wait game_type={GameType::Friend} joining={true}/>
            }
        }
    }
}

fn change_state_on_click(
    state: MenuState,
    change_state: &Callback<MenuState>,
) -> Callback<MouseEvent> {
    let change_state = change_state.clone();
    Callback::from(move |_| change_state.emit(state.clone()))
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub change_state: Callback<MenuState>,
}

#[function_component(GameSelect)]
fn game_select(props: &Props) -> Html {
    html! {
        <select_game>
            <h1>{"Pick who to fight!"}</h1>
            <button_row>
                <button onclick={change_state_on_click(MenuState::FriendSelect, &props.change_state)}>{"vs. Friend"}</button>
                <button onclick={change_state_on_click(MenuState::TeamSelect(GameType::Random), &props.change_state)}>{"vs. Random"}</button>
                <button onclick={change_state_on_click(MenuState::TeamSelect(GameType::Computer), &props.change_state)}>{"vs. Computer"}</button>
            </button_row>
        </select_game>
    }
}

#[derive(Properties, PartialEq)]
pub struct TeamProps {
    pub change_state: Callback<MenuState>,
    pub game_type: GameType,
}

#[function_component(TeamSelect)]
fn team_select(props: &TeamProps) -> Html {
    let (red, blue) = match props.game_type {
        GameType::Computer => (
            change_state_on_click(MenuState::GameComputer(Side::Red), &props.change_state),
            change_state_on_click(MenuState::GameComputer(Side::Blue), &props.change_state),
        ),
        GameType::Friend => (
            change_state_on_click(MenuState::NewGameFriend(Side::Red), &props.change_state),
            change_state_on_click(MenuState::NewGameFriend(Side::Blue), &props.change_state),
        ),
        GameType::Random => (
            change_state_on_click(MenuState::GameRandom(Side::Red), &props.change_state),
            change_state_on_click(MenuState::GameRandom(Side::Blue), &props.change_state),
        ),
    };

    let back_state = match props.game_type {
        GameType::Random => MenuState::GameSelect,
        GameType::Friend => MenuState::FriendSelect,
        GameType::Computer => MenuState::GameSelect,
    };

    html! {
        <select_game>
            <Back change_state={props.change_state.clone()} prev_menu_state={back_state}/>
            <h1>{"Pick a Team"}</h1>
            <button_row>
                <button onclick={red}>{"Red"}</button>
                <button onclick={blue}>{"Blue"}</button>
            </button_row>
        </select_game>
    }
}

#[function_component(FriendSelect)]
fn freind_select(props: &Props) -> Html {
    html! {
        <select_game>
            <Back change_state={props.change_state.clone()} prev_menu_state={MenuState::GameSelect}/>
            <button_row>
                <button onclick={change_state_on_click(MenuState::TeamSelect(GameType::Friend), &props.change_state)}>{"Create Game"}</button>
                <button onclick={change_state_on_click(MenuState::JoinSelect, &props.change_state)}>{"Join Game"}</button>
            </button_row>
        </select_game>
    }
}

#[function_component(JoinSelect)]
fn join_select(props: &Props) -> Html {
    let state = use_state(|| false);

    let input_ref = use_node_ref();
    let callback = 
    {
        let input_ref = input_ref.clone();
        let navigator = use_navigator().unwrap();
        let state = state.clone();

        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().expect("input_ref not attachhed to element");

            match input.value().parse::<u64>() {
                Ok(id) => {
                    navigator.push(&Route::Game { id });
                },
                Err(err) => {
                    log::warn!("{:?}", err);
                    state.set(true);
                },
            };

        })
    };


    let invalid = if *state {
        html! {
            <invalid>{"invalid"}</invalid>
        }
    } else {
        html! {
        }
    };


    html! {
        <select_game>
            <Back change_state={props.change_state.clone()} prev_menu_state={MenuState::FriendSelect}/>
            <input_row>
                <spacer/>
                <input type={"text"} ref={input_ref}/>
                <spacer/>
            </input_row>
            {invalid}
            <button onclick={callback}>{"Join Game"}</button>
        </select_game>
    }
}

#[derive(Properties, PartialEq)]
pub struct WaitProps {
    pub game_type: GameType,
    pub joining: Option<bool>,
}

#[function_component(Wait)]
fn wait(props: &WaitProps) -> Html {
    let text = match props.game_type {
        GameType::Computer => "Waiting For Match Creation",
        GameType::Friend => {
            if let Some(joining) = props.joining {
                if joining {
                    "Waiting For Server To Respond"
                } else {
                    "Waiting For Play To Join"
                }
            } else {
                "Error"
            }
        }
        GameType::Random => "Waiting For Match To Be Made",
    };

    html! {
        <select_game>
            <h1>{text}</h1>
        </select_game>
    }
}



#[derive(Properties, PartialEq)]
pub struct BackProps {
    pub change_state: Callback<MenuState>,
    pub prev_menu_state: MenuState,
}

#[function_component(Back)]
fn back(props: &BackProps) -> Html {
    html! {
        <back onclick={change_state_on_click(props.prev_menu_state.clone(), &props.change_state)}>{"< Back"}</back>
    }
}


fn join_random_game(navigator: &Navigator, team: &Side) {
    let navigator = navigator.clone();
    let team = team.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let fetched: u64 = Request::get(&format!(
            "http://127.0.0.1:8000/api/join_random_game/{}",
            team
        ))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .parse()
        .unwrap();
        navigator.push(&Route::Game { id: fetched });
    });
}

fn create_game(navigator: &Navigator) {
    let navigator = navigator.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let fetched: u64 = Request::get("http://127.0.0.1:8000/api/create_game")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .parse()
            .unwrap();
        navigator.push(&Route::Game { id: fetched });
    });
}

fn create_bot_game(navigator: &Navigator, team: &Side) {
    let navigator = navigator.clone();
    let team = team.not();
    wasm_bindgen_futures::spawn_local(async move {
        let fetched: u64 = Request::get(&format!(
            "http://127.0.0.1:8000/api/create_bot_game/{}",
            team
        ))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .parse()
        .unwrap();
        navigator.push(&Route::Game { id: fetched });
    });
}

/*
1. game_select -> 1. freind_select | 2. team_select(rand) | 3. team_select(comp)
2. freind_select -> 1. team_select(friend) | 2. join_select
3. team_select(rand) -> wait(rand)
4. team_select(comp) -> game
5. team_select(friend) -> wait(friend)
6. join_select -> game
*/
