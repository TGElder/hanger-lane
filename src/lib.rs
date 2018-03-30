extern crate rand;
extern crate network;
#[cfg(test)] #[macro_use] extern crate hamcrest;

mod city;
mod city_map;
mod version;
mod simulation;
mod occupancy;
mod graphics;
mod steps;
pub mod ui;

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
    location: usize,
    destination: Vec<usize>,
    destination_index: usize,
}

#[derive(Clone, Debug)]
pub struct Traffic {
    id: usize,
    vehicles: Vec<Vehicle>,
}
