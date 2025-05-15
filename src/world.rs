use macroquad::prelude::*;

pub struct World {
    pub grid: Grid<Tile>,
}

impl World {
    pub fn new() -> Self {
        let mut grid = Grid::new_sized(1500, 1100);
        for x in 0..grid.size_x() {
            for y in 0..grid.size_y() {
                grid.set_at(x, y, Tile::rand());
            }
        }
        Self { grid }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Grid<T> {
    size_x: usize,
    size_y: usize,
    values: Vec<T>,
}

impl<T> Grid<T>
where
    T: Copy + Clone + Default,
{
    pub fn new_sized(size_x: usize, size_y: usize) -> Self {
        Self {
            size_x,
            size_y,
            values: vec![Default::default(); size_x * size_y],
        }
    }

    pub fn at(&self, x: usize, y: usize) -> T {
        self.values[x * self.size_y + y]
    }

    pub fn set_at(&mut self, x: usize, y: usize, value: T) {
        self.values[x * self.size_y + y] = value;
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.size_x() as f32, self.size_y() as f32)
    }

    pub fn size_x(&self) -> usize {
        self.size_x
    }

    pub fn size_y(&self) -> usize {
        self.size_y
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub enum Plant {
    #[default]
    None,
    NopalBig,
    NopalSmall,
    Ocotillo,
    Saguaro,
}

impl Plant {
    pub fn rand() -> Self {
        if rand::gen_range(0.0, 1.0) < 0.8 {
            return Plant::None;
        }
        let plants = [
            Plant::NopalBig,
            Plant::NopalSmall,
            Plant::Ocotillo,
            Plant::Saguaro,
        ];
        plants[(rand::rand() as usize) % plants.len()]
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Tile {
    pub plant: Plant,
}

impl Tile {
    pub fn rand() -> Self {
        Self {
            plant: Plant::rand(),
        }
    }
}
