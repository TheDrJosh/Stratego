use crate::{Board, Piece, PieceType};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub fn valid_move_from_id(board: &Board, id: Uuid, x: usize, y: usize) -> MoveResult {
    let piece_position = board.find(id).ok_or(MoveError::PieceDoesNotExist(id))?;
    valid_move(board, piece_position.0, piece_position.1, x, y)
}

pub fn valid_move(board: &Board, u: usize, v: usize, x: usize, y: usize) -> MoveResult {
    let piece_position = (u, v);
    let piece = board
        .get(piece_position.0, piece_position.1)
        .ok_or(MoveError::OutsideOfMoveRange(u, v))?
        .clone()
        .ok_or(MoveError::PieceNotFound(u, v))?;

    //immovables
    match &piece.piece_type {
        PieceType::Bomb => Err(MoveError::Immovable)?,
        PieceType::Flag => Err(MoveError::Immovable)?,
        _ => {}
    }

    //same position
    if x == piece_position.0 && y == piece_position.1 {
        Err(MoveError::NoMoveNeeded)?;
    }

    //water
    if match (x, y) {
        //left
        (2, 4) => true,
        (3, 4) => true,
        (2, 5) => true,
        (3, 5) => true,
        //right
        (6, 4) => true,
        (7, 4) => true,
        (6, 5) => true,
        (7, 5) => true,
        _ => false,
    } {
        return Err(MoveError::InvalidLocation);
    }

    //grid constraints
    if piece.piece_type != PieceType::Scout {
        if !((piece_position.0 < 9 && (x == piece_position.0 + 1 && y == piece_position.1))
            || (piece_position.0 > 0 && (x == piece_position.0 - 1 && y == piece_position.1))
            || (piece_position.1 < 9 && (x == piece_position.0 && y == piece_position.1 + 1))
            || (piece_position.1 > 0 && (x == piece_position.0 && y == piece_position.1 - 1)))
        {
            Err(MoveError::OutsideOfMoveRange(x, y))?;
        }
    } else {
        if !(piece_position.0 == x || piece_position.1 == y) {
            Err(MoveError::OutsideOfMoveRange(x, y))?;
        }
        //scout constraints
        if piece_position.0 == x {
            if piece_position.1 > y {
                let mut max_move = 0;
                for i in piece_position.1 - 1..0 {
                    if board.get(x, i).unwrap().is_some() {
                        max_move = i;
                        break;
                    }
                }
                if y < max_move {
                    Err(MoveError::OutsideOfMoveRange(x, y))?;
                }
            } else {
                let mut max_move = 9;
                for i in piece_position.1 + 1..10 {
                    if board.get(x, i).unwrap().is_some() {
                        max_move = i;
                        break;
                    }
                }
                if y > max_move {
                    Err(MoveError::OutsideOfMoveRange(x, y))?;
                }
            }
        } else {
            if piece_position.0 > x {
                let mut max_move = 0;
                for i in piece_position.0 - 1..0 {
                    if board.get(i, y).unwrap().is_some() {
                        max_move = i;
                        break;
                    }
                }
                if x < max_move {
                    Err(MoveError::OutsideOfMoveRange(x, y))?;
                }
            } else {
                let mut max_move = 9;
                for i in piece_position.0 + 1..10 {
                    if board.get(i, y).unwrap().is_some() {
                        max_move = i;
                        break;
                    }
                }
                if y > max_move {
                    Err(MoveError::OutsideOfMoveRange(x, y))?;
                }
            }
        }
    }

    // attack testing

    if let Some(other_piece) = board.get(x, y).unwrap() {
        if piece.owner != other_piece.owner {
            if piece.piece_type == other_piece.piece_type {
                return Ok(MoveResponse::AttackFailureMutual(
                    other_piece.clone(),
                    piece,
                ));
            }

            if piece.piece_type.triumphs(&other_piece.piece_type) {
                return Ok(MoveResponse::AttackSuccess(other_piece.clone()));
            }
            return Ok(MoveResponse::AttackFailure(piece));
        } else {
            return Err(MoveError::FriendlyFire);
        }
    }

    Ok(MoveResponse::Success)
}

pub type MoveResult = Result<MoveResponse, MoveError>;

#[derive(Deserialize, Serialize, Debug)]
pub enum MoveResponse {
    Success,
    AttackSuccess(Piece),
    AttackFailure(Piece),
    AttackFailureMutual(Piece, Piece),
}

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum MoveError {
    #[error("Invalid Location")]
    InvalidLocation,
    #[error("No Move Needed")]
    NoMoveNeeded,
    #[error("Outside of Move Range: ({0}, {1})")]
    OutsideOfMoveRange(usize, usize),
    #[error("Piece doesn't Exist: {0}")]
    PieceDoesNotExist(Uuid),
    #[error("Friendly Fire")]
    FriendlyFire,
    #[error("Immovable")]
    Immovable,
    #[error("Piece Not Found At ({0}, {1})")]
    PieceNotFound(usize, usize),
}

pub fn move_piece(board: &mut Board, id: Uuid, x: usize, y: usize) -> MoveResult {
    let res = valid_move_from_id(board, id, x, y)?;

    let position = board.find(id).ok_or(MoveError::PieceDoesNotExist(id))?;
    match res {
        MoveResponse::Success => {
            board.set(x, y, board.get(position.0, position.1).unwrap().clone());
            board.set(position.0, position.1, None);
        },
        MoveResponse::AttackSuccess(_) => {
            board.set(x, y, board.get(position.0, position.1).unwrap().clone());
            board.set(position.0, position.1, None);
        },
        MoveResponse::AttackFailure(_) => {
            board.set(position.0, position.1, None);
        },
        MoveResponse::AttackFailureMutual(_, _) => {
            board.set(x, y, board.get(position.0, position.1).unwrap().clone());
            board.set(position.0, position.1, None);
        },
    }
    

    Ok(res)
}
