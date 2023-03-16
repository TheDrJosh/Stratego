use crate::Board;
use uuid::Uuid;

pub fn valid_move(id: Uuid, x: usize, y: usize, board: &Board) -> anyhow::Result<bool> {
    if let Some(piece) = board.iter().enumerate().find(|piece| {
        if let Some(piece) = piece.1 {
            piece.id == id
        } else {
            false
        }
    }) {

    }
    Ok(false)
}
