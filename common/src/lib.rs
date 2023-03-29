#![feature(inline_const)]
#![feature(const_trait_impl)]

pub mod game_logic;
#[cfg(feature = "client")]
pub mod request;
pub mod utils;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use thiserror::Error;
use utils::SendibleArray;
use uuid::Uuid;

pub const BOARD_SIZE: usize = 10 * 10;

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct Board(pub SendibleArray<Option<Piece>, BOARD_SIZE>);

impl Board {
    pub fn new() -> Self {
        Self(SendibleArray::default())
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&Option<Piece>> {
        self.0 .0.get(x + y * 10)
    }
    pub fn set(&mut self, x: usize, y: usize, piece: Option<Piece>) {
        self.0[x + y * 10] = piece;
    }

    pub fn find(&self, id: Uuid) -> Option<(usize, usize)> {
        let piece = self.0 .0.iter().enumerate().find(|piece| {
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

    pub fn count(&self) -> HashMap<PieceType, usize> {
        let mut counts = HashMap::new();

        for piece_type in PieceType::iter() {
            counts.insert(
                piece_type.clone(),
                self.0
                     .0
                    .iter()
                    .filter(move |&piece| {
                        if let Some(piece) = piece {
                            piece.piece_type == piece_type
                        } else {
                            false
                        }
                    })
                    .count(),
            );
        }

        counts
    }
}

#[derive(PartialEq, Clone, Debug, EnumString, Display, Deserialize, Serialize, Hash, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Side {
    Red,
    Blue,
}

impl std::ops::Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::Red => Side::Blue,
            Side::Blue => Side::Red,
        }
    }
}
impl std::ops::Not for &Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::Red => &Side::Blue,
            Side::Blue => &Side::Red,
        }
    }
}
// impl std::ops::Not for &mut Side {
//     type Output = Self;

//     fn not(self) -> Self::Output {
//         match self {
//             Side::Red => &mut Side::Blue,
//             Side::Blue =>  &mut Side::Red,
//         }
//     }
// }

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Piece {
    pub id: Uuid,
    pub owner: Side,
    pub piece_type: PieceType,
}

#[derive(
    Deserialize,
    Serialize,
    Clone,
    PartialEq,
    Display,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumIter,
    Debug,
    Default,
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
    #[default]
    Unknown = -1,
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
            PieceType::Unknown => 0,
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct GameInfo {
    pub vs_bot: bool,
    pub primary_side: Side,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize)]
pub struct InitState {
    pub access_token: Uuid,
    pub pieces: SendibleArray<PieceType, 40>,
}

#[derive(Deserialize, Serialize, Error, Debug)]
pub enum InitSetupError {
    #[error("Access Denied")]
    InvalidAccess,
    #[error("Incorrect Piece Count")]
    IncorrectPieceCount,
    #[error("Unknown")]
    UnknownFail,
    #[error("Game Does Not Exist")]
    GameDoesNotExist,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BoardState {
    pub board: Board,
    pub active_side: Side,
    pub ready: bool,
}
