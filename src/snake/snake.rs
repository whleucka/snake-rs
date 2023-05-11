use crate::snake::apple::Apple;
use crate::snake::seg::Seg;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Snake {
    pub alive: bool,
    pub direction: String,
    pub head: Seg,
    pub body: Option<Vec<Seg>>,
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
        let body = self.body.as_mut().unwrap();
        let color = match body.len() % 2 == 0 {
            true => YELLOW,
            false => GREEN,
        };
        body.push(Seg {
            x,
            y,
            color,
            ..Default::default()
        });
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
        self.head.x += self.head.radius as f32 * self.head.dx as f32;
        self.head.y += self.head.radius as f32 * self.head.dy as f32;
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
            if distance < body.radius as f64 {
                self.alive = false;
                return ();
            }
        });
    }
}
