mod board;
mod gamestate;
mod player;
mod server;
mod client;

use crate::board::TileStates;
use crate::gamestate::GameState;
use crate::server::Server;
use crate::client::Client;

use std::sync::Mutex;
use tokio::sync::Mutex as AsyncMutex;
use tauri::State;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn quit_app() {
    std::process::exit(0);
}

#[tauri::command]
fn update_board(row: u8, col: u8, gamestate: State<Mutex<GameState>>) -> Vec<Vec<TileStates>> {
    let mut gamestate = gamestate.lock().unwrap();
    gamestate.update_board(row, col);
    gamestate.get_current_player_board()
}

#[tauri::command]
fn bot_attack(gamestate: State<Mutex<GameState>>) -> Vec<Vec<TileStates>> {
    let mut gamestate = gamestate.lock().unwrap();
    gamestate.bot_attack();
    gamestate.get_current_player_board()
}

#[tauri::command]
fn reset_board(gamestate: State<Mutex<GameState>>) {
    let mut gamestate = gamestate.lock().unwrap();
    gamestate.reset_game();
}

#[tauri::command]
fn get_board(gamestate: State<Mutex<GameState>>) -> Vec<Vec<TileStates>> {
    let gamestate = gamestate.lock().unwrap();
    gamestate.get_current_player_board()
}

#[tauri::command]
fn switch_turn(gamestate: State<Mutex<GameState>>) {
    let mut gamestate = gamestate.lock().unwrap();
    gamestate.switch_turn();
}

#[tauri::command]
async fn start_server() -> Result<String, String> {
    tokio::spawn(async {
        Server::new().await;
    });

    Ok("Server started".to_string())
}

#[tauri::command]
async fn connect_to_game(ip: String, client: State<'_, AsyncMutex<Option<Client>>>) -> Result<String, String> {
    match Client::connect(&ip).await {
        Ok(new_client) => {
            let mut client_guard = client.lock().await;
            *client_guard = Some(new_client);
            Ok("Connected to server".to_string())
        }
        Err(e) => Err(format!("Failed to connect: {}", e)),
    }
}

#[tauri::command]
async fn send_data(client: State<'_, AsyncMutex<Option<Client>>>, data: Vec<u8>) -> Result<(), String> {
    let mut client_guard = client.lock().await;
    if let Some(client) = client_guard.as_mut() {
        match client.send(&data).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to send data: {}", e)),
        }
    } else {
        Err("No client connection available".to_string())
    }
}

#[tauri::command]
async fn receive_data(client: State<'_, AsyncMutex<Option<Client>>>) -> Result<Vec<u8>, String> {
    let mut client_guard = client.lock().await;
    if let Some(client) = client_guard.as_mut() {
        let mut buf = vec![0; 1024];
        match client.receive(&mut buf).await {
            Ok(size) if size > 0 => Ok(buf[..size].to_vec()),
            Ok(_) => Err("No data received".to_string()),
            Err(e) => Err(format!("Failed to receive data: {}", e)),
        }
    } else {
        Err("No client connection available".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(GameState::new()))
        .manage(AsyncMutex::new(Option::<Client>::None)) // Use tokio's async mutex
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            quit_app,
            update_board,
            bot_attack,
            reset_board,
            get_board,
            switch_turn,
            start_server,
            connect_to_game,
            send_data,
            receive_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}