use std::collections::HashMap;
use std::time::Duration;

use common::game_logic::MoveError;
use common::utils::SendibleArray;
use common::{request, PieceType, Side};
use common::{InitState, UserToken};
use strum::IntoEnumIterator;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_hooks::use_async;


//Convert to struct Component
#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(Game)]
pub fn game(props: &Props) -> Html {
    let setup_state = use_state(|| true);

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

    let setup_callback = {
        let setup_state = setup_state.clone();
        Callback::from(move |_| {
            setup_state.set(false);
        })
    };

    if !user_access.loading {
        if let Some(user_access) = &user_access.data {
            if let Some(_side) = &user_access.side {
                if !*setup_state {
                    html! { <GameLogic game_id={props.id.clone()} access_token={user_access.clone()} /> }
                } else {
                    html! { <SetupGame game_id={props.id.clone()} access_token={user_access.clone()} {setup_callback}/> }
                }
            } else {
                html! { <GameViewer game_id={props.id.clone()} access_token={user_access.clone()} /> }
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

#[derive(Properties, PartialEq)]
struct GameLogicProps {
    game_id: Uuid,
    access_token: UserToken,
}

#[function_component(GameLogic)]
fn game_logic(props: &GameLogicProps) -> Html {
    let board_state = use_state(|| common::Board::new());
    let active_side_state = use_state(|| Option::<Side>::None);
    let selected_state = use_state(|| Option::<(usize, usize)>::None);
    
    {
        let game_id = props.game_id.clone();
        let user_id = props.access_token.access_toket;
        let board_state = board_state.clone();
        let active_side_state = active_side_state.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let mut board = request::get_game_state(game_id, user_id).await.unwrap();

                    board_state.set(board.board);

                    while !board.ready {
                        async_std::task::sleep(Duration::from_secs(2)).await;
                        let changed = request::get_game_state_changed(game_id, user_id)
                            .await
                            .unwrap();
                        if changed {
                            board = request::get_game_state(game_id, user_id).await.unwrap();
                            board_state.set(board.board);
                            active_side_state.set(Some(board.active_side));
                        }
                    }
                });
            },
            (),
        );
    }

    {
        let selected_state = selected_state.clone();
        use_effect(move || {
            let document = gloo::utils::document();

            let listener = gloo::events::EventListener::new(&document, "keydown", move |event| {
                let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                if event.key_code() == 27 {
                    //Escape
                    selected_state.set(None);
                }
            });

            || drop(listener)
        });
    }

    let callback = {
        let selected_state = selected_state.clone();
        Callback::from(move |(x, y, e)| {
            selected_state.set(Some((x, y)));
        })
    };

    html! {
        <game>
        {
            if *active_side_state != props.access_token.side && active_side_state.is_some() {
                html! {
                    <waiting>{format!("Waiting for {}", (*active_side_state).clone().unwrap())}</waiting>
                }
            } else {
                html! { }
            }
        }
            <Board on_click={callback} board={(*board_state).clone()} selected={*selected_state}/>
        </game>
    }
}



#[function_component(GameViewer)]
fn game_viewer(props: &GameLogicProps) -> Html {
    let board_state = use_state(|| common::Board::new());

    let callback = Callback::from(move |e| {
        log::info!("game logic: {:?}", e);
    });

    {
        let game_id = props.game_id.clone();
        let user_id = props.access_token.access_toket;
        let board_state = board_state.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let board = request::get_game_state(game_id, user_id).await.unwrap();

                    board_state.set(board.board);

                    while !request::get_game_state_changed(game_id, user_id)
                    .await
                    .unwrap() {
                        async_std::task::sleep(Duration::from_secs(2)).await;

                    }
                    let board = request::get_game_state(game_id, user_id).await.unwrap();
                    board_state.set(board.board);
                    
                });
            },
            (),
        );
    }

    html! {
        <game>
            <Board on_click={callback} board={(*board_state).clone()}/>
        </game>
    }
}

#[derive(Properties, PartialEq)]
struct SetupGameProps {
    game_id: Uuid,
    access_token: UserToken,
    setup_callback: Callback<()>,
}

