use game::Game;

mod assets;
mod game;
mod info;
mod world;

#[macroquad::main("Green Island")]
async fn main() {
    let mut game = Game::default();
    game.run().await;
}
