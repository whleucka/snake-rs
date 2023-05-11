use macroquad::prelude::*;

#[derive(Debug)]
pub struct Seg {
    pub dx: i8,
    pub dy: i8,
    pub x: f32,
    pub y: f32,
    pub sides: u8,
    pub radius: f32,
    pub rotation: f32,
    pub color: Color,
}

impl Default for Seg {
    fn default() -> Self {
        Self {
            dx: 1,
            dy: 0,
            x: screen_width() / 2.0f32,
            y: screen_height(),
            sides: 4,
            radius: 10.0,
            rotation: 45.0,
            color: GREEN,
        }
    }
}

impl Seg {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            dx: 1,
            dy: 0,
            sides: 4,
            radius: 10.0,
            rotation: 45.0,
            color: GREEN,
        }
    }
}
