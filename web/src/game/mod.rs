use std::collections::HashMap;
use std::time::Duration;

use crate::game::game_logic::MoveResult;
use common::game_logic::{self, MoveError};
use common::utils::SendibleArray;
use common::{request, Board, BoardState, PieceMove, PieceType, Side, BOARD_SIZE};
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
fn use_join_game(id: Uuid) -> SuspensionResult<Result<UserToken, String>> {
    let token_state = use_state(|| Option::<Result<UserToken, String>>::None);
    let suspension_state = {
        let token_state = token_state.clone();
        use_state(|| {
            Suspension::from_future(async move {
                token_state.set(Some(
                    request::join_game(id).await.map_err(|e| e.to_string()),
                ));
            })
        })
    };

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
pub fn game_loader(props: &Props) -> HtmlResult {
    let user_token = match use_join_game(props.id)? {
        Ok(user_token) => user_token,
        Err(e) => {
            return Ok(html! {
                <error>{e}</error>
            })
        }
    };

    let setup_state = use_state(|| false);
    if let Some(side) = &user_token.side {
        let callback = {
            let setup_state = setup_state.clone();
            Callback::from(move |_| setup_state.set(true))
        };

        Ok(if *setup_state {
            html! {
                <Game id={props.id} access_toket={user_token.access_toket} side={side.clone()} />
            }
        } else {
            html! {
                <SetupGame game_id={props.id} access_token={user_token} setup_callback={callback}/>
            }
        })
    } else {
        Ok(html! {
            <GameViewer game_id={props.id} access_token={user_token} />
        })
    }
}
fn get_changed_board(ctx: &Context<Game>) {
    let game_id = ctx.props().id.clone();
    let user_id = ctx.props().access_toket.clone();

    ctx.link().send_future(async move {
        async_std::task::sleep(Duration::from_secs(2)).await;
        GameMsg::UpdateBoard(request::get_game_state(game_id, user_id).await.unwrap())
    });
    /*
    ctx.link().send_future(async move {
        loop {
            let changed = request::get_game_state_changed(game_id, user_id)
                .await
                .unwrap();
            if changed {
                let board = request::get_game_state(game_id, user_id).await.unwrap();
                return GameMsg::UpdateBoard(board);
            }
            async_std::task::sleep(Duration::from_secs(2)).await;
        }
    });
    */
}

fn get_board(ctx: &Context<Game>) {
    let game_id = ctx.props().id.clone();
    let user_id = ctx.props().access_toket.clone();
    ctx.link().send_future(async move {
        GameMsg::UpdateBoard(request::get_game_state(game_id, user_id).await.unwrap())
    });
}
#[derive(Properties, PartialEq)]
pub struct GameProps {
    pub id: Uuid,
    pub access_toket: Uuid,
    pub side: Side,
}

enum GameMsg {
    UpdateBoard(BoardState),
    ClearSelect,
    Select(usize, usize),
    PieceMoved(MoveResult),
}

struct Game {
    board: Board,
    active_side: Side,
    selected: Option<(usize, usize)>,
    highlighted: HashMap<(usize, usize), bool>,
}


impl Game {
    fn move_piece(&mut self, ctx: &Context<Game>, x: usize, y: usize) {
        if let Some(selected) = self.selected {
            log::info!("Move Result: {:?}", game_logic::valid_move(&self.board, selected.0, selected.1, x, y));
            if game_logic::valid_move(&self.board, selected.0, selected.1, x, y)
                .is_ok()
            {
                {
                    let game_id = ctx.props().id;
                    let access_toket = ctx.props().access_toket;
                    let id = self
                        .board
                        .get(selected.0, selected.1)
                        .unwrap()
                        .clone()
                        .unwrap()
                        .id;
                    ctx.link().send_future(async move {
                        GameMsg::PieceMoved(
                            request::move_piece(
                                game_id,
                                PieceMove {
                                    access_token: access_toket,
                                    piece_id: id,
                                    x,
                                    y,
                                },
                            )
                            .await
                            .unwrap(),
                        )
                    });
                }

                self.selected = None;
                self.highlighted.clear();
            }
        }
    }
}

impl Component for Game {
    type Message = Option<GameMsg>;
    type Properties = GameProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            board: Board::new(),
            active_side: Side::Red,
            selected: None,
            highlighted: HashMap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let Some(msg) = msg {
            match msg {
                GameMsg::UpdateBoard(board) => {
                    if !board.ready || board.active_side != ctx.props().side {
                        get_changed_board(ctx);
                    }
                    self.board = board.board;
                    self.active_side = board.active_side;
                    
                }
                GameMsg::ClearSelect => {
                    self.selected = None;
                    self.highlighted.clear();
                }
                GameMsg::Select(x, y) => {
                    log::info!("Select: ({}, {})", x, y);
                    if self.active_side == ctx.props().side {
                        if let Some(Some(piece)) = self.board.get(x, y) {
                            if piece.owner == ctx.props().side {
                                self.selected = Some((x, y));
                                self.highlighted.clear();
                                for i in 0..BOARD_SIZE {
                                    let u = i % 10;
                                    let v = i / 10;
                                    let mov = game_logic::valid_move(&self.board, x, y, u, v);
                                    self.highlighted.insert((u, v), mov.is_ok());
                                }
                            } else {
                                self.move_piece(ctx, x, y);
                            }
                        } else {
                            self.move_piece(ctx, x, y);
                        }
                    }
                }
                GameMsg::PieceMoved(_res) => {
                    get_board(ctx);
                    get_changed_board(ctx);
                }
            }

            true
        } else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.link().callback(|(x, y, _e)| GameMsg::Select(x, y));

        let onkeydown = ctx.link().callback(|event: KeyboardEvent| {
            if event.key_code() == 27 {
                Some(GameMsg::ClearSelect)
            } else {
                None
            }
        });

        html! {
            <game {onkeydown}>
            {
                if self.active_side.clone() != ctx.props().side {
                    html! {
                        <waiting>{format!("Waiting for {}", self.active_side)}</waiting>
                    }
                } else {
                    html! { }
                }
            }
                <BoardComponent on_click={callback} board={self.board.clone()} selected={self.selected} highlighted={self.highlighted.clone()} />
            </game>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        
        if first_render {
            get_board(ctx);
        }

    }

}

#[derive(Properties, PartialEq)]
struct GameViewerProps {
    game_id: Uuid,
    access_token: UserToken,
}

#[function_component(GameViewer)]
fn game_viewer(props: &GameViewerProps) -> Html {
    // needs upgrade to struct component
    let board_state = use_state(|| common::Board::new());

    let callback = Callback::from(move |e| {
        log::info!("game viewer: {:?}", e);
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
