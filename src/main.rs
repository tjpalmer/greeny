use game::Game;
use macroquad::prelude::*;

mod assets;
mod game;
mod info;
mod world;

#[macroquad::main("Green Island")]
async fn main() {
    let start = get_time();
    let mut game = Game::default();
    let done = get_time();
    let init_ms = (done - start) * 1e3;
    dbg!(init_ms);
    game.run().await;
}
