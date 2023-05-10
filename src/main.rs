use crate::rand;
use macroquad::prelude::*;
use std::{thread, time};

const DELAY: u64 = 50;

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
        self.direction = String::from("up");
        self.head.dx = 0;
        self.head.dy = -1;
    }
    pub fn down(&mut self) {
        self.direction = String::from("down");
        self.head.dx = 0;
        self.head.dy = 1;
    }
    pub fn left(&mut self) {
        self.direction = String::from("left");
        self.head.dx = -1;
        self.head.dy = 0;
    }
    pub fn right(&mut self) {
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
}

#[derive(Debug)]
pub struct Game {
    snake: Snake,
    apples: Apples,
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
        }
    }
    pub fn add_apple(&mut self, x: f32, y: f32) {
        self.apples.apples.as_mut().unwrap().push(Apple::new(x, y));
    }
    pub fn random_apple(&mut self) {
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
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
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game = Game::new();
    let mut apples_remaining: i32 = 50;
    game.random_apple();

    loop {
        clear_background(BLACK);

        ///////////////////////////////////////////////////////////////////////
        // RENDER
        game.player_movement();
        game.snake.slither();

        ///////////////////////////////////////////////////////////////////////
        // APPLE
        let apples = game.apples.apples.as_mut().unwrap();
        let is_collision = game.snake.check_collision(apples);
        if is_collision {
            apples_remaining -= 1;
            game.random_apple();
        }

        ///////////////////////////////////////////////////////////////////////
        // DRAWING
        game.snake.draw();
        game.apples.draw();

        ///////////////////////////////////////////////////////////////////////
        // END GAME
        if apples_remaining <= 0 {
            println!("You win!");
            std::process::exit(0);
        }
        let sleep = time::Duration::from_millis(DELAY);
        thread::sleep(sleep);
        next_frame().await
    }
}
