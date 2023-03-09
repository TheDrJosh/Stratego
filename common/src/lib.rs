
use serde::{Deserialize, Serialize};
use strum::{EnumString, Display};
use uuid::Uuid;

const BOARD_SIZE: usize = 10*10;

pub struct GameState {
    pub board: [Option<Piece>; BOARD_SIZE],
    pub primary_side: Side,
    pub clients: Vec<UserToken>,
}

impl GameState {
    pub fn new(primary_side: Side) -> Self {
        const INIT: Option<Piece> = None;
        Self {
            board: [INIT; BOARD_SIZE],
            primary_side,
            clients: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Deserialize, Serialize)]
pub struct UserToken {
    pub access_toket: Uuid,
    pub side: Option<Side>,
}


#[derive(Deserialize, Serialize)]
pub struct PieceMove {
    access_token: Uuid,
    piece_id: Uuid,
    x: u8,
    y: u8,
}

#[derive(Deserialize, Serialize)]
pub struct InitState {

}