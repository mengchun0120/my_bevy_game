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

const PLAYER_BOX_SIZE: usize = 4;
const PLAYER_BOX_ROTATE_COUNT: usize = 4;

#[derive(Debug, Deserialize)]
pub struct PlayBoxConfig {
    pub bitmaps: [[[u8; PLAYER_BOX_SIZE]; PLAYER_BOX_SIZE]; PLAYER_BOX_ROTATE_COUNT],
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

#[derive(Resource, Debug)]
pub struct GameLib {
    pub origin_pos: Vec2,
    pub game_panel_origin: Vec2,
    pub box_span: f32,
    pub box_mesh: Handle<Mesh>,
    pub box_colors: Vec<Handle<ColorMaterial>>,
    pub box_locations: [[Vec2; PLAYER_BOX_SIZE]; PLAYER_BOX_SIZE],
}

impl GameLib {
    pub fn new(
        game_config: &GameConfig,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> GameLib {
        let panel_config = &game_config.game_panel_config;
        let box_config = &game_config.box_config;

        let origin_pos = -vec_to_vec2(&game_config.window_size) / 2.0;

        let game_panel_origin =
            origin_pos + vec_to_vec2(&panel_config.pos) + Vec2::splat(panel_config.border_breath);
        
        let box_span = box_config.size + box_config.spacing;

        let box_mesh = meshes.add(Rectangle::new(box_config.size, box_config.size));

        let box_colors = Self::init_box_colors(&box_config.play_boxes, materials);

        let box_locations = Self::init_box_locations(box_span);
        
        let game_lib = GameLib {
            origin_pos: origin_pos,
            game_panel_origin: game_panel_origin,
            box_span: box_span,
            box_mesh: box_mesh,
            box_colors: box_colors,
            box_locations: box_locations,
        };

        info!("GameLib initialized");

        game_lib
    }

    fn init_box_colors(
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

    fn init_box_locations(box_span: f32) -> [[Vec2; PLAYER_BOX_SIZE]; PLAYER_BOX_SIZE] {
        let mut box_locations = [[Vec2{x: 0.0, y: 0.0}; PLAYER_BOX_SIZE]; PLAYER_BOX_SIZE];
        let initial_x = box_span / 2.0;
        let mut y = box_span / 2.0;

        for row in (0..PLAYER_BOX_SIZE).rev() {
            let mut x = initial_x;
            for col in 0..PLAYER_BOX_SIZE {
                box_locations[row][col].x = x;
                box_locations[row][col].y = y;
                x += box_span;
            }
            y += box_span;
        }

        box_locations
    }
}
