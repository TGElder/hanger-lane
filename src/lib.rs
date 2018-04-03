extern crate rand;
extern crate network;
#[cfg(test)] #[macro_use] extern crate hamcrest;

pub mod city;
pub mod city_map;
pub mod version;
pub mod simulation;
pub mod occupancy;
pub mod graphics;
pub mod steps;

const DIRECTIONS: [Direction; 4] = [Direction::North, Direction::East, Direction::South, Direction::West];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West
}

fn get_opposite(direction: &Direction) -> Direction {
    match direction {
        &Direction::North => Direction::South,
        &Direction::East => Direction::West,
        &Direction::South => Direction::North,
        &Direction::West => Direction::East,
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    x: usize,
    y: usize,
    d: Direction,
}

impl Cell {

    pub fn new(x: usize, y: usize, d: Direction) -> Cell {
        Cell{x, y, d}
    }

}

#[derive(Clone, Debug)]
pub struct Vehicle {
    pub location: usize,
    pub destination: Vec<usize>,
    pub destination_index: usize,
}

#[derive(Clone, Debug)]
pub struct Traffic {
    pub id: usize,
    pub vehicles: Vec<Vehicle>,
}
