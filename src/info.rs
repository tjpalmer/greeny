use macroquad::math::Vec2;

#[allow(unused)]
#[derive(Clone, Copy, Debug)]
pub struct GameMetrics {
    pub full_size: Vec2,
    pub full_size_px: Vec2,
    pub ground_center: Vec2,
    pub ground_center_full_px: Vec2,
    pub ground_size: Vec2,
    pub ground_size_px: Vec2,
    pub ground_start: Vec2,
    pub ground_start_px: Vec2,
    pub sky_size: Vec2,
    pub sky_size_px: Vec2,
    pub tile_size_px: Vec2,
    pub ui_size_px: Vec2,
}

impl GameMetrics {
    pub fn new() -> Self {
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
            ui_size_px: Vec2::new(320.0, 180.0),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ScreenMetrics {
    pub full_size: Vec2,
    pub full_start: Vec2,
    pub ground_size: Vec2,
    pub ground_start: Vec2,
    pub scale: Vec2,
    pub sky_size: Vec2,
    pub sky_start: Vec2,
    pub tile_size: Vec2,
    pub ui_size: Vec2,
}

impl ScreenMetrics {
    pub fn tile(&self, vec: Vec2) -> Vec2 {
        self.ground_start + (vec + Vec2::new(0.0, -1.0)) * self.tile_size
    }
}

impl Default for GameMetrics {
    fn default() -> Self {
        Self::new()
    }
}
