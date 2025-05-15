use macroquad::prelude::*;

use crate::info::GameMetrics;

pub struct Assets {
    pub mountains: Texture2D,
    pub tile_info: TileInfo,
    pub tiles: Texture2D,
}

impl Assets {
    pub fn load(game_metrics: &GameMetrics) -> Self {
        Assets {
            mountains: load_texture(MOUNTAINS),
            tile_info: TileInfo::new(game_metrics),
            tiles: load_texture(TILES),
        }
    }
}

#[allow(unused)]
pub struct TileInfo {
    pub bead: Vec2,
    pub bobcat: Vec2,
    pub coyote: Vec2,
    pub jack: Vec2,
    pub javelina: Vec2,
    pub nopal_big: Rect,
    pub nopal_small: Rect,
    pub ocotillo: Rect,
    pub rattler: Vec2,
    pub roadie: Vec2,
    pub saguaro: Rect,
    pub turkey: Vec2,
}

impl TileInfo {
    pub fn new(game_metrics: &GameMetrics) -> Self {
        let rect = |x: i32, y: i32, w: i32, h: i32| -> Rect {
            let pos = Vec2::new(x as f32, y as f32) * game_metrics.tile_size_px;
            let size = Vec2::new(w as f32, h as f32) * game_metrics.tile_size_px;
            Rect::new(pos.x, pos.y, size.x, size.y)
        };
        Self {
            bead: Vec2::new(8.0, 9.0) * game_metrics.tile_size_px,
            bobcat: Vec2::new(6.0, 8.0) * game_metrics.tile_size_px,
            coyote: Vec2::new(6.0, 9.0) * game_metrics.tile_size_px,
            jack: Vec2::new(7.0, 8.0) * game_metrics.tile_size_px,
            javelina: Vec2::new(5.0, 9.0) * game_metrics.tile_size_px,
            nopal_big: rect(6, 6, 3, 2),
            nopal_small: rect(5, 6, 1, 1),
            ocotillo: rect(11, 5, 3, 3),
            rattler: Vec2::new(7.0, 9.0) * game_metrics.tile_size_px,
            roadie: Vec2::new(8.0, 8.0) * game_metrics.tile_size_px,
            saguaro: rect(1, 3, 3, 5),
            turkey: Vec2::new(5.0, 8.0) * game_metrics.tile_size_px,
        }
    }
}

pub fn load_texture(bytes: &[u8]) -> Texture2D {
    let texture = Texture2D::from_file_with_format(bytes, None);
    texture.set_filter(FilterMode::Nearest);
    texture
}

pub const MOUNTAINS: &[u8] = include_bytes!("../sprites/mountains.png");
pub const TILES: &[u8] = include_bytes!("../sprites/distinct.png");
