use crate::game_lib::*;
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct PlayBoxRecord(pub Option<PlayBox>);

#[derive(Resource, Component, Debug, Clone)]
pub struct BoxPos {
    pub row: i32,
    pub col: i32,
}

impl BoxPos {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub fn to_panel_pos(&self, game_lib: &GameLib) -> Vec2 {
        let offset = Vec2::new(self.col as f32, self.row as f32) * game_lib.box_span;
        game_lib.box_origin + offset
    }

    pub fn reset(&mut self, row: i32, col: i32) {
        self.row = row;
        self.col = col;
    }
}

#[derive(Resource, Debug)]
pub struct PlayBox {
    pub pos: BoxPos,
    pub type_index: usize,
    pub rotate_index: usize,
    entities: Vec<Entity>,
}

impl PlayBox {
    pub fn new(game_lib: &mut GameLib, commands: &mut Commands) -> Self {
        let config = &game_lib.config;
        let type_index = config.box_config.rand_type_index(&mut game_lib.rng);
        let rotate_index = BoxConfig::rand_rotate_index(&mut game_lib.rng);
        let pos = Self::init_pos(game_lib, type_index, rotate_index);

        let mut play_box = PlayBox {
            pos,
            type_index,
            rotate_index,
            entities: Vec::new(),
        };

        play_box.add_components(game_lib, commands);

        play_box
    }

    pub fn move_to(&mut self, dest: BoxPos, commands: &mut Commands, game_lib: &GameLib) {
        let delta = dest.to_panel_pos(game_lib) - self.pos.to_panel_pos(game_lib);
        self.pos = dest;
        self.update_position(delta, commands);
    }

    fn init_pos(game_lib: &GameLib, type_index: usize, rotate_index: usize) -> BoxPos {
        let box_size = &game_lib.box_sizes[type_index][rotate_index];
        let panel_config = &game_lib.config.game_panel_config;
        let col = (panel_config.col_count() as u32 - box_size.width) / 2;
        let row = game_lib.config.game_panel_config.main_rows as u32 - box_size.height;
        BoxPos {
            row: row as i32,
            col: col as i32,
        }
    }

    fn add_components(&mut self, game_lib: &mut GameLib, commands: &mut Commands) {
        let config = &game_lib.config;
        let init_pos = self.pos.to_panel_pos(game_lib);
        let color = &game_lib.box_colors[self.type_index];
        let box_span = game_lib.box_span;
        let box_config = &config.box_config;
        let panel_config = &config.game_panel_config;
        let bitmap = box_config.play_box_bitmap(self.type_index, self.rotate_index);
        let mut y = init_pos.y;
        let z = box_config.z;
        let mut pos = self.pos.clone();

        self.entities.clear();
        for row in (0..PLAY_BOX_BITMAP_SIZE).rev() {
            let mut x = init_pos.x;
            pos.col = self.pos.col;
            for col in 0..PLAY_BOX_BITMAP_SIZE {
                if bitmap[row][col] != 0 {
                    let visibility = if panel_config.is_inside(pos.row, pos.col) {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    };
                    let e = commands.spawn((
                        Mesh2d(game_lib.box_mesh.clone()),
                        MeshMaterial2d(color.clone()),
                        Transform::from_xyz(x, y, z),
                        pos.clone(),
                        visibility,
                    ));
                    self.entities.push(e.id());
                }
                x += box_span;
                pos.col += 1;
            }
            y += box_span;
            pos.row += 1;
        }
    }

    fn update_position(&mut self, delta: Vec2, commands: &mut Commands) {
        let change_pos = move |mut t: Mut<'_, Transform>| {
            t.translation.x += delta.x;
            t.translation.y += delta.y;
        };

        for e in self.entities.iter() {
            commands.entity(e.clone())
                .entry::<Transform>()
                .and_modify(change_pos);
        }
    }
}
