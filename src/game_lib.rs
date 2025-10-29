use crate::my_error::*;
use crate::play_box::*;
use crate::utils::*;
use bevy::prelude::*;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Resource)]
pub struct GameConfig {
    pub window_size: ISize,
    pub game_panel_config: GamePanelConfig,
    pub box_config: BoxConfig,
    pub drop_down_interval: f32,
    pub fast_down_interval: f32,
    pub fast_down_max_steps: u32,
    pub flash_full_line_interval: f32,
    pub flash_full_line_max_count: u32,
    pub preview_config: PreviewConfig,
}

#[derive(Debug, Deserialize)]
pub struct GamePanelConfig {
    size: [usize; 2],
    pub pos: [f32; 2],
    pub background_color: [u8; 4],
    pub border_color: [u8; 4],
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

    pub fn play_box_bitmap(&self, index: &BoxIndex) -> &BitMap {
        &self.play_boxes[index.type_index].bitmaps[index.rotate_index]
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

    pub fn box_pos(&self, rotate_index: usize) -> Vec<BoxPos> {
        let mut row = 0;
        let bmp = &self.bitmaps[rotate_index];
        let mut result: Vec<BoxPos> = Vec::new();

        for r in (0..PLAY_BOX_BITMAP_SIZE).rev() {
            let mut col = 0;
            for c in 0..PLAY_BOX_BITMAP_SIZE {
                if bmp[r][c] != 0 {
                    result.push(BoxPos::new(row, col));
                }

                col += 1;
            }
            row += 1;
        }

        result
    }

    pub fn color(&self) -> Color {
        vec_to_color(&self.color)
    }
}

#[derive(Deserialize, Resource, Debug)]
pub struct PreviewConfig {
    pub pos: [f32; 2],
    pub background_color: [u8; 4],
    pub border_color: [u8; 4],
    pub background_z: f32,
    pub border_breath: f32,
    pub border_z: f32,
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
    pub box_positions: Vec<Vec<Vec<BoxPos>>>,
    pub preview_origin: Vec2,
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

        let box_positions = Self::init_box_positions(&box_config.play_boxes);

        let preview_config = &config.preview_config;
        let preview_origin = origin_pos
            + vec_to_vec2(&preview_config.pos)
            + Vec2::splat(preview_config.border_breath + box_config.spacing)
            + Vec2::splat(box_config.size) / 2.0;

        let game_lib = GameLib {
            config,
            origin_pos,
            box_origin,
            box_span,
            box_mesh,
            box_colors,
            box_sizes,
            box_positions,
            preview_origin,
        };

        info!("GameLib initialized");

        Ok(game_lib)
    }

    pub fn box_size(&self, index: &BoxIndex) -> &ISize {
        &self.box_sizes[index.type_index][index.rotate_index]
    }

    pub fn box_pos(&self, index: &BoxIndex) -> &Vec<BoxPos> {
        &self.box_positions[index.type_index][index.rotate_index]
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

    fn init_box_positions(play_boxes: &[PlayBoxConfig]) -> Vec<Vec<Vec<BoxPos>>> {
        let mut result: Vec<Vec<Vec<BoxPos>>> = Vec::new();

        for config in play_boxes {
            let mut pos: Vec<Vec<BoxPos>> = Vec::new();
            for rotate_index in 0..PLAY_BOX_ROTATE_COUNT {
                pos.push(config.box_pos(rotate_index));
            }
            result.push(pos);
        }

        result
    }
}
