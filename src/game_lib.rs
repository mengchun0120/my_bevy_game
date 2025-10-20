use crate::my_error::*;
use crate::play_box::BoxPos;
use crate::utils::*;
use bevy::prelude::*;
use rand::prelude::*;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Resource)]
pub struct GameConfig {
    pub window_size: ISize,
    pub game_panel_config: GamePanelConfig,
    pub box_config: BoxConfig,
}

#[derive(Debug, Deserialize)]
pub struct GamePanelConfig {
    size: [usize; 2],
    pos: [f32; 2],
    background_color: [u8; 4],
    border_color: [u8; 4],
    pub main_rows: usize,
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

    pub fn is_inside(&self, row: i32, col: i32) -> bool {
        (0..self.col_count() as i32).contains(&col)
            && (0..self.main_rows as i32).contains(&row)
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
        &self.play_boxes[type_index].bitmaps[rotate_index]
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
    pub bitmaps: [BitMap; PLAY_BOX_ROTATE_COUNT],
    pub level: u32,
    color: [u8; 4],
}

impl PlayBoxConfig {
    pub fn bmp_size(&self, rotate_index: usize) -> ISize {
        let mut min_col: Option<usize> = None;
        let mut max_col: Option<usize> = None;
        let mut min_row: Option<usize> = None;
        let mut max_row: Option<usize> = None;
        let bmp = self.bitmaps[rotate_index];

        for row in 0..PLAY_BOX_BITMAP_SIZE {
            let mut empty_row = true;
            for col in 0..PLAY_BOX_BITMAP_SIZE {
                if bmp[row][col] != 0 {
                    set_opt_min(&mut min_col, &col);
                    set_opt_max(&mut max_col, &col);
                    empty_row = false;
                }
            }

            if !empty_row {
                set_opt_min(&mut min_row, &row);
                set_opt_max(&mut max_row, &row);
            }
        }

        let width = if let (Some(min), Some(max)) = (min_col, max_col) {
            max - min + 1
        } else {
            0
        };

        let height = if let (Some(min), Some(max)) = (min_row, max_row) {
            max - min + 1
        } else {
            0
        };

        ISize {
            width: width as u32,
            height: height as u32,
        }
    }

    pub fn color(&self) -> Color {
        vec_to_color(&self.color)
    }
}

#[derive(Resource, Debug)]
pub struct GameLib {
    pub config: GameConfig,
    pub origin_pos: Vec2,
    pub box_origin: Vec2,
    pub box_span: f32,
    pub box_mesh: Handle<Mesh>,
    pub box_colors: Vec<Handle<ColorMaterial>>,
    pub box_sizes: Vec<Vec<ISize>>,
    pub rng: StdRng,
}

impl GameLib {
    pub fn new<P: AsRef<Path>>(
        path: P,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Result<Self, MyError> {
        let config: GameConfig = read_json(path)?;

        let panel_config = &config.game_panel_config;
        let box_config = &config.box_config;

        let origin_pos = -Vec2::new(
            config.window_size.width as f32,
            config.window_size.height as f32,
        ) / 2.0;

        let box_origin = origin_pos
            + vec_to_vec2(&panel_config.pos)
            + Vec2::splat(panel_config.border_breath + box_config.spacing)
            + Vec2::splat(box_config.size) / 2.0;

        let box_span = box_config.size + box_config.spacing;

        let box_mesh = meshes.add(Rectangle::new(box_config.size, box_config.size));

        let box_colors = Self::init_box_colors(&box_config.play_boxes, materials);

        let box_sizes = Self::init_box_sizes(&box_config.play_boxes);

        let rng = StdRng::from_os_rng();

        let game_lib = GameLib {
            config,
            origin_pos,
            box_origin,
            box_span,
            box_mesh,
            box_colors,
            box_sizes,
            rng,
        };

        info!("GameLib initialized");

        Ok(game_lib)
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

    fn init_box_sizes(play_boxes: &[PlayBoxConfig]) -> Vec<Vec<ISize>> {
        let mut result: Vec<Vec<ISize>> = Vec::new();

        for config in play_boxes {
            let mut sizes: Vec<ISize> = Vec::new();
            for rotate_index in 0..PLAY_BOX_ROTATE_COUNT {
                sizes.push(config.bmp_size(rotate_index));
            }
            result.push(sizes);
        }

        result
    }
}