#[function_component(SetupGame)]
fn setup_game(props: &SetupGameProps) -> Html {
    let board_state = use_state(|| common::Board::new());
    let selected_piece_state = use_state(|| Option::<PieceType>::None);

    {
        let selected_piece_state = selected_piece_state.clone();
        use_effect(move || {
            let document = gloo::utils::document();

            let listener = gloo::events::EventListener::new(&document, "keydown", move |event| {
                let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                if event.key_code() == 27 {
                    //Escape
                    selected_piece_state.set(None);
                }
            });

            || drop(listener)
        });
    }

    let bar_callback = {
        let selected_piece_state = selected_piece_state.clone();
        Callback::from(move |piece_type| {
            selected_piece_state.set(Some(piece_type));
        })
    };
    let board_callback = {
        let board_state = board_state.clone();
        let side = props.access_token.side.clone().unwrap();
        let selected_piece_state = selected_piece_state.clone();

        Callback::from(move |e| {
            let (x, y, event): (usize, usize, MouseEvent) = e;
            let mut board = (*board_state).clone();
            event.prevent_default();

            if y >= 6 {
                if event.button() == 0 {
                    if let Some(piece) = &(*selected_piece_state) {
                        let count = (*board_state).count();
                        if count[&piece] < piece.starting_count() {
                            board.set(
                                x,
                                y,
                                Some(common::Piece {
                                    id: Uuid::new_v4(),
                                    owner: side.clone(),
                                    piece_type: piece.clone(),
                                }),
                            );
                        }
                    }
                } else {
                    board.set(x, y, None);
                }
                board_state.set(board);
            }
        })
    };

    let mut count = (*board_state).count();
    let mut finishable = true;

    for piece_type in PieceType::iter() {
        count.insert(
            piece_type.clone(),
            piece_type.starting_count() - count[&piece_type],
        );
        finishable &= *count.get(&piece_type).unwrap_or(&0) == 0;
    }

    let finsh_callback = {
        let board_state = board_state.clone();
        let access_token = props.access_token.access_toket.clone();
        let game_id = props.game_id.clone();
        let setup_callback = props.setup_callback.clone();
        Callback::from(move |_| {
            let mut pieces = SendibleArray::<PieceType, 40>::default();
            for i in 0..40 {
                pieces[i] = (*board_state).0[i + 60].clone().unwrap().piece_type;
            }
            let init_state = InitState {
                access_token: access_token,
                pieces,
            };
            let setup_callback = setup_callback.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match request::init_setup(game_id, init_state).await {
                    Ok(_) => {
                        setup_callback.emit(());
                    }
                    Err(err) => {
                        let err: MoveError = err.downcast().unwrap();
                        log::info!("{}", err);
                    }
                }
            })
        })
    };

    html! {
        <game>
            <Board on_click={board_callback} board={(*board_state).clone()}/>
            <SetupBar side={props.access_token.side.clone().unwrap()} type_select={bar_callback} selected_type={(*selected_piece_state).clone()} type_count={count}/>
            {
                if finishable {
                    html!{
                        <finish>
                            <button onclick={finsh_callback}>{"Finish"}</button>
                        </finish>
                    }
                } else {
                    html!{}
                }
            }
        </game>
    }
}

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    board: common::Board,
    on_click: Callback<(usize, usize, MouseEvent)>,
    selected: Option<(usize, usize)>,
    highlighted: Option<HashMap<(usize, usize), bool>>,
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
        if let Some(piece) = &props.board.0[i] {
            let selected = if let Some((u, v)) = props.selected {
                x == u && y == v
            } else {
                false
            };
            let highlighted = if let Some(highlighted) = &props.highlighted {
                *highlighted.get(&(x, y)).unwrap_or(&false)
            } else {
                false
            };
            pieces.push(html! {
                <Piece side={piece.owner.clone()} piece_type={piece.piece_type.clone()} {x} {y} on_click={callback} {selected} {highlighted} />
            });
        } else {
            let mut class = Classes::new();
            if let Some(selected) = props.selected {
                if x == selected.0 && y == selected.1 {
                    class.push("selected");
                }
            }
            if let Some(highlighted) = &props.highlighted {
                if *highlighted.get(&(x, y)).unwrap_or(&false) {
                    class.push("hightlighted");
                }
            }
            pieces.push(html! {
                <empty class={class} style={format!("grid-column: {}; grid-row: {};", x + 1, y + 1)} onclick={callback.clone()} oncontextmenu={callback} />
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
    on_click: Callback<MouseEvent>,
    #[prop_or_default]
    selected: bool,
    #[prop_or_default]
    highlighted: bool,
}

#[function_component(Piece)]
fn piece(props: &PieceProps) -> Html {
    let mut class = Classes::new();
    if props.selected {
        class.push("selected");
    }
    if props.highlighted {
        class.push("hightlighted");
    }

    html! {
        <piece class={class}>
            <img onclick={props.on_click.clone()} oncontextmenu={props.on_click.clone()} style={format!("grid-column: {}; grid-row: {};", props.x + 1, props.y + 1)} src={format!("/static/assets/temp/{} {}.webp", props.side.to_string(), props.piece_type.to_string().to_lowercase())}/>
        </piece>
    }
}

#[derive(Properties, PartialEq)]
pub struct SetupBarProps {
    side: Side,
    selected_type: Option<PieceType>,
    type_select: Callback<PieceType>,
    type_count: HashMap<PieceType, usize>,
}

#[function_component(SetupBar)]
fn setup_bar(props: &SetupBarProps) -> Html {
    let mut pieces = Vec::new();
    for piece_type in PieceType::iter() {
        if piece_type != PieceType::Unknown {
            let callback = {
                let type_select = props.type_select.clone();
                let piece_type = piece_type.clone();
                Callback::from(move |_| {
                    let piece_type = piece_type.clone();
                    type_select.emit(piece_type);
                })
            };

            let class = if Some(piece_type.clone()) == props.selected_type {
                classes!("selected")
            } else {
                classes!()
            };

            pieces.push(html! {
                <piece_box onclick={callback} class={class}>
                    <piece>
                        <img src={format!("/static/assets/temp/{} {}.webp", props.side.to_string(), piece_type.to_string().to_lowercase())}/>
                    </piece>
                    
                    <text>
                        <name>
                            {piece_type.to_string()}
                        </name>
                        <spacer/>
                        <count>
                            {props.type_count[&piece_type]}
                        </count>
                    </text>
                </piece_box>
            });
        }
    }

    html! {
        <setup_bar>
            {pieces}
        </setup_bar>
    }
}
