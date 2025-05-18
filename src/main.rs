use game::Game;
use std::time::Instant;

mod assets;
mod game;
mod info;
mod world;

#[macroquad::main("Green Island")]
async fn main() {
    let start = Instant::now();
    let mut game = Game::default();
    dbg!(start.elapsed());
    game.run().await;
}
