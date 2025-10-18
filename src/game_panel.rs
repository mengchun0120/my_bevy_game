use crate::game_lib::*;
use crate::play_box::*;
use crate::utils::*;
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct GamePanel {
    pub play_box: Option<PlayBox>,
    pub panel: Vec<Vec<Option<Entity>>>,
}

impl GamePanel {
    pub fn new(
        commands: &mut Commands,
        game_config: &GameConfig,
        game_lib: &GameLib,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        let panel_config = &game_config.game_panel_config;
        let (internal_size, total_size) = Self::calculate_size(game_config, game_lib);
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
            play_box: None,
            panel: vec![vec![None; panel_config.col_count()]; panel_config.row_count()],
        };

        info!("Game panel initialized");

        panel
    }

    pub fn new_play_box(
        &mut self,
        commands: &mut Commands,
        game_config: &GameConfig,
        game_lib: &mut GameLib,
    ) {
        let pos = BoxPos::new(26, 0);
        let play_box = PlayBox::new(&pos, game_config, game_lib, commands);
        self.play_box = Some(play_box);
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
