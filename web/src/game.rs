use common::{request, PieceType, Side, empty_board};
use strum::IntoEnumIterator;
use uuid::Uuid;
use yew::prelude::*;
use yew_hooks::use_async;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

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
            if let Some(side) = &user_access.side {
                if !state.setup {
                    html!{ <GameLogic/> }
                } else {
                    html!{ <SetupGame side={side.clone()} /> }
                }
            } else {
                html!{ <GameLogic/> }
            }
            
        } else {
            if let Some(err) = &user_access.error {
                html! {
                    <error>
                        { format!("Error: {}", err) }
                    </error>
                }
            } else {
                html! { }
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

struct GameLogicState {
    board: common::Board
}

#[function_component(GameLogic)]
fn game_logic() -> Html {

    let state = use_state(|| GameLogicState {
        board: empty_board(),
    });

    let callback = Callback::from(move |e| {
        log::info!("game logic: {:?}", e);
    });


    html! {
        <game>
            <Board on_click={callback} board={state.board.clone()}/>
        </game>
    }
}

#[derive(Properties, PartialEq)]
struct SetupGameProps {
    side: Side,
}

struct SetupGameState {
    board: common::Board,
    selected_piece: Option<PieceType>,
}


#[function_component(SetupGame)]
fn setup_game(props: &SetupGameProps) -> Html {

    let state = use_state(|| SetupGameState {
        board: empty_board(),
        selected_piece: None,
    });

    use_effect(move || {
        let document = gloo::utils::document();
        let state = state.clone();

        let listener = gloo::events::EventListener::new(&document, "keydown", |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            if event.key_code() == 27 { //Escape
                state.set(SetupGameState {
                    board: state.board,
                    selected_piece: None,
                });
            }
        });

        || drop(listener)
    });

    

    

    let bar_callback = Callback::from(|t| {
        log::info!("setup game; bar callback: {:?}", t);
    });
    let board_callback = Callback::from(move |e| {
        log::info!("setup game; board callback: {:?}", e);
    });

    

    html! {
        <game>
            <Board on_click={board_callback} board={state.board.clone()}/>
            <SetupBar side={props.side.clone()} type_select={bar_callback} />
        </game>
    }
}


#[derive(Properties, PartialEq)]
pub struct BoardProps {
    board: common::Board,
    on_click: Callback<(usize, usize, MouseEvent)>,
}



#[function_component(Board)]
fn board(props: &BoardProps) -> Html {
    let mut pieces = Vec::new();

    for i in 0..100 {
        let x = i % 10;
        let y = i / 10;

        let callback = {
            let on_click = props.on_click.clone();
            Callback::from(move |e| {
                on_click.emit((x, y, e));
            })
        };
        if let Some(piece) = &props.board[i] {
            pieces.push(html! {
                <Piece side={Side::Red} piece_type={piece.piece_type.clone()} {x} {y} on_click={callback}/>
            });
        } else {
            pieces.push(html! {
                <empty style={format!("grid-column: {}; grid-row: {};", x + 1, y + 1)} onclick={callback}/>
            });
        }
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
    //#[prop_or_default]
    on_click: Callback<MouseEvent>,
}

#[function_component(Piece)]
fn piece(props: &PieceProps) -> Html {
    html! {
        <img onclick={props.on_click.clone()} style={format!("grid-column: {}; grid-row: {};", props.x + 1, props.y + 1)} src={format!("/static/assets/temp/{} {}.webp", props.side.to_string(), props.piece_type.to_string().to_lowercase())}/>
    }
}

#[derive(Properties, PartialEq)]
pub struct SetupBarProps {
    side: Side,
    type_select: Callback<PieceType>
}

#[function_component(SetupBar)]
fn setup_bar(props: &SetupBarProps) -> Html {
    let mut pieces = Vec::new();
    for piece_type in PieceType::iter() {
        
        
        let callback = {
            let type_select = props.type_select.clone();
            let piece_type = piece_type.clone();
            Callback::from(move |_| {
                let piece_type = piece_type.clone();
                type_select.emit(piece_type);
            })
        };


        pieces.push(html! {
            <piece_box onclick={callback}>
                <img src={format!("/static/assets/temp/{} {}.webp", props.side.to_string(), piece_type.to_string().to_lowercase())}/>
                
                <text>
                    <name>
                        {piece_type.to_string()}
                    </name>
                    <count>
                        {"5"}
                    </count>
                </text>
            </piece_box>
        });
    }

    html! {
        <setup_bar>
            {pieces}
        </setup_bar>
    }
}
