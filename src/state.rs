
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
    let state = AppState {
        store: Arc::new(RwLock::new(HashMap::new())),
        device: dev,
    };
    return state;
}