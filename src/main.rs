mod game_lib;
mod my_error;
mod utils;
use crate::game_lib::*;
use crate::my_error::*;
use bevy::window::WindowResolution;
use bevy::{log::LogPlugin, prelude::*};
use clap::Parser;
use std::{fs::File, path::PathBuf};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

struct LogFileGuard(WorkerGuard);

fn main() -> Result<(), MyError> {
    let args = Cli::parse();
    let _log_guard = setup_log(&args.log_path);
    let config = GameConfig::read(&args.config_path)?;

    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .disable::<LogPlugin>()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            config.window_size[0],
                            config.window_size[1],
                        ),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(config)
        .init_state::<AppState>()
        .add_systems(Startup, setup_game)
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

fn setup_log(log_path: &std::path::PathBuf) -> LogFileGuard {
    let log_file = File::create(log_path).expect("Open file");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(log_file);

    let file_layer = fmt::layer()
        .with_ansi(false) // Disable ANSI color codes for the file to keep it clean
        .with_writer(non_blocking_appender)
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
        .with_thread_names(true);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(file_layer)
        .init();

    LogFileGuard(guard)
}

fn setup_game(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let game_lib = GameLib::new(game_config.as_ref(), meshes.as_mut(), materials.as_mut());
    commands.insert_resource(game_lib);

    info!("Finished setting up game");
}

fn setup_game_panel(
    commands: &mut Commands,
    game_config: &GameConfig,
    game_lib: &GameLib,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let box_config = &game_config.box_config;
    let box_span = box_config.size + box_config.spacing;
    let panel_config = &game_config.game_panel_config;
    let panel_internal_width = (panel_config.size[0] as f32) * box_span + box_config.spacing;
    let panel_internal_height = (panel_config.size[1] as f32) * box_span + box_config.spacing;
    let panel_width = panel_internal_width + panel_config.border_breath * 2.0;
    let panel_height = panel_internal_height + panel_config.border_breath * 2.0;
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Menu,
}
