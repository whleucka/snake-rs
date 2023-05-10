use crate::rand;
use macroquad::prelude::*;
use std::{thread, time};

// ~60 FPS
const DELAY: u64 = 15;

#[derive(Debug)]
pub struct Apple {
    x: f32,
    y: f32,
    sides: u8,
    radius: f32,
    rotation: f32,
    color: Color,
    active: bool,
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

#[derive(Debug)]
pub struct Apples {
    apples: Option<Vec<Apple>>,
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

#[derive(Debug)]
pub struct Seg {
    dx: i8,
    dy: i8,
    x: f32,
    y: f32,
    sides: u8,
    radius: f32,
    rotation: f32,
    color: Color,
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

#[derive(Debug)]
pub struct Snake {
    direction: String,
    head: Seg,
    body: Option<Vec<Seg>>,
}

impl Snake {
    pub fn up(&mut self) {
        if &self.direction == "down" {
            return;
        }
        self.direction = String::from("up");
        self.head.dx = 0;
        self.head.dy = -1;
    }
    pub fn down(&mut self) {
        if &self.direction == "up" {
            return;
        }
        self.direction = String::from("down");
        self.head.dx = 0;
        self.head.dy = 1;
    }
    pub fn left(&mut self) {
        if &self.direction == "right" {
            return;
        }
        self.direction = String::from("left");
        self.head.dx = -1;
        self.head.dy = 0;
    }
    pub fn right(&mut self) {
        if &self.direction == "left" {
            return;
        }
        self.direction = String::from("right");
        self.head.dx = 1;
        self.head.dy = 0;
    }
    pub fn add_segment(&mut self, x: f32, y: f32) {
        self.body.as_mut().unwrap().push(Seg::new(x, y));
    }
    pub fn check_collision(&mut self, apples: &mut Vec<Apple>) -> bool {
        let mut found = false;
        apples
            .iter_mut()
            .filter(|apple| apple.active)
            .for_each(|apple| {
                let a = (apple.x - self.head.x) as f64;
                let b = (apple.y - self.head.y) as f64;
                let distance = f64::sqrt(a.powi(2) + b.powi(2));
                if !found && distance < apple.radius as f64 * 2.0 as f64 {
                    apple.active = false;
                    found = true;
                    self.add_segment(apple.x, apple.y);
                }
            });
        found
    }
    pub fn move_head(&mut self) {
        self.head.x += self.head.radius * 2.0 as f32 * self.head.dx as f32;
        self.head.y += self.head.radius * 2.0 as f32 * self.head.dy as f32;
    }
    pub fn move_body(&mut self) {
        let segs = self.body.as_mut().unwrap();
        // You must reverse the body so that you're moving from the tail
        // to the head. This way, the snake moves properly.
        for i in (0..segs.len()).rev() {
            if i == 0 {
                // Copy the head x,y
                segs[i].x = self.head.x;
                segs[i].y = self.head.y;
            } else {
                // Copy the next seg x,y
                segs[i].x = segs[i - 1].x;
                segs[i].y = segs[i - 1].y;
            };
        }
    }
    pub fn slither(&mut self) {
        // Wrap the snake around the screen width,height
        if self.head.x < 0.0 {
            self.head.x = screen_width();
        } else if self.head.x > screen_width() {
            self.head.x = 0.0;
        }
        if self.head.y < 0.0 {
            self.head.y = screen_height();
        } else if self.head.y > screen_height() {
            self.head.y = 0.0;
        }

        // Move segments
        self.move_body();
        // Move the snake head
        self.move_head();
    }
    pub fn draw_head(&mut self) {
        draw_poly(
            self.head.x,
            self.head.y,
            self.head.sides,
            self.head.radius,
            self.head.rotation,
            self.head.color,
        );
    }
    pub fn draw_body(&mut self) {
        self.body.iter_mut().flatten().for_each(|snake| {
            draw_poly(
                snake.x,
                snake.y,
                snake.sides,
                snake.radius,
                snake.rotation,
                snake.color,
            );
        });
    }
    pub fn draw(&mut self) {
        self.draw_head();
        self.draw_body();
    }
    pub fn head_collision(&mut self) {
        self.body.iter_mut().flatten().for_each(|body| {
            let a = (body.x - self.head.x) as f64;
            let b = (body.y - self.head.y) as f64;
            let distance = f64::sqrt(a.powi(2) + b.powi(2));
            if distance < body.radius as f64 * 2.0 as f64 {
                println!("You lose!");
                std::process::exit(0);
            }
        });
    }
}

#[derive(Debug)]
pub struct Game {
    snake: Snake,
    apples: Apples,
    count: i32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake {
                direction: String::from("right"),
                head: Seg {
                    color: GREEN,
                    ..Default::default()
                },
                body: Some(Vec::<Seg>::new()),
            },
            apples: Apples {
                apples: Some(Vec::<Apple>::new()),
            },
            count: 49,
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
            std::process::exit(0);
        }
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game = Game::new();
    game.random_apple();
    // Main game loop
    let mut count: u32 = 0;
    loop {
        clear_background(BLACK);
        // RENDER
        game.player_movement();
        game.snake.slither();
        // SELF COLLISION
        game.snake.head_collision();
        // APPLE COLLISION
        game.apple_collision();
        // DRAWING
        game.snake.draw();
        game.apples.draw();
        // END GAME
        game.detect_endgame();
        // TIMER DELAY
        count += 1;
        if count % 100 == 0 {
            println!("FPS: {:.1}", get_fps());
        }
        let sleep = time::Duration::from_millis(DELAY);
        thread::sleep(sleep);
        next_frame().await
    }
}
