use crate::utils::*;
use bevy::prelude::*;
use rand::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Resource)]
pub struct GameConfig {
    pub window_size: RectSize,
    pub game_panel_config: GamePanelConfig,
    pub box_config: BoxConfig,
}

#[derive(Debug, Deserialize)]
pub struct GamePanelConfig {
    size: [usize; 2],
    pos: [f32; 2],
    background_color: [u8; 4],
    border_color: [u8; 4],
    pub border_breath: f32,
    pub background_z: f32,
    pub border_z: f32,
}

impl GamePanelConfig {
    pub fn row_count(&self) -> usize {
        self.size[0]
    }

    pub fn col_count(&self) -> usize {
        self.size[1]
    }

    pub fn pos(&self) -> Vec2 {
        vec_to_vec2(&self.pos)
    }

    pub fn background_color(&self) -> Color {
        vec_to_color(&self.background_color)
    }

    pub fn border_color(&self) -> Color {
        vec_to_color(&self.border_color)
    }
}

#[derive(Debug, Deserialize)]
pub struct BoxConfig {
    pub size: f32,
    pub spacing: f32,
    pub z: f32,
    pub play_boxes: Vec<PlayBoxConfig>,
}

impl BoxConfig {
    pub fn play_box_type_count(&self) -> usize {
        self.play_boxes.len()
    }

    pub fn play_box_bitmap(&self, type_index: usize, rotate_index: usize) -> &BitMap {
        self.play_boxes[type_index].bitmap(rotate_index)
    }

    pub fn rand_type_index<R: Rng>(&self, rng: &mut R) -> usize {
        rng.random_range(0..self.play_box_type_count())
    }

    pub fn rand_rotate_index<R: Rng>(rng: &mut R) -> usize {
        rng.random_range(0..PLAY_BOX_ROTATE_COUNT)
    }
}

pub const PLAY_BOX_BITMAP_SIZE: usize = 4;
pub const PLAY_BOX_ROTATE_COUNT: usize = 4;

type BitMap = [[u8; PLAY_BOX_BITMAP_SIZE]; PLAY_BOX_BITMAP_SIZE];

#[derive(Debug, Deserialize)]
pub struct PlayBoxConfig {
    bitmaps: [BitMap; PLAY_BOX_ROTATE_COUNT],
    pub level: u32,
    color: [u8; 4],
}

impl PlayBoxConfig {
    pub fn color(&self) -> Color {
        vec_to_color(&self.color)
    }

    pub fn bitmap(&self, rotate_index: usize) -> &BitMap {
        &self.bitmaps[rotate_index]
    }
}

#[derive(Resource, Debug)]
pub struct GameLib {
    pub origin_pos: Vec2,
    pub box_origin: Vec2,
    pub box_span: f32,
    pub box_mesh: Handle<Mesh>,
    pub box_colors: Vec<Handle<ColorMaterial>>,
    pub rng: StdRng,
}

impl GameLib {
    pub fn new(
        game_config: &GameConfig,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> GameLib {
        let panel_config = &game_config.game_panel_config;
        let box_config = &game_config.box_config;

        let origin_pos = -Vec2::new(
            game_config.window_size.width,
            game_config.window_size.height,
        ) / 2.0;

        let box_origin = origin_pos
            + vec_to_vec2(&panel_config.pos)
            + Vec2::splat(panel_config.border_breath + box_config.spacing)
            + Vec2::splat(box_config.size) / 2.0;

        let box_span = box_config.size + box_config.spacing;

        let box_mesh = meshes.add(Rectangle::new(box_config.size, box_config.size));

        let box_colors = Self::init_box_colors(&box_config.play_boxes, materials);

        let rng = StdRng::from_os_rng();

        let game_lib = GameLib {
            origin_pos,
            box_origin,
            box_span,
            box_mesh,
            box_colors,
            rng,
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
            let material = materials.add(b.color());
            colors.push(material);
        }
        colors
    }
}
