// src/models/config.rs
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Config {
    pub app_name: String,
    pub version: String,
    pub icon_path: String,
    pub is_vsync_enabled: bool,
}
