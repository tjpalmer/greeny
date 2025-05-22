use game::Game;
use macroquad::prelude::*;

mod assets;
mod game;
mod info;
mod world;

fn window_conf() -> Conf {
    Conf {
        // fullscreen: true,
        window_height: 480,
        window_width: 840,
        window_title: "Green Island".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let start = get_time();
    let mut game = Game::default();
    let done = get_time();
    let init_ms = (done - start) * 1e3;
    info!("Started in {:.3}ms", init_ms);
    game.run().await;
}
