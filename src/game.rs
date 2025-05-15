use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::assets::Assets;
use crate::info::{GameMetrics, ScreenMetrics};
use crate::world::World;

#[derive(Default)]
pub struct Game {
    assets: Option<Assets>,
    fullscreen: bool,
    game_metrics: GameMetrics,
    screen_metrics: ScreenMetrics,
    world: World,
}

impl Game {
    pub async fn run(&mut self) {
        self.load();
        loop {
            self.update_screen();
            self.handle_input();
            self.draw();
            next_frame().await
        }
    }

    fn draw(&self) {
        let Self {
            assets: Some(assets),
            game_metrics,
            screen_metrics,
            ..
        } = self
        else {
            panic!()
        };
        clear_background(BLACK);
        draw_rectangle(
            screen_metrics.sky_start.x,
            screen_metrics.sky_start.y,
            screen_metrics.sky_size.x,
            screen_metrics.sky_size.y,
            Color::from_hex(0xA5C7ED),
        );
        draw_texture_ex(
            &assets.mountains,
            screen_metrics.full_start.x,
            screen_metrics.full_start.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(screen_metrics.full_size),
                ..Default::default()
            },
        );
        draw_rectangle(
            screen_metrics.ground_start.x,
            screen_metrics.ground_start.y,
            screen_metrics.ground_size.x,
            screen_metrics.ground_size.y,
            Color::from_hex(0xC5AD95),
        );
        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        let place = screen_metrics.tile(Vec2::new(7.0, 5.0));
        draw_texture_ex(
            &assets.tiles,
            place.x,
            place.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(screen_metrics.tile_size),
                source: Some(Rect::new(
                    assets.tile_info.roadie.x,
                    assets.tile_info.roadie.y,
                    game_metrics.tile_size_px.x,
                    game_metrics.tile_size_px.y,
                )),
                ..Default::default()
            },
        );
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::F11)
            || (is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt))
                && is_key_pressed(KeyCode::Enter)
        {
            self.fullscreen = !self.fullscreen;
            set_fullscreen(self.fullscreen);
        }
    }

    fn load(&mut self) {
        self.assets = Some(Assets::load(&self.game_metrics));
    }

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
        let tile_size = scale * game_metrics.tile_size_px;
        self.screen_metrics = ScreenMetrics {
            full_size,
            full_start,
            ground_size,
            ground_start,
            sky_size,
            sky_start,
            tile_size,
        };
    }
}
