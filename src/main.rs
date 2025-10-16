mod game_lib;
mod game_panel;
mod my_error;
mod play_box;
mod utils;

use crate::game_lib::*;
use crate::game_panel::*;
use crate::my_error::*;
use crate::utils::*;
use bevy::window::WindowResolution;
use bevy::{log::LogPlugin, prelude::*};
use clap::Parser;
use std::path::PathBuf;

fn main() -> Result<(), MyError> {
    let args = Cli::parse();
    let _log_guard = setup_log(&args.log_path);
    let config: GameConfig = read_json(&args.config_path)?;

    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .disable::<LogPlugin>()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            config.window_size.width,
                            config.window_size.height,
                        ),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(config)
        .init_state::<AppState>()
        .add_systems(Startup, setup_game)
        .add_systems(Update, play_game)
        .run();

    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    log_path: PathBuf,

    #[arg(short, long)]
    config_path: PathBuf,
}

fn setup_game(
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

fn play_game(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    mut game_lib: ResMut<GameLib>,
    mut game_panel: ResMut<GamePanel>,
) {
    if game_panel.play_box.is_none() {
        game_panel.new_play_box(&mut commands, game_config.as_ref(), game_lib.as_mut());
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Playing,
}
