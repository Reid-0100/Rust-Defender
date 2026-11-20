
mod player;
mod bullet;
mod enemy;
mod collision;
mod game;
mod ui;

use game::Game;
use macroquad::prelude::*;

#[macroquad::main("Rust Defender 2026")]
async fn main() {

    macroquad::rand::srand(
    macroquad::miniquad::date::now().to_bits()
);
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();

        next_frame().await;
    }
}
