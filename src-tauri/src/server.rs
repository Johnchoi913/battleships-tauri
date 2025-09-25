use crate::board::TileStates;
use crate::gamestate::GameState;

use axum::{
    extract::State,
    http::StatusCode,
    response::Json as ResponseJson,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Server {
    gamestate: Arc<RwLock<GameState>>,
}

#[derive(Deserialize)]
struct AttackRequest {
    row: u8,
    col: u8,
}

#[derive(Serialize)]
struct GameResponse {
    board: Vec<Vec<TileStates>>,
    game_over: bool,
    winner: Option<String>,
}

impl Server {
    pub async fn new() {
        let server = Arc::new(Server {
            gamestate: Arc::new(RwLock::new(GameState::new())),
        });

        let app = Router::new()
            .route("/attack", post(attack))
            .route("/board", get(get_board))
            .with_state(server);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        axum::serve(listener, app).await.unwrap();
    }
}

async fn attack(
    State(server): State<Arc<Server>>,
    Json(attack_request): Json<AttackRequest>,
) -> Result<ResponseJson<GameResponse>, StatusCode> {

    let mut gamestate = server.gamestate.write().await;
    gamestate.update_board(attack_request.row, attack_request.col);

    let board = gamestate.get_current_player_board();

    let game_over = false;
    let winner = None;

    Ok(ResponseJson(GameResponse {
        board,
        game_over,
        winner,
    }))
}

async fn get_board(State(server): State<Arc<Server>>) -> ResponseJson<Vec<Vec<TileStates>>> {
    let gamestate = server.gamestate.read().await;
    ResponseJson(gamestate.get_current_player_board())
}
