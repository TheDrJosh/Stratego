use std::rc::Rc;
use std::sync::Mutex;
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
use yew::suspense::Suspension;
use yew::suspense::SuspensionResult;

use crate::game::utils::{BoardComponent, SetupBar};
mod utils;

//Convert to struct Component



#[hook]
fn use_join_game(id: Uuid) -> SuspensionResult<UserToken> {
    let token_state = use_state(|| Option::<UserToken>::None);
    let suspension_state = use_state(|| Suspension::from_future(async move {
        token_state.set(Some(request::join_game(id).await.unwrap()));
    }));

    if suspension_state.resumed() {
        Ok((*token_state).clone().unwrap())
    } else {
        Err((*suspension_state).clone())
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(GameSetup)]
pub fn game_setup(props: &Props) -> Html {
    let fallback = html! {<loading>{"Loading..."}</loading>};

    html! {
        <Suspense {fallback}>
            <GameLoader id={props.id}/>
        </Suspense>
    }
}

#[function_component(GameLoader)]
pub fn game_loader(props: &Props) -> Html {
    html! {}
}

#[derive(Properties, PartialEq)]
pub struct GameProps {
    pub id: Uuid,
}

struct Game {
    setup_complete: bool,
}

impl Component for Game {
    type Message = ();
    type Properties = GameProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            setup_complete: false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            // impl
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
            <BoardComponent on_click={callback} board={(*board_state).clone()} selected={*selected_state}/>
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
                        .unwrap()
                    {
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
            <BoardComponent on_click={callback} board={(*board_state).clone()}/>
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
            <BoardComponent on_click={board_callback} board={(*board_state).clone()}/>
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
