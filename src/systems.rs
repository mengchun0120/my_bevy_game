use crate::game_lib::*;
use crate::game_panel::*;
use crate::utils::*;
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
    args: Res<Args>,
    mut exit_app: MessageWriter<AppExit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut window: Single<&mut Window>,
) {
    let game_lib = match GameLib::new(
        args.config_path.as_path(),
        meshes.as_mut(),
        materials.as_mut(),
    ) {
        Ok(lib) => lib,
        Err(err) => {
            error!("Failed to initialize GameLib {}", err);
            exit_app.write(AppExit::error());
            return;
        }
    };

    let window_size = &game_lib.config.window_size;
    window
        .resolution
        .set(window_size.width as f32, window_size.height as f32);
    window.resizable = false;

    commands.spawn(Camera2d);

    let game_panel = GamePanel::new(
        &mut commands,
        &game_lib,
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
    mut game_lib: ResMut<GameLib>,
    mut game_panel: ResMut<GamePanel>,
) {
    if game_panel.play_box.is_none() {
        game_panel.new_play_box(&mut commands, game_lib.as_mut());
    }
}
