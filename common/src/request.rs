use gloo_net::http::Request;
use uuid::Uuid;

use crate::{
    game_logic::MoveResult, BoardState, GameInfo, InitSetupError, InitState, PieceMove, Side,
    UserToken,
};

pub async fn create_game(game_info: GameInfo) -> anyhow::Result<Uuid> {
    let fetched = Request::post("http://127.0.0.1:8000/api/create_game")
        .json(&game_info)?
        .send()
        .await?
        .json()
        .await?;

    Ok(fetched)
}

pub async fn game_exists(id: Uuid) -> anyhow::Result<bool> {
    let fetched =
        Request::get(format!("http://127.0.0.1:8000/api/{}/game_exists", id.to_string()).as_str())
            .send()
            .await?
            .json()
            .await?;

    Ok(fetched)
}

pub async fn join_game(id: Uuid) -> anyhow::Result<UserToken> {
    let fetched =
        Request::get(format!("http://127.0.0.1:8000/api/{}/join", id.to_string()).as_str())
            .send()
            .await?;
    let fetched = if fetched.ok() {
        fetched.json().await?
    } else {
        anyhow::bail!(fetched.text().await?);
    };

    Ok(fetched)
}

pub async fn join_random_game(side: Side) -> anyhow::Result<Uuid> {
    let fetched = Request::get(
        format!("http://127.0.0.1:8000/api/join_random/{}", side.to_string()).as_str(),
    )
    .send()
    .await?
    .json()
    .await?;

    Ok(fetched)
}

pub async fn get_game_state(id: Uuid) -> anyhow::Result<BoardState> {
    let fetched =
        Request::get(format!("http://127.0.0.1:8000/api/{}/game_state", id.to_string()).as_str())
            .send()
            .await?;
    let fetched: BoardState = if fetched.ok() {
        fetched.json().await?
    } else {
        anyhow::bail!(fetched.text().await?);
    };

    Ok(fetched)
}

pub async fn get_game_state_changed(id: Uuid) -> anyhow::Result<BoardState> {
    let fetched =
        Request::get(format!("http://127.0.0.1:8000/api/{}/game_state", id.to_string()).as_str())
            .send()
            .await?;
    let fetched: BoardState = if fetched.ok() {
        fetched.json().await?
    } else {
        anyhow::bail!(fetched.text().await?);
    };

    Ok(fetched)
}

pub async fn move_piece(id: Uuid, piece_move: PieceMove) -> anyhow::Result<MoveResult> {
    let fetched =
        Request::put(format!("http://127.0.0.1:8000/api/{}/move_piece", id.to_string()).as_str())
            .json(&piece_move)?
            .send()
            .await?
            .json()
            .await?;

    Ok(fetched)
}

pub async fn init_setup(id: Uuid, init_state: InitState) -> anyhow::Result<()> {
    let fetched =
        Request::post(format!("http://127.0.0.1:8000/api/{}/init_setup", id.to_string()).as_str())
            .json(&init_state)?
            .send()
            .await?;
    if fetched.ok() {
        Ok(())
    } else {
        Err(fetched.json::<InitSetupError>().await?.into())
    }
}
