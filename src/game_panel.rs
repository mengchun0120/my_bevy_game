use crate::game_lib::*;
use crate::play_box::*;
use crate::utils::*;
use bevy::prelude::*;
use core::ops::Range;

#[derive(Resource, Debug)]
pub struct GamePanel {
    pub main_rows: usize,
    pub boxes: Vec<Vec<Option<Entity>>>,
    pub full_rows: Vec<usize>,
    pub height: usize,
    pub play_region: PlayBoxRegion,
}

impl GamePanel {
    pub fn new(
        commands: &mut Commands,
        game_lib: &GameLib,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        let panel_config = &game_lib.config.game_panel_config;

        let panel = Self {
            main_rows: panel_config.main_rows,
            boxes: vec![vec![None; panel_config.col_count()]; panel_config.row_count()],
            full_rows: Vec::new(),
            height: 0,
            play_region: Self::get_play_region(game_lib),
        };

        Self::create_panel(commands, game_lib, meshes, materials);

        info!("Game panel initialized");

        panel
    }

    #[inline]
    pub fn row_count(&self) -> usize {
        self.boxes.len()
    }

    #[inline]
    pub fn col_count(&self) -> usize {
        self.boxes[0].len()
    }

    #[inline]
    pub fn is_inside(&self, row: i32, col: i32) -> bool {
        (0..self.row_count() as i32).contains(&row) && (0..self.col_count() as i32).contains(&col)
    }

    pub fn put_in_entity(&mut self, row: i32, col: i32, entity: Entity) {
        if !self.is_inside(row, col) {
            panic!(
                "Failed to put entity into GamePanel: row={} or col={} is out of range",
                row, col
            );
        }

        let row = row as usize;
        let col = col as usize;

        if self.boxes[row][col].is_some() {
            panic!(
                "Failed to put entity into GamePanel: entity at row={} col={} is not empty",
                row, col
            );
        }

        self.boxes[row][col] = Some(entity);
    }

    pub fn init_pos(&self, index: &BoxIndex, game_lib: &GameLib) -> Option<BoxPos> {
        let box_size = game_lib.box_size(index);
        let max_row = self.row_count() as i32 - box_size.height as i32;
        let init_row = self.main_rows as i32 - box_size.height as i32;
        let col = (self.col_count() as i32 - box_size.width as i32) / 2;

        for row in init_row..=max_row {
            let pos = BoxPos::new(row, col);
            if self.can_move_to(&pos, index, game_lib) {
                return Some(pos);
            }
        }

        None
    }

    pub fn can_move_to(&self, pos: &BoxPos, index: &BoxIndex, game_lib: &GameLib) -> bool {
        let config = &game_lib.config;
        let bmp = config.box_config.play_box_bitmap(index);
        let mut row = pos.row;

        for r in (0..PLAY_BOX_BITMAP_SIZE).rev() {
            let mut col = pos.col;
            for c in 0..PLAY_BOX_BITMAP_SIZE {
                if bmp[r][c] != 0
                    && (!self.is_inside(row, col)
                        || self.boxes[row as usize][col as usize].is_some())
                {
                    return false;
                }
                col += 1;
            }
            row += 1;
        }

        return true;
    }

    pub fn put_down_play_box(&mut self, play_box: &mut PlayBox, game_lib: &GameLib) {
        if !play_box.is_valid() {
            return;
        }

        let index = play_box.index().unwrap().clone();
        let pos = play_box.pos().clone();

        play_box.put_in_panel(game_lib, self);
        self.update_height(&index, &pos, game_lib);
        self.check_full_rows(&index, &pos, game_lib);
    }

    pub fn reach_top(&self) -> bool {
        self.height >= self.main_rows
    }

    pub fn has_full_lines(&self) -> bool {
        self.full_rows.len() > 0
    }

    pub fn toggle_full_rows_visibility(&self, commands: &mut Commands) {
        for row in self.full_rows.iter() {
            for e in self.boxes[*row].iter() {
                if let Some(e1) = e {
                    commands
                        .entity(e1.clone())
                        .entry::<Visibility>()
                        .and_modify(|mut v| {
                            v.toggle_visible_hidden();
                        });
                }
            }
        }
    }

    pub fn remove_full_rows(&mut self, commands: &mut Commands, game_lib: &GameLib) {
        if self.full_rows.is_empty() {
            return;
        }

        self.despawn_full_rows(commands);

        let move_ranges = self.get_move_ranges();
        for (range, offset) in move_ranges {
            self.copy_rows(range, offset);
        }

        let clear_start_row = self.height - self.full_rows.len();
        self.clear_rows(clear_start_row..self.height);

        let update_start_row = self.full_rows[0];
        if update_start_row < clear_start_row {
            self.update_rows_pos(update_start_row, clear_start_row, commands, game_lib);
        }

        self.height -= self.full_rows.len();
        self.full_rows.clear();
    }

    fn get_play_region(game_lib: &GameLib) -> PlayBoxRegion {
        let panel_config = &game_lib.config.game_panel_config;
        PlayBoxRegion::new(
            Self::get_box_origin(game_lib),
            panel_config.main_rows,
            panel_config.col_count(),
        )
    }

