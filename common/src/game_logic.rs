use crate::{Board, Piece, PieceType};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub fn valid_move(board: &Board, id: Uuid, x: usize, y: usize) -> MoveResult {
    let piece_position = get_piece_position(board, id)?;
    let piece = board[piece_position.0].clone().unwrap();

    //same position
    if x == piece_position.1 .0 && y == piece_position.1 .1 {
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
        return Err(MoveError::InvalidLocation)
    }

    //grid constraints
    if piece.piece_type != PieceType::Scout {
        if !((x == piece_position.1 .0 + 1 && y == piece_position.1 .1)
            || (x == piece_position.1 .0 - 1 && y == piece_position.1 .1)
            || (x == piece_position.1 .0 && y == piece_position.1 .1 + 1)
            || (x == piece_position.1 .0 && y == piece_position.1 .1 - 1))
        {
            Err(MoveError::OutsideOfMoveRange(x, y))?;
        }
    } else {
        if !(piece_position.1 .0 == x || piece_position.1 .1 == y) {
            Err(MoveError::OutsideOfMoveRange(x, y))?;
        }
        //scout constraints
        if piece_position.1 .0 == x {
            if piece_position.1 .1 > y {
                let mut max_move = 0;
                for i in piece_position.1 .1 - 1..0 {
                    if board[x + i * 10].is_some() {
                        max_move = i;
                        break;
                    }
                }
                if y < max_move {
                    Err(MoveError::OutsideOfMoveRange(x, y))?;
                }
            } else {
                let mut max_move = 9;
                for i in piece_position.1 .1 + 1..10 {
                    if board[x + i * 10].is_some() {
                        max_move = i;
                        break;
                    }
                }
                if y > max_move {
                    Err(MoveError::OutsideOfMoveRange(x, y))?;
                }
            }
        } else {
            if piece_position.1 .0 > x {
                let mut max_move = 0;
                for i in piece_position.1 .0 - 1..0 {
                    if board[i + y * 10].is_some() {
                        max_move = i;
                        break;
                    }
                }
                if x < max_move {
                    Err(MoveError::OutsideOfMoveRange(x, y))?;
                }
            } else {
                let mut max_move = 9;
                for i in piece_position.1 .0 + 1..10 {
                    if board[i + y * 10].is_some() {
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

    if let Some(other_piece) = &board[x + y * 10] {
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
    }




    Ok(MoveResponse::Success)
}

pub type MoveResult = Result<MoveResponse, MoveError>;

#[derive(Deserialize, Serialize)]
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
}

pub fn move_piece(board: &mut Board, id: Uuid, x: usize, y: usize) -> MoveResult {
    let res = valid_move(board, id, x, y)?;

    let position = get_piece_position(board, id)?;
    board[x + y * 10] = board[position.0].clone();
    board[position.0] = None;

    Ok(res)
}

pub fn get_piece_position(board: &Board, id: Uuid) -> Result<(usize, (usize, usize)), MoveError> {
    let piece = board
        .iter()
        .enumerate()
        .find(|piece| {
            if let Some(piece) = piece.1 {
                piece.id == id
            } else {
                false
            }
        })
        .ok_or(MoveError::InvalidLocation)?;
    let x = piece.0 % 10;
    let y = piece.0 / 10;
    Ok((piece.0, (x, y)))
}
