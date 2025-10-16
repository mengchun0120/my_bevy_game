use crate::game_lib::*;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Debug)]
pub enum BoxState {
    Active,
    Inactive,
}

#[derive(Resource, Debug)]
pub struct PlayBox {
    pub index_pos: [usize; 2],
    pub type_index: usize,
    pub rotate_index: usize,
}

impl PlayBox {
    pub fn new(
        index_pos: &[usize; 2],
        game_config: &GameConfig,
        game_lib: &mut GameLib,
        commands: &mut Commands,
    ) -> Self {
        let play_box = PlayBox {
            index_pos: index_pos.clone(),
            type_index: game_config.box_config.rand_type_index(&mut game_lib.rng),
            rotate_index: BoxConfig::rand_rotate_index(&mut game_lib.rng),
        };

        play_box.add_components(&play_box.index_pos, game_config, game_lib, commands);

        play_box
    }

    fn add_components(
        &self,
        index_pos: &[usize; 2],
        game_config: &GameConfig,
        game_lib: &mut GameLib,
        commands: &mut Commands,
    ) {
        let init_pos = Self::init_play_box_pos(index_pos, game_config, game_lib);
        let color = &game_lib.box_colors[self.type_index];
        let box_span = game_lib.box_span;
        let box_config = &game_config.box_config;
        let bitmap = box_config.play_box_bitmap(self.type_index, self.rotate_index);
        let mut y = init_pos.y;
        let z = box_config.z;

        for row in (0..PLAY_BOX_BITMAP_SIZE).rev() {
            let mut x = init_pos.x;
            for col in 0..PLAY_BOX_BITMAP_SIZE {
                if bitmap[row][col] != 0 {
                    commands.spawn((
                        Mesh2d(game_lib.box_mesh.clone()),
                        MeshMaterial2d(color.clone()),
                        Transform::from_xyz(x, y, z),
                        BoxState::Active,
                    ));
                }
                x += box_span;
            }
            y += box_span;
        }
    }

    fn init_play_box_pos(
        index_pos: &[usize; 2],
        game_config: &GameConfig,
        game_lib: &GameLib,
    ) -> Vec2 {
        let x = index_pos[1] as f32;
        let y = (game_config.game_panel_config.row_count() - index_pos[0] - 1) as f32;
        let offset = Vec2::new(x, y) * game_lib.box_span;
        game_lib.box_origin + offset
    }
}