    fn get_box_origin(game_lib: &GameLib) -> Vec2 {
        let panel_config = &game_lib.config.game_panel_config;
        let box_config = &game_lib.config.box_config;
        game_lib.origin_pos
            + vec_to_vec2(&panel_config.pos)
            + Vec2::splat(panel_config.border_breath + box_config.spacing)
            + Vec2::splat(box_config.size) / 2.0
    }

    fn create_panel(
        commands: &mut Commands,
        game_lib: &GameLib,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) {
        let panel_config = &game_lib.config.game_panel_config;
        let (internal_size, total_size) = Self::calculate_size(game_lib);
        let background_color = vec_to_color(&panel_config.background_color);
        let border_color = vec_to_color(&panel_config.border_color);
        let pos = game_lib.origin_pos
            + vec_to_vec2(&panel_config.pos)
            + Vec2::new(total_size.width, total_size.height) / 2.0;

        create_rect(
            &pos,
            panel_config.background_z,
            &internal_size,
            background_color,
            commands,
            meshes,
            materials,
        );

        create_rect(
            &pos,
            panel_config.border_z,
            &total_size,
            border_color,
            commands,
            meshes,
            materials,
        );
    }

    fn calculate_size(game_lib: &GameLib) -> (RectSize, RectSize) {
        let spacing = game_lib.config.box_config.spacing;
        let box_span = game_lib.box_span;
        let panel_config = &game_lib.config.game_panel_config;

        let internal_size = RectSize {
            width: (panel_config.col_count() as f32) * box_span + spacing,
            height: (panel_config.main_rows as f32) * box_span + spacing,
        };

        let total_size = RectSize {
            width: internal_size.width + panel_config.border_breath * 2.0,
            height: internal_size.height + panel_config.border_breath * 2.0,
        };

        (internal_size, total_size)
    }

    fn update_height(&mut self, index: &BoxIndex, pos: &BoxPos, game_lib: &GameLib) {
        let s = game_lib.box_size(index);
        let new_height = pos.row as usize + s.height as usize;
        if new_height > self.height {
            self.height = new_height;
        }
    }

    fn check_full_rows(&mut self, index: &BoxIndex, pos: &BoxPos, game_lib: &GameLib) {
        let s = game_lib.box_size(index);
        let start_row = pos.row as usize;
        let end_row = start_row + (s.height as usize);

        self.full_rows.clear();
        for row in start_row..end_row {
            if self.is_full_row(row) {
                self.full_rows.push(row);
            }
        }
    }

    fn is_full_row(&self, row: usize) -> bool {
        self.boxes[row].iter().all(|item| item.is_some())
    }

    fn despawn_full_rows(&mut self, commands: &mut Commands) {
        for row in self.full_rows.iter() {
            for col in 0..self.col_count() {
                if let Some(e) = self.boxes[*row][col] {
                    commands.entity(e.clone()).despawn();
                }
            }
        }
    }

    fn get_move_ranges(&self) -> Vec<(Range<usize>, usize)> {
        let last_index = self.full_rows.len() - 1;
        let mut result: Vec<(Range<usize>, usize)> = Vec::new();

        for i in 0..last_index {
            let start_row = self.full_rows[i] + 1;
            let end_row = self.full_rows[i + 1];
            if start_row < end_row {
                result.push((start_row..end_row, i + 1));
            }
        }

        let start_row = self.full_rows[last_index] + 1;
        let end_row = self.height;
        if start_row < end_row {
            result.push((start_row..end_row, last_index + 1));
        }

        result
    }

    fn copy_rows(&mut self, range: Range<usize>, offset: usize) {
        for r in range {
            self.copy_row(r - offset, r);
        }
    }

    fn clear_rows(&mut self, range: Range<usize>) {
        for row in range {
            for col in 0..self.col_count() {
                self.boxes[row][col] = None;
            }
        }
    }

    fn copy_row(&mut self, dest_row: usize, src_row: usize) {
        for col in 0..self.col_count() {
            self.boxes[dest_row][col] = self.boxes[src_row][col];
        }
    }

    fn update_rows_pos(
        &self,
        start_row: usize,
        end_row: usize,
        commands: &mut Commands,
        game_lib: &GameLib,
    ) {
        let init_pos = get_box_pos(
            &self.play_region.box_origin,
            start_row as i32,
            0,
            game_lib.box_span,
        );
        let span = game_lib.box_span;
        let mut y = init_pos.y;

        for row in start_row..end_row {
            let mut x = init_pos.x;
            for col in 0..self.col_count() {
                if let Some(e) = self.boxes[row][col] {
                    let pos = Vec2::new(x, y);
                    commands
                        .entity(e.clone())
                        .entry::<Transform>()
                        .and_modify(move |mut t| {
                            t.translation.x = pos.x;
                            t.translation.y = pos.y;
                        });
                }
                x += span;
            }
            y += span;
        }
    }
}
