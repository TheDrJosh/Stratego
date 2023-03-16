use crate::{Board, Piece, PieceType};
use uuid::Uuid;

pub fn valid_move(board: &Board, id: Uuid, x: usize, y: usize) -> anyhow::Result<MoveResponse> {
    let piece_position = get_piece_position(board, id)?;
    let piece = board[piece_position.0].clone().ok_or(anyhow::anyhow!(""))?;

    if (x == piece_position.1.0 && y == piece_position.1.1) {
        anyhow::bail!("No Move Needed");
    }

    //grid constraints
    if piece.piece_type != PieceType::Scout {
        if !((x == piece_position.1 .0 + 1 && y == piece_position.1 .1)
            || (x == piece_position.1 .0 - 1 && y == piece_position.1 .1)
            || (x == piece_position.1 .0 && y == piece_position.1 .1 + 1)
            || (x == piece_position.1 .0 && y == piece_position.1 .1 - 1))
        {
            anyhow::bail!("Outside of Move Range");
        }
    } else {
        if !(piece_position.1.0 == x || piece_position.1.1 == y) {
            anyhow::bail!("Outside of Move Range");
        }
        //scout constraints
        if piece_position.1.0 == x {
            if piece_position.1.1 > y {
                let mut max_move = 0;
                for i in piece_position.1.1..0 {
                    if board[x + i * 10].is_some() {
                        max_move = i;
                        break;
                    }
                }
                if y < max_move {
                    anyhow::bail!("Outside of Move Range");
                }

            }
        }
    }

    todo!()
}

pub enum MoveResponse {
    Success,
    AttackSuccess(Piece),
    AttackFailure(Piece),
}

pub fn move_piece(board: &mut Board, id: Uuid, x: usize, y: usize) -> anyhow::Result<MoveResponse> {
    let res = valid_move(board, id, x, y)?;

    let position = get_piece_position(board, id)?;
    board[x + y * 10] = board[position.0].clone();
    board[position.0] = None;

    Ok(res)
}

pub fn get_piece_position(board: &Board, id: Uuid) -> anyhow::Result<(usize, (usize, usize))> {
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
        .unwrap();
    let x = piece.0 % 10;
    let y = piece.0 / 10;
    Ok((piece.0, (x, y)))
}
