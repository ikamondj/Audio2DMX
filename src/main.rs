use axum::{
    extract::State,
    routing::{post, get},
    Json, Router,
};
use axum::response::IntoResponse;

mod audio;
mod state;
use serde::Deserialize;
use serde_json::Value;


use Audio2DMX::audio::audio_loop;
use Audio2DMX::state::AppState;
use Audio2DMX::state::create_app_state;
use Audio2DMX::effects::EffectSuite;

#[derive(Deserialize)]
struct SetRequest {
    key: String,
    value: Value,
}

#[tokio::main]
async fn main() {
    let state = create_app_state();

    // Spawn audio processing loop
    let audio_state = state.clone();
    tokio::spawn(async move {
        audio_loop(audio_state).await;
    });

    let app = Router::new()
    .route("/set", post(set_value))
    .route("/get/{key}", get(get_value))
    .with_state(state.clone());

    println!("Server running on http://127.0.0.1:3000");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind listener");

    axum::serve(listener, app)
        .await
        .expect("Server failed");

}





async fn set_value(
    State(state): State<AppState>,
    Json(payload): Json<SetRequest>,
) -> impl IntoResponse {
    let mut map = state.store.write().unwrap();
    println!("{} is now {}", payload.key, payload.value);
    map.insert(payload.key, payload.value);
}

async fn get_value(
    State(state): State<AppState>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> impl IntoResponse {
    let map = state.store.read().unwrap();
    let result = map.get(&key).cloned().unwrap_or(Value::String("not found".to_string()));
    Json(result)
}
