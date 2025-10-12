use bevy::prelude::*;
use serde::de::DeserializeOwned;
use serde_json;
use std::{fs::File, io::BufReader, path::Path};
use crate::my_error::MyError;

pub fn vec_to_vec2(v: &[f32; 2]) -> Vec2 {
    Vec2 { x: v[0], y: v[1] }
}

pub fn vec_to_color(v: &[u8; 4]) -> Color {
    Color::srgba_u8(v[0], v[1], v[2], v[3])
}

pub fn read_json<T, P>(path: P) -> Result<T, MyError>
where
    T: DeserializeOwned,
    P: AsRef<Path>
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result: T = serde_json::from_reader(reader)?;
    Ok(result)
}