use bevy::prelude::*;
use serde::Deserialize;
use serde_json;
use std::fs;
use std::path::Path;
use crate::my_error::MyError;

#[derive(Debug, Deserialize, Resource)]
pub struct GameConfig {
    pub window_size: [f32; 2],
    pub game_panel_config: GamePanelConfig,
    pub box_config: BoxConfig,
}

#[derive(Debug, Deserialize)]
pub struct GamePanelConfig {
    pub size: [u32; 2],
    pub pos: [f32; 2],
    pub background_color: [u8; 4],
    pub border_color: [u8; 4],
    pub border_width: f32,
    pub background_z: f32,
    pub border_z: f32,
}

#[derive(Debug, Deserialize)]
pub struct BoxConfig {
    pub size: f32,
    pub spacing: f32,
    pub z: f32,
    pub play_boxes: Vec<PlayBoxConfig>,
}

#[derive(Debug, Deserialize)]
pub struct PlayBoxConfig {
    pub bitmaps: [[[u8; 4]; 4]; 4],
    pub level: u32,
    pub color: [u8; 4],
}


impl GameConfig {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<GameConfig, MyError> {
        let json_str = fs::read_to_string(path.as_ref())?;
        let mut game_config: GameConfig = serde_json::from_str(&json_str)?;

        info!("Config read successfully from {:?}", path.as_ref());

        Ok(game_config)
    }
}

#[derive(Resource)]
pub struct GameLib {
    pub config: GameConfig,
    pub origin_pos: Vec2,
    pub game_panel_origin: Vec2,
    pub box_mesh: Handle<Mesh>,
    pub box_colors: Vec<Handle<ColorMaterial>>,
}

impl GameLib {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<GameLib, MyError> {
        todo!();
    }
}