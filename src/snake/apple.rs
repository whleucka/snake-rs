use macroquad::prelude::*;

#[derive(Debug)]
pub struct Apple {
    pub x: f32,
    pub y: f32,
    pub sides: u8,
    pub radius: f32,
    pub rotation: f32,
    pub color: Color,
    pub active: bool,
}
impl Default for Apple {
    fn default() -> Self {
        Self {
            x: rand::gen_range(0.0, screen_width()),
            y: rand::gen_range(0.0, screen_width()),
            sides: 4,
            radius: 10.0,
            rotation: 45.0,
            color: GREEN,
            active: true,
        }
    }
}

impl Apple {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            color: RED,
            ..Default::default()
        }
    }
}
