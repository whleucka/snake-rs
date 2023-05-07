use macroquad::prelude::*;

#[derive(Debug)]
pub struct Seg {
    head: bool,
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
    pub fn new(head: bool) -> Self {
        Self {
            head,
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
pub struct Game {
    snake: Option<Vec<Seg>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Some(Vec::<Seg>::new()),
        }
    }
    pub fn add_segment(&mut self, head: bool) {
        self.snake.as_mut().unwrap().push(Seg::new(head));
    }
    pub fn up(&mut self) {
        self.snake
            .iter_mut()
            .flatten()
            .filter(|x| x.head)
            .for_each(|head| {
                head.dx = 0;
                head.dy = -1;
            });
    }
    pub fn down(&mut self) {
        self.snake
            .iter_mut()
            .flatten()
            .filter(|x| x.head)
            .for_each(|head| {
                head.dx = 0;
                head.dy = 1;
            });
    }
    pub fn left(&mut self) {
        self.snake
            .iter_mut()
            .flatten()
            .filter(|x| x.head)
            .for_each(|head| {
                head.dx = -1;
                head.dy = 0;
            });
    }
    pub fn right(&mut self) {
        self.snake
            .iter_mut()
            .flatten()
            .filter(|x| x.head)
            .for_each(|head| {
                head.dx = 1;
                head.dy = 0;
            });
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game = Game::new();
    game.add_segment(true);

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
        // Moving the snake x,y position
        game.snake.iter_mut().flatten().for_each(|snake| {
            snake.x += snake.radius * snake.dx as f32;
            snake.y += snake.radius * snake.dy as f32;
            if snake.x < 0.0 {
                snake.x = screen_width();
            } else if snake.x > screen_width() {
                snake.x = 0.0;
            }
            if snake.y < 0.0 {
                snake.y = screen_height();
            } else if snake.y > screen_height() {
                snake.y = 0.0;
            }
        });
        // Drawing the snake segments
        game.snake.iter_mut().flatten().for_each(|snake| {
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
