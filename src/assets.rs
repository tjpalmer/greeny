use macroquad::texture::{FilterMode, Texture2D};

pub struct Assets {
    pub mountains: Texture2D,
    pub tiles: Texture2D,
}

impl Assets {
    pub fn load() -> Self {
        Assets {
            mountains: load_texture(MOUNTAINS),
            tiles: load_texture(TILES),
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
