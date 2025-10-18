mod game_lib;
mod game_panel;
mod my_error;
mod play_box;
mod systems;
mod utils;

use crate::game_lib::*;
use crate::my_error::*;
use crate::systems::*;
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
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(config)
        .init_state::<AppState>()
        .add_systems(Startup, setup_game)
        .add_systems(Update, play_game.run_if(in_state(AppState::Playing)))
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
