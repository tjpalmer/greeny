use macroquad::ui::{Skin, hash, root_ui, widgets};
use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::assets::Assets;
use crate::info::{GameMetrics, ScreenMetrics};
use crate::world::{AnimalKind, Occupant, Plant, Tile, World};

#[derive(Default)]
pub struct Game {
    input: Input,
    assets: Option<Assets>,
    fullscreen: bool,
    facing_x: f32,
    game_metrics: GameMetrics,
    icon_skin: Option<Skin>,
    mouse: Vec2,
    pos: Vec2,
    screen_metrics: ScreenMetrics,
    ui_hidden: bool,
    world: World,
}

#[derive(Default)]
pub struct Input {
    down: bool,
    left: bool,
    right: bool,
    up: bool,
}

impl Game {
    pub async fn run(&mut self) {
        self.load();
        // set_camera(&Camera2D::from_display_rect(Rect::new(0.0, 0.0, 500.0, 500.0)));
        loop {
            self.update_screen();
            if !self.ui_hidden {
                self.ui();
            }
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
        // Clip so we can be sloppy in drawing.
        let gl = unsafe { get_internal_gl().quad_gl };
        gl.scissor(Some((
            screen_metrics.full_start.x as i32,
            screen_metrics.full_start.y as i32,
            screen_metrics.full_size.x as i32,
            screen_metrics.full_size.y as i32,
        )));
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
                dest_size: Some(screen_metrics.full_size - vec2(0.0, game_metrics.tile_size_px.y)),
                source: Some(Rect::new(
                    0.0,
                    game_metrics.tile_size_px.y,
                    game_metrics.full_size_px.x,
                    game_metrics.full_size_px.y - game_metrics.tile_size_px.y,
                )),
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
        let pos = screen_metrics.tile(vec2(7.0, 5.0));
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
        // Reset clip rect to draw ui.
        gl.scissor(None);
        root_ui().pop_skin();
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
                (source.size() - game_metrics.tile_size_px) * vec2(0.5, 1.0)
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
        let margin = vec2(10.0, 10.0);
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
                let pos = vec2(x as f32, draw_y) - start - margin;
                self.draw_tile(tile, pos);
            }
        }
    }

    fn handle_input(&mut self) {
        // Check UI state.
        // TODO Fade smoothly.
        let keys = get_keys_pressed();
        if !keys.is_empty() {
            self.ui_hidden = true;
        }
        let mouse = Vec2::from_array(mouse_position().into());
        if mouse != self.mouse {
            self.mouse = mouse;
            self.ui_hidden = false;
        }
        // Full screen toggle.
        if is_key_pressed(KeyCode::F11)
            || (is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt))
                && is_key_pressed(KeyCode::Enter)
        {
            self.fullscreen = !self.fullscreen;
            set_fullscreen(self.fullscreen);
        }
        // Move.
        self.input.up |= is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W);
        self.input.down |= is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S);
        self.input.left |= is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A);
        self.input.right |= is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D);
        if self.input.up {
            self.maybe_move_by(vec2(0.0, -1.0));
        }
        if self.input.down {
            self.maybe_move_by(vec2(0.0, 1.0));
        }
        if self.input.left {
            self.maybe_move_by(vec2(-1.0, 0.0));
        }
        if self.input.right {
            self.maybe_move_by(vec2(1.0, 0.0));
        }
        // Reset input for next frame.
        self.input = Default::default();
    }

    fn load(&mut self) {
        self.assets = Some(Assets::load(&self.game_metrics));
        let Self { world, .. } = self;
        self.facing_x = 1.0;
        self.pos = Vec2::floor(world.grid.size() * 0.5);
    }

    fn maybe_move_by(&mut self, vec: Vec2) {
        let next = self.pos + vec;
        if vec.x != 0.0 {
            self.facing_x = vec.x;
        }
        if !self.world.occupied(next) {
            self.pos = next;
        }
    }

    fn ui(&mut self) {
        let Self { screen_metrics, .. } = self;
        let icon_size = screen_metrics.icon_size;
        let screen_size = Vec2::from_array(screen_size().into());
        // draw_text_ex(
        //     "\u{e801}",
        //     10.0,
        //     50.0,
        //     TextParams {
        //         font_size: 50,
        //         font: Some(&assets.icons),
        //         ..Default::default()
        //     },
        // );
        root_ui().push_skin(self.icon_skin.as_ref().unwrap());
        let gap = vec2(7.0, 5.0) * screen_metrics.scale;
        let step_y = vec2(0.0, icon_size + 2.0 * gap.y);
        // Up/Down
        // TODO Force a common button size?
        let pos = vec2(0.0, screen_metrics.ui_start.y) + gap;
        if root_ui().button(pos, "\u{e803}") {
            self.input.up = true;
        }
        if root_ui().button(pos + step_y, "\u{e800}") {
            self.input.down = true;
        }
        // Left/Right
        let gap = vec2(4.0, 5.0) * screen_metrics.scale;
        let pos = vec2(screen_size.x, screen_metrics.ui_start.y) + vec2(-gap.x - icon_size, gap.y);
        if root_ui().button(pos, "\u{e801}") {
            self.input.left = true;
        }
        if root_ui().button(pos + step_y, "\u{e802}") {
            self.input.right = true;
        }
    }

    fn update_screen(&mut self) {
        let screen_size = Vec2::from_array(screen_size().into());
        let Self { game_metrics, .. } = self;
        let scale = screen_size / game_metrics.ui_size_px;
        let scale = Vec2::splat(scale.x.min(scale.y));
        let scale_changed = self.screen_metrics.scale != scale;
        let ui_size = scale * game_metrics.ui_size_px;
        let ui_start = Vec2::floor((screen_size - ui_size) * 0.5);
        let full_size = scale * game_metrics.full_size_px;
        // dbg!(screen_size);
        // dbg!(scale);
        // dbg!(game_metrics.full_size_px);
        // dbg!(full_size);
        let full_start = vec2(((screen_size - full_size).x * 0.5).floor(), ui_start.y);
        let ground_start = full_start + scale * game_metrics.ground_start_px;
        let ground_size = scale * game_metrics.ground_size_px;
        let icon_size = scale.y * 20.0;
        let sky_size = scale * game_metrics.sky_size_px;
        let sky_start = full_start;
        let tile_size = scale * game_metrics.tile_size_px;
        self.screen_metrics = ScreenMetrics {
            full_size,
            full_start,
            ground_size,
            ground_start,
            icon_size,
            scale,
            sky_size,
            sky_start,
            tile_size,
            ui_size,
            ui_start,
        };
        if scale_changed || self.icon_skin.is_none() {
            self.update_skin();
        }
    }

    fn update_skin(&mut self) {
        let button_style = root_ui()
            .style_builder()
            .with_font(&self.assets.as_ref().unwrap().icons)
            .unwrap()
            .color(BLANK)
            .color_clicked(BLANK)
            .color_hovered(BLANK)
            .font_size(self.screen_metrics.icon_size as u16)
            .text_color(WHITE)
            .text_color_clicked(Color::from_rgba(0xC0, 0xC0, 0xC0, 255))
            .text_color_hovered(WHITE)
            .build();
        self.icon_skin = Some(Skin {
            button_style,
            ..root_ui().default_skin()
        });
    }
}
