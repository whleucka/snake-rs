pub mod apple;
pub mod apples;
pub mod seg;
pub mod snake;

use crate::snake::apple::Apple;
use crate::snake::apples::Apples;
use crate::snake::seg::Seg;
use crate::snake::snake::Snake;
use macroquad::prelude::*;

// ~60 FPS
const NUMBER_APPLES: u32 = 100;

#[derive(Debug)]
pub struct Game {
    pub snake: Snake,
    pub apples: Apples,
    pub count: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake {
                alive: true,
                direction: String::from("right"),
                head: Seg {
                    color: GREEN,
                    radius: 13.0,
                    ..Default::default()
                },
                body: Some(Vec::<Seg>::new()),
            },
            apples: Apples {
                apples: Some(Vec::<Apple>::new()),
            },
            count: NUMBER_APPLES,
        }
    }
    pub fn add_apple(&mut self, x: f32, y: f32) {
        self.apples.apples.as_mut().unwrap().push(Apple::new(x, y));
    }
    pub fn random_apple(&mut self) {
        let x = rand::gen_range(10.0, screen_width() - 10.0);
        let y = rand::gen_range(10.0, screen_height() - 10.0);
        self.add_apple(x, y);
    }
    pub fn player_movement(&mut self) {
        if is_key_down(KeyCode::J) {
            self.snake.down();
        } else if is_key_down(KeyCode::K) {
            self.snake.up();
        } else if is_key_down(KeyCode::H) {
            self.snake.left();
        } else if is_key_down(KeyCode::L) {
            self.snake.right();
        }
    }
    pub fn apple_collision(&mut self) {
        let apples = self.apples.apples.as_mut().unwrap();
        let is_collision = self.snake.check_collision(apples);
        if is_collision {
            self.count -= 1;
            self.random_apple();
        }
    }
    pub fn detect_endgame(&mut self) {
        if self.count <= 0 {
            println!("You win!");
            self.snake.alive = false;
        }
    }
    pub fn display_score(&mut self, font: Font) {
        let score = format!("SCORE: {}", (NUMBER_APPLES - self.count) * 100);
        draw_text_ex(
            &score,
            10.0,
            screen_height() - 10.0,
            TextParams {
                font_size: 16,
                font,
                ..Default::default()
            },
        );
    }
    pub fn game_over(&mut self, font: Font) {
        draw_text_ex(
            "GAME OVER",
            (screen_width() / 2.0) - 100.0,
            screen_height() / 2.0,
            TextParams {
                font_size: 40,
                font,
                ..Default::default()
            },
        );
    }
}
