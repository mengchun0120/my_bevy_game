use crate::my_error::MyError;
use bevy::prelude::*;
use clap::Parser;
use serde::{Deserialize, de::DeserializeOwned};
use serde_json;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Parser, Resource)]
pub struct Args {
    #[arg(short, long)]
    pub log_path: PathBuf,

    #[arg(short, long)]
    pub config_path: PathBuf,
}

pub struct LogFileGuard(WorkerGuard);

#[derive(Debug, Deserialize, Resource)]
pub struct ISize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize, Resource)]
pub struct RectSize {
    pub width: f32,
    pub height: f32,
}

pub fn vec_to_vec2(v: &[f32; 2]) -> Vec2 {
    Vec2 { x: v[0], y: v[1] }
}

pub fn vec_to_color(v: &[u8; 4]) -> Color {
    Color::srgba_u8(v[0], v[1], v[2], v[3])
}

pub fn read_json<T, P>(path: P) -> Result<T, MyError>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result: T = serde_json::from_reader(reader)?;
    Ok(result)
}

pub fn setup_log<P: AsRef<Path>>(log_path: P) -> LogFileGuard {
    let log_file = File::create(log_path.as_ref()).expect("Open file");
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

pub fn set_opt_min<T: PartialOrd + Clone>(prev_value: &mut Option<T>, new_value: &T) {
    if let Some(value) = prev_value {
        if new_value < value {
            prev_value.replace(new_value.clone());
        }
    } else {
        prev_value.replace(new_value.clone());
    }
}

pub fn set_opt_max<T: PartialOrd + Clone>(prev_value: &mut Option<T>, new_value: &T) {
    if let Some(value) = prev_value {
        if new_value > value {
            prev_value.replace(new_value.clone());
        }
    } else {
        prev_value.replace(new_value.clone());
    }
}
