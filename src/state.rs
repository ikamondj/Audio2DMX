
use cpal::Device;
use serde_json::Value;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<RwLock<HashMap<String, Value>>>,
    pub device: Device,
}

pub fn create_app_state(dev : Device) -> AppState {
    let mut mp = HashMap::new();
    let v: Value = Value::from("RB Jams");
    mp.insert("effect".to_string(), v);
    let state = AppState {
        store: Arc::new(RwLock::new(mp)),
        device: dev,
    };

    return state;
}