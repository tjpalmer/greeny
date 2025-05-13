use macroquad::{Error, miniquad::window::screen_size, prelude::*};

#[macroquad::main("Green Island")]
async fn main() -> Result<(), Error> {
    let mut game = Game::default();
    let mut fullscreen = false;
    let tiles = Texture2D::from_file_with_format(TILES, None);
    tiles.set_filter(FilterMode::Nearest);
    loop {
        game.update_screen();
        if is_key_pressed(KeyCode::F11)
            || (is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt))
                && is_key_pressed(KeyCode::Enter)
        {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen);
        }
        let Game { screen_metrics, .. } = game;
        clear_background(BLACK);
        draw_rectangle(
            screen_metrics.sky_start.x,
            screen_metrics.sky_start.y,
            screen_metrics.sky_size.x,
            screen_metrics.sky_size.y,
            Color::from_hex(0xA5C7ED),
        );
        draw_rectangle(
            screen_metrics.ground_start.x,
            screen_metrics.ground_start.y,
            screen_metrics.ground_size.x,
            screen_metrics.ground_size.y,
            Color::from_hex(0xC5AD95),
        );
        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_texture_ex(
            &tiles,
            screen_metrics.full_start.x,
            screen_metrics.full_start.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(screen_metrics.full_size),
                ..Default::default()
            },
        );
        next_frame().await
    }
}

#[derive(Default)]
struct Game {
    game_metrics: GameMetrics,
    screen_metrics: ScreenMetrics,
}

impl Game {
    fn update_screen(&mut self) {
        let screen_size = Vec2::from_array(screen_size().into());
        let Self { game_metrics, .. } = self;
        let scale = Vec2::floor(screen_size / game_metrics.full_size_px);
        let scale = Vec2::splat(scale.x.min(scale.y));
        let full_size = scale * game_metrics.full_size_px;
        let full_start = Vec2::floor((screen_size - full_size) * 0.5);
        let ground_start = full_start + scale * game_metrics.ground_start_px;
        let ground_size = scale * game_metrics.ground_size_px;
        let sky_size = scale * game_metrics.sky_size_px;
        let sky_start = full_start;
        self.screen_metrics = ScreenMetrics {
            full_size,
            full_start,
            ground_size,
            ground_start,
            sky_size,
            sky_start,
        };
    }
}

#[allow(unused)]
#[derive(Clone, Copy, Debug)]
struct GameMetrics {
    full_size: Vec2,
    full_size_px: Vec2,
    ground_center: Vec2,
    ground_center_full_px: Vec2,
    ground_size: Vec2,
    ground_size_px: Vec2,
    ground_start: Vec2,
    ground_start_px: Vec2,
    sky_size: Vec2,
    sky_size_px: Vec2,
    tile_size_px: Vec2,
}

#[derive(Clone, Copy, Debug, Default)]
struct ScreenMetrics {
    full_size: Vec2,
    full_start: Vec2,
    ground_size: Vec2,
    ground_start: Vec2,
    sky_size: Vec2,
    sky_start: Vec2,
}

impl GameMetrics {
    fn new() -> Self {
        let full_size = Vec2::splat(15.0);
        let sky_size = Vec2::new(full_size.x, 4.0);
        let ground_size = Vec2::new(full_size.x, full_size.y - sky_size.y);
        let ground_start = Vec2::new(0.0, sky_size.y);
        let ground_center = Vec2::floor(ground_size * 0.5);
        let tile_size_px = Vec2::splat(10.0);
        let ground_start_px = ground_start * tile_size_px;
        Self {
            full_size,
            full_size_px: full_size * tile_size_px,
            ground_center,
            ground_center_full_px: ground_center * tile_size_px + ground_start_px,
            ground_size,
            ground_size_px: ground_size * tile_size_px,
            ground_start,
            ground_start_px,
            sky_size,
            sky_size_px: sky_size * tile_size_px,
            tile_size_px,
        }
    }
}

impl Default for GameMetrics {
    fn default() -> Self {
        Self::new()
    }
}

const TILES: &[u8] = include_bytes!("../sprites/distinct.png");
