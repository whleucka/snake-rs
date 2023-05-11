use crate::snake::apple::Apple;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Apples {
    pub apples: Option<Vec<Apple>>,
}

impl Apples {
    pub fn draw(&mut self) {
        self.apples
            .iter_mut()
            .flatten()
            .filter(|apple| apple.active)
            .for_each(|apple| {
                draw_poly(
                    apple.x,
                    apple.y,
                    apple.sides,
                    apple.radius,
                    apple.rotation,
                    apple.color,
                );
            });
    }
}
