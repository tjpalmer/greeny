use game::Game;

mod assets;
mod game;
mod info;

#[macroquad::main("Green Island")]
async fn main() {
    let mut game = Game::default();
    game.run().await;
}
