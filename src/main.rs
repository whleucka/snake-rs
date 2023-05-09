use macroquad::prelude::*;

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
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct Snake {
    head: Seg,
    body: Option<Vec<Seg>>,
}

impl Snake {}

#[derive(Debug)]
pub struct Game {
    snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake {
                head: Seg {
                    color: GREEN,
                    ..Default::default()
                },
                body: Some(Vec::<Seg>::new()),
            },
        }
    }
    pub fn add_segment(&mut self, x: f32, y: f32) {
        self.snake.body.as_mut().unwrap().push(Seg::new(x, y));
    }
    pub fn up(&mut self) {
        self.snake.head.dx = 0;
        self.snake.head.dy = -1;
    }
    pub fn down(&mut self) {
        self.snake.head.dx = 0;
        self.snake.head.dy = 1;
    }
    pub fn left(&mut self) {
        self.snake.head.dx = -1;
        self.snake.head.dy = 0;
    }
    pub fn right(&mut self) {
        self.snake.head.dx = 1;
        self.snake.head.dy = 0;
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(BLACK);

        ///////////////////////////////////////////////////////////////////////
        // MOVEMENT
        // Player movement
        if is_key_down(KeyCode::J) {
            game.down();
        } else if is_key_down(KeyCode::K) {
            game.up();
        } else if is_key_down(KeyCode::H) {
            game.left();
        } else if is_key_down(KeyCode::L) {
            game.right();
        }
        // Move segments
        let segs = game.snake.body.as_mut().unwrap();
        for i in 0..segs.len() {
            if i == 0 {
                // Copy the head x,y
                segs[i].x = game.snake.head.x;
                segs[i].y = game.snake.head.y;
            } else {
                // Copy the previous seg x,y
                segs[i].x = segs[i - 1].x;
                segs[i].y = segs[i - 1].y;
            }
        }

        // Always keep the snake moving
        game.snake.head.x += game.snake.head.radius * game.snake.head.dx as f32;
        game.snake.head.y += game.snake.head.radius * game.snake.head.dy as f32;

        // Wrap the snake around the screen width,height
        if game.snake.head.x < 0.0 {
            game.snake.head.x = screen_width();
        } else if game.snake.head.x > screen_width() {
            game.snake.head.x = 0.0;
        }
        if game.snake.head.y < 0.0 {
            game.snake.head.y = screen_height();
        } else if game.snake.head.y > screen_height() {
            game.snake.head.y = 0.0;
        }

        ///////////////////////////////////////////////////////////////////////
        // DRAWING
        // Drawing the snake head
        draw_poly(
            game.snake.head.x,
            game.snake.head.y,
            game.snake.head.sides,
            game.snake.head.radius,
            game.snake.head.rotation,
            game.snake.head.color,
        );
        // Drawing the snake body segments
        game.snake.body.iter_mut().flatten().for_each(|snake| {
            draw_poly(
                snake.x,
                snake.y,
                snake.sides,
                snake.radius,
                snake.rotation,
                snake.color,
            );
        });
        next_frame().await
    }
}
