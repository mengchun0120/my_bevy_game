use bevy::prelude::*;

pub fn vec_to_vec2(v: &[f32; 2]) -> Vec2 {
    Vec2 { x: v[0], y: v[1] }
}

pub fn vec_to_color(v: &[u8; 4]) -> Color {
    Color::srgba_u8(v[0], v[1], v[2], v[3])
}
