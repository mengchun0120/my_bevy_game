use serde::Deserialize;
use bevy::prelude::*;

#[derive(Debug, Deserialize)]
struct BoxConfig {
    bitmaps: [[[i32; 4]; 4]; 4],
    level: i32,
    color: [f32; 4],
}

#[derive(Deserialize, Debug)]
struct ScreenConfig {
    width: f32,
    height: f32,
}

#[derive(Debug, Deserialize, Resource)]
struct GameConfig {
    screen: ScreenConfig,
    boxes: Vec<BoxConfig>,
}