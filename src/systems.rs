use crate::game_lib::*;
use crate::game_panel::*;
use crate::utils::*;
use crate::play_box::*;
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
    commands.insert_resource(PlayBoxRecord(None));

    next_state.set(AppState::Playing);

    info!("Finished setting up game");
}

pub fn play_game(
    mut commands: Commands,
    mut game_lib: ResMut<GameLib>,
    game_panel: Res<GamePanel>,
    mut play_box: ResMut<PlayBoxRecord>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Some(b) = &mut play_box.0 {
        if keys.just_pressed(KeyCode::ArrowLeft) {
            try_move_left(b, &mut commands, game_lib.as_ref(), game_panel.as_ref());
        } else if keys.just_pressed(KeyCode::ArrowRight) {
            try_move_right(b, &mut commands, game_lib.as_ref(), game_panel.as_ref());
        }
    } else {
        play_box.0 = Some(PlayBox::new(game_lib.as_mut(), &mut commands));
    }
}

fn try_move_left(
    play_box: &mut PlayBox,
    commands: &mut Commands,
    game_lib: &GameLib,
    game_panel: &GamePanel,
) {
    let dest = BoxPos::new(play_box.pos.row, play_box.pos.col - 1);
    if game_panel.can_move_to(&dest, play_box, game_lib) {
        play_box.move_to(dest, commands, game_lib);
    }
}

fn try_move_right(
    play_box: &mut PlayBox,
    commands: &mut Commands,
    game_lib: &GameLib,
    game_panel: &GamePanel,
) {
    let dest = BoxPos::new(play_box.pos.row, play_box.pos.col + 1);
    if game_panel.can_move_to(&dest, play_box, game_lib) {
        play_box.move_to(dest, commands, game_lib);
    }
}
