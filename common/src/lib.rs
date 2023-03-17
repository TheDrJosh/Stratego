pub mod game_logic;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

const BOARD_SIZE: usize = 10 * 10;

pub type Board = [Option<Piece>; BOARD_SIZE];

pub struct GameState {
    pub board: Board,
    pub primary_side: Side,
    pub clients: HashMap<Uuid, Option<Side>>, //Vec<UserToken>,
    pub has_primary: bool,
    pub has_secondary: bool,
}

impl GameState {
    pub fn new(primary_side: Side) -> Self {
        const INIT: Option<Piece> = None;
        Self {
            board: [INIT; BOARD_SIZE],
            primary_side,
            clients: Default::default(),
            has_primary: false,
            has_secondary: false,
        }
    }
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
pub struct Piece {
    pub id: Uuid,
    pub owner: Side,
    pub piece_type: PieceType,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Display, Eq, Hash)]
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

#[derive(Deserialize, Serialize, Clone)]
pub struct UserToken {
    pub access_toket: Uuid,
    pub side: Option<Side>,
}

#[derive(Deserialize, Serialize)]
pub struct PieceMove {
    pub access_token: Uuid,
    pub piece_id: Uuid,
    pub x: usize,
    pub y: usize,
}

#[derive(Deserialize, Serialize)]
pub struct InitState {
    pub access_token: Uuid,
    pub pieces: Vec<PieceType>, //[PieceType; 40],
}
#[derive(Deserialize, Serialize)]
pub enum InitSetupReturn {
    InvalidAccess,
    IncorrectPieceCount,
    Success,
    UnknownFail,
    GameDoesNotExist,
}
