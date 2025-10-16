use crate::game_lib::*;
use crate::game_panel::*;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Playing,
}

pub fn setup_game(
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    game_config: Res<GameConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut game_lib = GameLib::new(game_config.as_ref(), meshes.as_mut(), materials.as_mut());

    commands.spawn(Camera2d);

    let game_panel = GamePanel::new(
        &mut commands,
        game_config.as_ref(),
        &mut game_lib,
        meshes.as_mut(),
        materials.as_mut(),
    );

    commands.insert_resource(game_lib);
    commands.insert_resource(game_panel);

    next_state.set(AppState::Playing);

    info!("Finished setting up game");
}

pub fn play_game(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    mut game_lib: ResMut<GameLib>,
    mut game_panel: ResMut<GamePanel>,
) {
    if game_panel.play_box.is_none() {
        game_panel.new_play_box(&mut commands, game_config.as_ref(), game_lib.as_mut());
    }
}
