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

impl Seg {
    pub fn new() -> Self {
        Self {
            dx: 1,
            dy: 0,
            x: 50.0,
            y: 50.0,
            sides: 4,
            radius: 10.0,
            rotation: 45.0,
            color: GREEN,
        }
    }
}

#[derive(Debug)]
pub struct Snake {
    head: Seg,
    body: Option<Vec<Seg>>,
}

#[derive(Debug)]
pub struct Game {
    snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake {
                head: Seg::new(),
                body: Some(Vec::<Seg>::new()),
            },
        }
    }
    pub fn add_segment(&mut self) {
        self.snake.body.as_mut().unwrap().push(Seg::new());
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
    game.add_segment();

    loop {
        clear_background(BLACK);
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
        game.snake
            .body
            .iter_mut()
            .flatten()
            .enumerate()
            .for_each(|(i, snake)| {
                if i == 0 {
                    snake.x = game.snake.head.x;
                    snake.y = game.snake.head.y;
                } else {
                    //snake.x = game.snake.body.as_ref().unwrap().get(i + 1).unwrap().x;
                    //snake.y = game.snake.body.as_ref().unwrap().get(i + 1).unwrap().y;
                }
            });
        // Moving the head x,y position
        game.snake.head.x += game.snake.head.radius * game.snake.head.dx as f32;
        game.snake.head.y += game.snake.head.radius * game.snake.head.dy as f32;
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

        // Drawing the snake segments
        draw_poly(
            game.snake.head.x,
            game.snake.head.y,
            game.snake.head.sides,
            game.snake.head.radius,
            game.snake.head.rotation,
            game.snake.head.color,
        );

        // Drawing the snake segments
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
