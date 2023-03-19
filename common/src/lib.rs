pub mod game_logic;

#[cfg(feature = "client")]
pub mod request;

use std::{collections::HashMap};

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

const BOARD_SIZE: usize = 10 * 10;

pub type Board = [Option<Piece>; BOARD_SIZE];

pub fn empty_board() -> Board {
    const INIT: Option<Piece> = None;
    [INIT; BOARD_SIZE]
}

pub struct GameState {
    pub board: Board,
    pub primary_side: Side,
    pub clients: HashMap<Uuid, Option<Side>>, //Vec<UserToken>,
    pub has_primary: bool,
    pub has_secondary: bool,
    pub active_side: Side,
}

impl GameState {
    pub fn new(primary_side: Side) -> Self {
        Self {
            board: empty_board(),
            primary_side: primary_side.clone(),
            clients: Default::default(),
            has_primary: false,
            has_secondary: false,
            active_side: primary_side,
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

#[derive(Deserialize, Serialize, Clone, PartialEq, Display, Eq, Hash, PartialOrd, Ord)]
pub enum PieceType {
    Bomb = 11,
    Marshal = 10,
    General = 9,
    Colonel = 8,
    Major = 7,
    Captain = 6,
    Lieutenant = 5,
    Sergeant = 4,
    Miner = 3,
    Scout = 2,
    Spy = 1,
    Flag = 0,
}

impl PieceType {
    pub fn triumphs(&self, other: &Self) -> bool {
        match self {
            PieceType::Miner => {
                if other == &PieceType::Bomb {
                    return true;
                }
            }
            PieceType::Spy => {
                if other == &PieceType::Marshal {
                    return true;
                }
            }
            _ => {}
        }

        self > other
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct GameInfo {
    pub vs_bot: bool,
    pub primary_side: Side,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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

#[derive(Deserialize, Serialize)]
pub struct BoardStateSendible {
    pub board: Vec<Option<Piece>>,
    pub active_side: Side,
}

pub struct BoardState {
    pub board: Board,
    pub active_side: Side,
}