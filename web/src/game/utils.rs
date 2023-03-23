use std::collections::HashMap;

use common::{Side, PieceType};
use strum::IntoEnumIterator;
use web_sys::MouseEvent;
use yew::{Properties, Callback, function_component, Html, html, Classes, classes};

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub board: common::Board,
    pub on_click: Callback<(usize, usize, MouseEvent)>,
    pub selected: Option<(usize, usize)>,
    pub highlighted: Option<HashMap<(usize, usize), bool>>,
}

#[function_component(BoardComponent)]
pub fn board(props: &BoardProps) -> Html {
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
    pub side: Side,
    pub piece_type: PieceType,
    pub x: usize,
    pub y: usize,
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub highlighted: bool,
}

#[function_component(Piece)]
pub fn piece(props: &PieceProps) -> Html {
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
    pub side: Side,
    pub selected_type: Option<PieceType>,
    pub type_select: Callback<PieceType>,
    pub type_count: HashMap<PieceType, usize>,
}

#[function_component(SetupBar)]
pub fn setup_bar(props: &SetupBarProps) -> Html {
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
