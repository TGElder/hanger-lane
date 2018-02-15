extern crate rand;

mod version;
mod simulation;
mod graphics;
mod editor;
pub mod ui;

#[derive(Clone, Debug)]
struct Cell {
    index: u32,
}

#[derive(Clone, Debug)]
pub struct City {
    id: usize,
    cells: Vec<Cell>,
}

impl City {
    fn new(size: u32) -> City {
        City{ id: 0, cells: (0..size).map(|i| Cell{ index: i}).collect() }
    }

    fn from(_: &str) -> City {
        City::new(1048576)
    }
}

#[derive(Clone, Debug)]
struct Vehicle {
    x: u16,
    y: u16,
    vx: i8,
    vy: i8,
}

#[derive(Clone, Debug)]
pub struct Traffic {
    id: usize,
    vehicles: Vec<Vehicle>,
}

impl Traffic {
    fn new(size: usize) -> Traffic {
        Traffic{
            id: 0,
            vehicles: (0..size).map(|_| Vehicle{
                x: rand::random::<u16>(),
                y: rand::random::<u16>(),
                vx: rand::random::<i8>()/32,
                vy: rand::random::<i8>()/32,
            }).collect() }
    }
}
