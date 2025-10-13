mod game_lib;
mod my_error;
mod play_box;
mod utils;

use crate::game_lib::*;
use crate::my_error::*;
use crate::play_box::*;
use crate::utils::read_json;
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
    let config: GameConfig = read_json(&args.config_path)?;

    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .disable::<LogPlugin>()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            config.window_width(),
                            config.window_height(),
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
    let mut game_lib = GameLib::new(game_config.as_ref(), meshes.as_mut(), materials.as_mut());

    commands.spawn(Camera2d);

    setup_game_panel(
        &mut commands,
        game_config.as_ref(),
        &game_lib,
        meshes.as_mut(),
        materials.as_mut(),
    );

    setup_play_box(game_config.as_ref(), &mut game_lib, &mut commands);

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
    let box_span = game_lib.box_span;
    let panel_config = &game_config.game_panel_config;
    let panel_internal_width = (panel_config.col_count() as f32) * box_span + box_config.spacing;
    let panel_internal_height = (panel_config.row_count() as f32) * box_span + box_config.spacing;
    let panel_width = panel_internal_width + panel_config.border_breath * 2.0;
    let panel_height = panel_internal_height + panel_config.border_breath * 2.0;
    let panel_pos =
        game_lib.origin_pos + panel_config.pos() + Vec2::new(panel_width, panel_height) / 2.0;

    let panel_internal_mesh =
        meshes.add(Rectangle::new(panel_internal_width, panel_internal_height));
    let panel_internal_material = materials.add(panel_config.background_color());
    commands.spawn((
        Mesh2d(panel_internal_mesh),
        MeshMaterial2d(panel_internal_material),
        Transform::from_xyz(panel_pos.x, panel_pos.y, panel_config.background_z),
    ));

    let panel_border_mesh = meshes.add(Rectangle::new(panel_width, panel_height));
    let panel_border_material = materials.add(panel_config.border_color());
    commands.spawn((
        Mesh2d(panel_border_mesh),
        MeshMaterial2d(panel_border_material),
        Transform::from_xyz(panel_pos.x, panel_pos.y, panel_config.border_z),
    ));

    info!("Game panel initialized");
}

fn setup_play_box(game_config: &GameConfig, game_lib: &mut GameLib, commands: &mut Commands) {
    let index_pos: [usize; 2] = [27, 0];
    PlayBox::add(&index_pos, game_config, game_lib, commands);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Menu,
}
