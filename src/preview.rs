use crate::game_lib::*;
use crate::play_box::*;
use crate::utils::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Preview {
    pub play_box: Option<PlayBox>,
    pub box_origin: Vec2,
}

impl Preview {
    pub fn new(
        commands: &mut Commands,
        game_lib: &GameLib,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        let preview = Preview {
            play_box: None,
            box_origin: Self::get_box_origin(game_lib),
        };

        Self::create_panel(commands, game_lib, meshes, materials);

        info!("Preview initialized successfully");

        preview
    }

    pub fn init_box(
        &mut self,
        index_gen: &mut IndexGen,
        commands: &mut Commands,
        game_lib: &GameLib,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) {
        if self.play_box.is_some() {
            return;
        }

        let play_box = PlayBox::new(
            index_gen.rand_box(),
            BoxPos::new(0, 0),
            &self.box_origin,
            commands,
            meshes,
            materials,
        );
    }

    fn get_box_origin(game_lib: &GameLib) -> Vec2 {
        let preview_config = &game_lib.config.preview_config;
        let box_config = &game_lib.config.box_config;
        game_lib.origin_pos
            + vec_to_vec2(&preview_config.pos)
            + Vec2::splat(preview_config.border_breath + box_config.spacing)
            + Vec2::splat(box_config.size) / 2.0
    }

    fn get_size(game_lib: &GameLib) -> (RectSize, RectSize) {
        let spacing = game_lib.config.box_config.spacing;
        let box_span = game_lib.box_span;
        let preview_config = &game_lib.config.preview_config;

        let internal_size = RectSize {
            width: (PLAY_BOX_BITMAP_SIZE as f32) * box_span + spacing,
            height: (PLAY_BOX_BITMAP_SIZE as f32) * box_span + spacing,
        };

        let total_size = RectSize {
            width: internal_size.width + preview_config.border_breath * 2.0,
            height: internal_size.height + preview_config.border_breath * 2.0,
        };

        (internal_size, total_size)
    }

    fn create_panel(
        commands: &mut Commands,
        game_lib: &GameLib,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) {
        let preview_config = &game_lib.config.preview_config;
        let (internal_size, total_size) = Self::get_size(game_lib);
        let background_color = vec_to_color(&preview_config.background_color);
        let border_color = vec_to_color(&preview_config.border_color);
        let pos = game_lib.origin_pos
            + vec_to_vec2(&preview_config.pos)
            + Vec2::new(total_size.width, total_size.height) / 2.0;

        create_rect(
            &pos,
            preview_config.background_z,
            &internal_size,
            background_color,
            commands,
            meshes,
            materials,
        );
        create_rect(
            &pos,
            preview_config.border_z,
            &total_size,
            border_color,
            commands,
            meshes,
            materials,
        );
    }
}
