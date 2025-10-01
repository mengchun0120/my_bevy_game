use bevy::prelude::*;
use serde::Deserialize;
use serde_json;
use std::fs;
use std::path::Path;
use crate::my_error::MyError;

#[derive(Debug, Deserialize)]
pub struct BoxConfig {
    bitmaps: [[[i32; 4]; 4]; 4],
    level: i32,
    color: [f32; 4],
}

#[derive(Debug, Deserialize, Resource)]
pub struct GameConfig {
    window_size: [f32; 2],
    boxes: Vec<BoxConfig>,
}

impl GameConfig {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<GameConfig, MyError> {
        let json_str = fs::read_to_string(path.as_ref())?;
        let game_config: GameConfig = serde_json::from_str(&json_str)?;

        info!("Config read successfully from {:?}", path.as_ref());

        Ok(game_config)
    }
}
