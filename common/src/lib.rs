pub mod game_logic;

#[cfg(feature = "client")]
pub mod request;

use std::{collections::HashMap, fmt::Write};

use serde::{
    de::{self, Expected, SeqAccess, Unexpected, Visitor},
    ser::SerializeSeq,
    Deserialize, Serialize,
};
use strum::{Display, EnumIter, EnumString};
use uuid::Uuid;

const BOARD_SIZE: usize = 10 * 10;

#[derive(Clone, PartialEq)]
pub struct Board([Option<Piece>; BOARD_SIZE]);

impl Board {
    pub fn new() -> Self {
        const INIT: Option<Piece> = None;
        Self([INIT; BOARD_SIZE])
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&Option<Piece>> {
        self.0.get(x + y * 10)
    }
    pub fn set(&mut self, x: usize, y: usize, piece: Option<Piece>) {
        self.0[x + y * 10] = piece;
    }

    pub fn find(&self, id: Uuid) -> Option<(usize, usize)> {
        let piece = self.0.iter().enumerate().find(|piece| {
            if let Some(piece) = piece.1 {
                piece.id == id
            } else {
                false
            }
        })?;
        let x = piece.0 % 10;
        let y = piece.0 / 10;
        Some((x, y))
    }
}

impl Default for Board {
    fn default() -> Self {
        const INIT: Option<Piece> = None;
        Self([INIT; BOARD_SIZE])
    }
}

impl Serialize for Board {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(BOARD_SIZE))?;
        for piece in &self.0 {
            seq.serialize_element(&piece)?;
        }
        seq.end()
    }
}

struct BoardVisitor;

impl<'de> Visitor<'de> for BoardVisitor {
    type Value = Board;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a board struct")
    }
    
    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        // if let Some(size) = seq.size_hint() {
        //     if size != BOARD_SIZE {
        //         return Err(de::Error::invalid_length(size, &Unexpected::Other("Length")))
        //     }
        // }

        let mut board = Board::new();

        let mut i = 0;
        while let Some(piece) = seq.next_element()? {
            board.0[i] = piece;
            i += 1;
        }

        Ok(board)
    }
}

impl<'de> Deserialize<'de> for Board {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(BoardVisitor)
    }
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
            board: Board::new(),
            primary_side: primary_side.clone(),
            clients: Default::default(),
            has_primary: false,
            has_secondary: false,
            active_side: primary_side,
        }
    }
}

#[derive(PartialEq, Clone, Debug, EnumString, Display, Deserialize, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum Side {
    Red,
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

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Piece {
    pub id: Uuid,
    pub owner: Side,
    pub piece_type: PieceType,
}

#[derive(
    Deserialize, Serialize, Clone, PartialEq, Display, Eq, Hash, PartialOrd, Ord, EnumIter, Debug,
)]
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
    pub fn starting_count(&self) -> usize {
        match self {
            PieceType::Bomb => 6,
            PieceType::Marshal => 1,
            PieceType::General => 1,
            PieceType::Colonel => 2,
            PieceType::Major => 3,
            PieceType::Captain => 4,
            PieceType::Lieutenant => 4,
            PieceType::Sergeant => 4,
            PieceType::Miner => 5,
            PieceType::Scout => 8,
            PieceType::Spy => 1,
            PieceType::Flag => 1,
        }
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
pub struct BoardState {
    pub board: Board,
    pub active_side: Side,
}
