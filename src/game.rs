use macroquad::{miniquad::window::screen_size, prelude::*};
// use std::time::{Duration, Instant};

use crate::assets::Assets;
use crate::info::{GameMetrics, ScreenMetrics};
use crate::world::{AnimalKind, Occupant, Plant, Tile, World};

#[derive(Default)]
pub struct Game {
    assets: Option<Assets>,
    fullscreen: bool,
    facing_x: f32,
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
        // Sky.
        draw_rectangle(
            screen_metrics.sky_start.x,
            screen_metrics.sky_start.y,
            screen_metrics.sky_size.x,
            screen_metrics.sky_size.y,
            Color::from_hex(0xA5C7ED),
        );
        // Mountains.
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
        // Behind horizon.
        self.draw_world(false);
        // Ground.
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
                flip_x: self.facing_x < 0.0,
                source: Some(Rect::new(
                    assets.tile_info.runner.x,
                    assets.tile_info.runner.y,
                    game_metrics.tile_size_px.x,
                    game_metrics.tile_size_px.y,
                )),
                ..Default::default()
            },
        );
        self.draw_world(true);
        self.mask_edges();
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
        let source = match tile.occupant {
            Some(occupant) => match occupant {
                Occupant::Animal(animal_idx) => {
                    let animal = self.world.animals[animal_idx];
                    let pos = match animal.kind {
                        AnimalKind::Bead => tile_info.bead,
                        AnimalKind::Bob => tile_info.bob,
                        AnimalKind::Coyote => tile_info.coyote,
                        AnimalKind::Jack => tile_info.jack,
                        AnimalKind::Javelina => tile_info.javelina,
                        AnimalKind::Rattler => tile_info.rattler,
                        AnimalKind::Runner => tile_info.runner,
                        AnimalKind::Turkey => tile_info.turkey,
                    };
                    let size = game_metrics.tile_size_px;
                    Rect::new(pos.x, pos.y, size.x, size.y)
                }
                Occupant::Plant(plant) => match plant {
                    Plant::NopalBig => tile_info.nopal_big,
                    Plant::NopalSmall => tile_info.nopal_small,
                    Plant::Ocotillo => tile_info.ocotillo,
                    Plant::Saguaro => tile_info.saguaro,
                },
            },
            None => return,
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

    fn draw_world(&self, front: bool) {
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
            let draw_y = match front {
                false => {
                    if (y as f32) >= self.pos.y - game_metrics.ground_center.y {
                        continue;
                    }
                    // TODO This formula ended up being hacky.
                    self.pos.y - (y as f32 - self.pos.y) - margin.y
                }
                true => {
                    if (y as f32) < self.pos.y - game_metrics.ground_center.y {
                        continue;
                    }
                    y as f32
                }
            };
            for x in start.x as usize..end.x as usize {
                let tile = world.grid.at(x, y);
                let pos = Vec2::new(x as f32, draw_y) - start - margin;
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
        self.facing_x = 1.0;
        self.pos = Vec2::floor(world.grid.size() * 0.5);
    }

    fn mask_edges(&self) {
        // Better would be scissors or render to texture then to screen.
        // I can't seem to access scissors, and render to texture was failing in wasm.
        let Self { screen_metrics, .. } = self;
        let screen_size = Vec2::from_array(screen_size().into());
        draw_rectangle(0.0, 0.0, screen_size.x, screen_metrics.full_start.y, BLACK);
        draw_rectangle(0.0, 0.0, screen_metrics.full_start.x, screen_size.y, BLACK);
        let full_end = screen_metrics.full_start + screen_metrics.full_size;
        draw_rectangle(
            full_end.x,
            0.0,
            screen_size.x - full_end.x,
            screen_size.y,
            BLACK,
        );
        draw_rectangle(
            0.0,
            full_end.y,
            screen_size.x,
            screen_size.y - full_end.y,
            BLACK,
        );
    }

    fn maybe_move_by(&mut self, vec: Vec2) {
        let next = self.pos + vec;
        if self.world.occupied(next) {
            if vec.x != 0.0 {
                // Provide an option to change facing when against a wall.
                // TODO Or will this be an attack later?
                self.facing_x = -self.facing_x;
            }
        } else {
            if vec.x != 0.0 {
                self.facing_x = vec.x;
            }
            self.pos = next;
        }
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
