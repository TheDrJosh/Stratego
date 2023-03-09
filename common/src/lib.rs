
use serde::{Deserialize, Serialize};
use strum::{EnumString, Display};

const BOARD_SIZE: usize = 10*10;

pub struct GameState {
    board: [Option<Piece>; BOARD_SIZE],
    primary_side: Side,
}

impl GameState {
    pub fn new(primary_side: Side) -> Self {
        const INIT: Option<Piece> = None;
        Self {
            board: [INIT; BOARD_SIZE],
            primary_side,
        }
    }
}

pub struct Piece {
    id: u8,
    owner: Side,
    piece_type: PieceType,
}

#[derive(PartialEq, Clone, Debug, EnumString, Display, Deserialize, Serialize)]
pub enum Side {
    #[strum(serialize = "red")]
    Red,
    #[strum(serialize = "blue")]
    Blue,
}

impl Side {
    pub fn not(&self) -> Self {
        match self {
            Side::Red => Side::Blue,
            Side::Blue => Side::Red,
        }
    }
}


pub enum PieceType {
    Bomb,
    Marshal,
    General,
    Colonel,
    Major,
    Captain,
    Lieutenant,
    Sergeant,
    Miner,
    Scout,
    Spy,
    Flag,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct GameInfo {
    pub vs_bot: bool,
    pub primary_side: Side,
}