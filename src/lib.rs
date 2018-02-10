mod version;
mod simulation;
pub mod ui;

#[derive(Clone, Debug)]
struct Cell {
    index: usize,
}

#[derive(Clone, Debug)]
pub struct City {
    id: usize,
    cells: Vec<Cell>,
}

impl City {
    fn new(size: usize) -> City {
        City{ id: 0, cells: (0..size).map(|i| Cell{ index: i}).collect() }
    }

    fn from(file: &str) -> City {
        City::new(1048576)
    }
}

#[derive(Clone, Debug)]
struct Vehicle {
    x: u32,
    y: u32,
}

#[derive(Clone, Debug)]
pub struct Traffic {
    id: usize,
    vehicles: Vec<Vehicle>,
}

impl Traffic {
    fn new(size: usize) -> Traffic {
        Traffic{ id: 0, vehicles: (0..size).map(|i| Vehicle{ x: 0, y: 0 }).collect() }
    }
}
