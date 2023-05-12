mod snake;
use macroquad::prelude::*;
use std::{thread, time};

use crate::snake::Game;

const DELAY: u64 = 15;
const FONT: &str = "./assets/mono.ttf";

fn window_conf() -> Conf {
    Conf {
        window_title: "snake-rs".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    let font = load_ttf_font(FONT).await.unwrap();
    game.random_apple();
    // Main game loop
    loop {
        clear_background(BLACK);
        if game.snake.alive {
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
        } else {
            // GAME OVER
            game.game_over(font);
        }
        // SCORE
        game.display_score(font);
        // SLEEP
        let sleep = time::Duration::from_millis(DELAY);
        thread::sleep(sleep);
        next_frame().await
    }
}
