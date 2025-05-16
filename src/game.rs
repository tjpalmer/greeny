use macroquad::{miniquad::window::screen_size, prelude::*};
// use std::time::{Duration, Instant};

use crate::assets::Assets;
use crate::info::{GameMetrics, ScreenMetrics};
use crate::world::{Plant, Tile, World};

#[derive(Default)]
pub struct Game {
    assets: Option<Assets>,
    fullscreen: bool,
    game_metrics: GameMetrics,
    pos: Vec2,
    screen_metrics: ScreenMetrics,
    world: World,
}

impl Game {
    pub async fn run(&mut self) {
        self.load();
        // set_camera(&Camera2D::from_display_rect(Rect::new(0.0, 0.0, 500.0, 500.0)));
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
        let pos = screen_metrics.tile(Vec2::new(7.0, 5.0));
        draw_texture_ex(
            &assets.tiles,
            pos.x,
            pos.y,
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
        self.draw_world();
    }

    fn draw_tile(&self, tile: Tile, pos: Vec2) {
        let Self {
            assets: Some(assets),
            game_metrics,
            screen_metrics,
            ..
        } = self
        else {
            panic!()
        };
        let Assets { tile_info, .. } = assets;
        let source = match tile.plant {
            Plant::None => return,
            Plant::NopalBig => tile_info.nopal_big,
            Plant::NopalSmall => tile_info.nopal_small,
            Plant::Ocotillo => tile_info.ocotillo,
            Plant::Saguaro => tile_info.saguaro,
        };
        let pos = screen_metrics.tile(pos)
            - Vec2::floor(
                (source.size() - game_metrics.tile_size_px) * Vec2::new(0.5, 1.0)
                    / game_metrics.tile_size_px,
            ) * game_metrics.tile_size_px
                * screen_metrics.scale;
        draw_texture_ex(
            &assets.tiles,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(screen_metrics.scale * source.size()),
                source: Some(source),
                ..Default::default()
            },
        );
    }

    fn draw_world(&self) {
        let Self {
            game_metrics,
            world,
            ..
        } = self;
        let min = Vec2::default();
        let max = world.grid.size();
        // Margin needs to be larger than any game item.
        let margin = Vec2::new(10.0, 10.0);
        let extent = game_metrics.ground_center + margin;
        let start = Vec2::clamp(self.pos - extent, min, max);
        let end = Vec2::clamp(self.pos + extent, min, max);
        for y in start.y as usize..end.y as usize {
            for x in start.x as usize..end.x as usize {
                // TODO Draw with proper offset.
                let tile = world.grid.at(x, y);
                let pos = Vec2::new(x as f32, y as f32) - start - margin;
                self.draw_tile(tile, pos);
            }
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::F11)
            || (is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt))
                && is_key_pressed(KeyCode::Enter)
        {
            self.fullscreen = !self.fullscreen;
            set_fullscreen(self.fullscreen);
        }
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.maybe_move_by(Vec2::new(0.0, -1.0));
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            self.maybe_move_by(Vec2::new(0.0, 1.0));
        }
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            self.maybe_move_by(Vec2::new(-1.0, 0.0));
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            self.maybe_move_by(Vec2::new(1.0, 0.0));
        }
    }

    fn load(&mut self) {
        self.assets = Some(Assets::load(&self.game_metrics));
        let Self { world, .. } = self;
        self.pos = Vec2::floor(world.grid.size() * 0.5);
    }

    fn maybe_move_by(&mut self, vec: Vec2) {
        let next = self.pos + vec;
        if !self.occupied(next) {
            self.pos = next;
        }
    }

    fn occupied(&self, vec: Vec2) -> bool {
        !matches!(
            self.world.grid.at(vec.x as usize, vec.y as usize).plant,
            Plant::None
        )
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
            scale,
            sky_size,
            sky_start,
            tile_size,
        };
    }
}
