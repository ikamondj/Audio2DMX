use axum::{
    extract::State,
    routing::{post, get},
    Json, Router,
};

use axum::response::IntoResponse;


use serde::Deserialize;
use serde_json::Value;


use audio_2_dmx::{audio::audio_loop, dmx::spawn_olad, presets::effect_suite_vec};
use audio_2_dmx::state::AppState;
use audio_2_dmx::state::create_app_state;
use audio_2_dmx::effects::EffectSuite;
use tokio::task::LocalSet;
use std::collections::HashMap;
use audio_2_dmx::presets::{effect_suite_map};

mod audio_selection;


#[derive(Deserialize)]
struct SetRequest {
    key: String,
    value: Value,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Optionally start olad here (await, not spawn)
    spawn_olad().await;

    let local = LocalSet::new();

    local
        .run_until(async {
            let num_bins: usize = 128;

            // however you build these:
            let glob_effects: HashMap<String, EffectSuite> = effect_suite_map(num_bins);
            let ord_effects: Vec<EffectSuite> = effect_suite_vec(num_bins);

            let mode = audio_selection::parse_args();
            let device = audio_selection::choose_audio_device(mode);
            let state = create_app_state(device);

            // Spawn audio processing loop on the *local* task set (no Send required)
            let audio_state = state.clone();
            let glob = glob_effects;
            let ord = ord_effects;
            tokio::task::spawn_local(async move {
                audio_loop(audio_state, glob, ord, num_bins).await;
            });

            // Build your Axum app as before
            let app = Router::new()
                .route("/set", post(set_value))
                .route("/get/{key}", get(get_value))
                .with_state(state);

            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
                .await
                .expect("failed to bind");

            axum::serve(listener, app).await.unwrap();
        })
        .await;
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
