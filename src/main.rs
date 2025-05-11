use macroquad::{color::colors, prelude::*, Error};

#[macroquad::main("Green Island")]
async fn main() -> Result<(), Error> {
    loop {
        if is_key_pressed(KeyCode::Enter) {
            set_fullscreen(false);
        }
        if is_key_pressed(KeyCode::Escape) {
            set_fullscreen(true);
        }
        clear_background(RED);
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        let tiles = Texture2D::from_file_with_format(TILES, None);
        draw_texture_ex(&tiles, 0.0, 0.0, colors::WHITE, Default::default());
        next_frame().await
    }
}

const TILES: &[u8] = include_bytes!("../sprites/distinct.png");
