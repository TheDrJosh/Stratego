use common::{request, PieceType, Side};
use strum::IntoEnumIterator;
use uuid::Uuid;
use yew::prelude::*;
use yew_hooks::use_async;

struct GameState {
    setup: bool,
}
impl GameState {
    pub fn new() -> Self {
        Self { setup: true }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(Game)]
pub fn game(props: &Props) -> Html {
    let state = use_state(|| GameState::new());

    let user_access = {
        let id = props.id.clone();
        use_async(async move { request::join_game(id).await.map_err(|err| err.to_string()) })
    };

    {
        let user_access = user_access.clone();
        use_effect_with_deps(
            move |_| {
                user_access.run();
            },
            (),
        );
    }

    if !user_access.loading {
        if let Some(user_access) = &user_access.data {
            html! {
                <game>
                    <Board/>
                    {
                        if state.setup {
                            html! {
                                <SetupBar />
                            }
                        } else {
                            html!{}
                        }
                    }
                </game>
            }
        } else {
            if let Some(err) = &user_access.error {
                html! {
                    <error>
                        { format!("Error: {}", err) }
                    </error>
                }
            } else {
                html! {}
            }
        }
    } else {
        html! {
            <loading>
                {"Loading ..."}
            </loading>
        }
    }
}

#[function_component(Board)]
fn board() -> Html {
    let mut pieces = Vec::new();

    for i in 0..100 {
        let x = i % 10;
        let y = i / 10;
        pieces.push(html! {
            <Piece side={Side::Red} piece_type={PieceType::Spy} {x} {y}/>
        });
    }

    html! {
        <board>
            {pieces}
        </board>
    }
}

#[derive(Properties, PartialEq)]
pub struct PieceProps {
    side: Side,
    piece_type: PieceType,
    x: usize,
    y: usize,
}

#[function_component(Piece)]
fn piece(props: &PieceProps) -> Html {
    // html! {
    //     <piece style={format!("grid-column: {}; grid-row: {};", props.x + 1, props.y + 1)}>
    //         <img src={format!("/static/assets/temp/{}/{} {}.webp", props.side.to_string(), props.side.to_string(), props.piece_type.to_string().to_lowercase())}/>
    //     </piece>
    // }
    html! {
        <img style={format!("grid-column: {}; grid-row: {};", props.x + 1, props.y + 1)} src={format!("/static/assets/temp/{}/{} {}.webp", props.side.to_string(), props.side.to_string(), props.piece_type.to_string().to_lowercase())}/>
    }
}

#[function_component(SetupBar)]
fn setup_bar() -> Html {
    let mut pieces = Vec::new();
    for piece_type in PieceType::iter() {
        pieces.push(html! {
            <piece_box>
                <placeholder>
                    {piece_type.to_string()}
                </placeholder>
                <count>
                    {"5"}
                </count>
            </piece_box>
        });
    }

    html! {
        <setup_bar>
            {pieces}
        </setup_bar>
    }
}
