
use serde::Deserialize;
use strum::{EnumString, Display};

pub struct GameState {
    board: [Option<Piece>; 10*10-2*2*3]
}

pub struct Piece {
    id: u8,
    owner: Side,
    piece_type: PieceType,
}

#[derive(PartialEq, Clone, Debug, EnumString, Display, Deserialize)]
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

#[derive(Clone, PartialEq, Deserialize)]
pub struct GameInfo {
    vs_bot: bool,
    primary_side: Side,
}