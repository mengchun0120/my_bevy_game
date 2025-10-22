use crate::game_lib::*;
use crate::play_box::*;
use crate::utils::*;
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct GamePanel {
    pub panel: Vec<Vec<Option<Entity>>>,
}

impl GamePanel {
    pub fn new(
        commands: &mut Commands,
        game_lib: &GameLib,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        let config = &game_lib.config;
        let panel_config = &config.game_panel_config;
        let (internal_size, total_size) = Self::calculate_size(config, game_lib);
        let panel_pos = game_lib.origin_pos
            + panel_config.pos()
            + Vec2::new(total_size.width, total_size.height) / 2.0;

        Self::add_panel_internal(
            &internal_size,
            &panel_pos,
            panel_config,
            commands,
            meshes,
            materials,
        );
        Self::add_panel_border(
            &total_size,
            &panel_pos,
            panel_config,
            commands,
            meshes,
            materials,
        );

        let panel = Self {
            panel: vec![vec![None; panel_config.col_count()]; panel_config.row_count()],
        };

        info!("Game panel initialized");

        panel
    }

    pub fn can_move_to(&self, dest: &BoxPos, play_box: &PlayBox, game_lib: &GameLib) -> bool {
        let config = &game_lib.config;
        let bmp = config.box_config.play_box_bitmap(play_box.type_index, play_box.rotate_index);
        let panel_config = &config.game_panel_config;
        let mut row = dest.row;

        for r in (0..PLAY_BOX_BITMAP_SIZE).rev() {
            let mut col = dest.col;
            for c in 0..PLAY_BOX_BITMAP_SIZE {
                if bmp[r][c] == 0 {
                    continue;
                }

                if !panel_config.is_inside(row, col) || self.panel[row as usize][col as usize].is_some() {
                    return false;
                }

                col += 1;
            }
            row += 1;
        }

        return true;
    }

    fn calculate_size(game_config: &GameConfig, game_lib: &GameLib) -> (RectSize, RectSize) {
        let spacing = game_config.box_config.spacing;
        let box_span = game_lib.box_span;
        let panel_config = &game_config.game_panel_config;

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

    fn add_panel_internal(
        size: &RectSize,
        pos: &Vec2,
        panel_config: &GamePanelConfig,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) {
        let mesh = meshes.add(Rectangle::new(size.width, size.height));
        let material = materials.add(panel_config.background_color());
        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_xyz(pos.x, pos.y, panel_config.background_z),
        ));
    }

    fn add_panel_border(
        size: &RectSize,
        pos: &Vec2,
        panel_config: &GamePanelConfig,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) {
        let mesh = meshes.add(Rectangle::new(size.width, size.height));
        let material = materials.add(panel_config.border_color());
        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_xyz(pos.x, pos.y, panel_config.border_z),
        ));
    }
}
