mod config;
mod my_error;
use bevy::{log::LogPlugin, prelude::*};
use std::{fs::File, path::PathBuf};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{prelude::*, fmt, EnvFilter};
use clap::Parser;
use crate::my_error::MyError;

use crate::config::GameConfig;

struct LogFileGuard(WorkerGuard);

fn main() -> Result<(), MyError> {
    let args = Cli::parse();

    let _log_guard = setup_log(&args.log_path);

    let config = GameConfig::read(&args.config_path)?;

    println!("{:?}", &config);

    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
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

fn setup_game() {
    println!("setup");
    info!("setup");
    warn!("warn");
    error!("error");
    debug!("debug");
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Menu,
}
