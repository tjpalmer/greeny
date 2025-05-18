use crate::info::GameMetrics;
use macroquad::prelude::*;

pub struct World {
    pub animals: Vec<Animal>,
    pub grid: Grid<Tile>,
}

impl World {
    pub fn new() -> Self {
        // Base grid, including plants.
        let mut grid = Grid::new_sized(1500, 1100);
        for x in 0..grid.size_x() {
            for y in 0..grid.size_y() {
                grid.set_at(x, y, Tile::rand());
            }
        }
        // Animals.
        let animals = vec![];
        let mut selfish = Self { animals, grid };
        let metrics = GameMetrics::default();
        let area_size = (metrics.ground_size.x * metrics.ground_size.y) as usize;
        let animal_count = selfish.grid.size_x() * selfish.grid.size_y() / area_size;
        // dbg!(animal_count);
        while selfish.animals.len() < 10_000 {
            let x = rand::rand() as usize % selfish.grid.size_x();
            let y = rand::rand() as usize % selfish.grid.size_y();
            let pos = Vec2::new(x as f32, y as f32);
            if !selfish.occupied(pos) {
                let animal = Animal {
                    kind: AnimalKind::rand(),
                    pos,
                };
                let animal_idx = selfish.animals.len();
                selfish.grid.mut_at(x, y).occupant = Some(Occupant::Animal(animal_idx));
                selfish.animals.push(animal);
            }
        }
        selfish
    }

    pub fn occupied(&self, vec: Vec2) -> bool {
        self.grid
            .at(vec.x as usize, vec.y as usize)
            .occupant
            .is_some()
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

    pub fn mut_at(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.values[x * self.size_y + y]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Occupant {
    Animal(usize),
    Plant(Plant),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AnimalKind {
    Bead,
    Bob,
    /// Coy?
    Coyote,
    Jack,
    /// Havi?
    Javelina,
    Rattler,
    /// Chap?
    Runner,
    /// Tova? (O'odham-based)
    Turkey,
}

impl AnimalKind {
    pub fn rand() -> Self {
        let animals = [
            AnimalKind::Bead,
            AnimalKind::Bob,
            AnimalKind::Coyote,
            AnimalKind::Jack,
            AnimalKind::Javelina,
            AnimalKind::Rattler,
            AnimalKind::Runner,
            AnimalKind::Turkey,
        ];
        animals[rand::rand() as usize % animals.len()]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Animal {
    pub kind: AnimalKind,
    pub pos: Vec2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Plant {
    NopalBig,
    NopalSmall,
    Ocotillo,
    Saguaro,
}

impl Plant {
    pub fn rand() -> Option<Self> {
        let n = rand::gen_range(0.0, 1.0);
        let plant = match () {
            _ if n < 0.8 => return None,
            _ if n < 0.87 => Plant::NopalSmall,
            _ if n < 0.94 => Plant::NopalBig,
            _ if n < 0.98 => Plant::Ocotillo,
            _ => Plant::Saguaro,
        };
        Some(plant)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Tile {
    pub occupant: Option<Occupant>,
}

impl Tile {
    pub fn rand() -> Self {
        Self {
            occupant: Plant::rand().map(|x| Occupant::Plant(x)),
        }
    }
}
