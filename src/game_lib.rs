use crate::my_error::MyError;
use crate::utils::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde_json;
use std::fs;
use std::path::Path;

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
    pub border_breath: f32,
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
        let game_config: GameConfig = serde_json::from_str(&json_str)?;

        info!("Config read successfully from {:?}", path.as_ref());

        Ok(game_config)
    }
}

#[derive(Resource)]
pub struct GameLib {
    pub origin_pos: Vec2,
    pub game_panel_origin: Vec2,
    pub box_mesh: Handle<Mesh>,
    pub box_colors: Vec<Handle<ColorMaterial>>,
}

impl GameLib {
    pub fn new(
        game_config: &GameConfig,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> GameLib {
        let origin_pos = -vec_to_vec2(&game_config.window_size) / 2.0;
        let panel_config = &game_config.game_panel_config;
        let box_config = &game_config.box_config;
        let game_panel_origin =
            origin_pos + vec_to_vec2(&panel_config.pos) + Vec2::splat(panel_config.border_breath);

        let box_mesh = meshes.add(Rectangle::new(box_config.size, box_config.size));

        let box_colors = Self::init_box_colors(&box_config.play_boxes, materials);

        let game_lib = GameLib {
            origin_pos: origin_pos,
            game_panel_origin: game_panel_origin,
            box_mesh: box_mesh,
            box_colors: box_colors,
        };

        info!("GameLib initialized");

        game_lib
    }

    pub fn init_box_colors(
        play_boxes: &[PlayBoxConfig],
        materials: &mut Assets<ColorMaterial>,
    ) -> Vec<Handle<ColorMaterial>> {
        let mut colors: Vec<Handle<ColorMaterial>> = Vec::new();
        for b in play_boxes.iter() {
            let material = materials.add(vec_to_color(&b.color));
            colors.push(material);
        }
        colors
    }
}
