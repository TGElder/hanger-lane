extern crate rand;

mod version;
mod simulation;
mod graphics;
mod editor;
pub mod ui;

#[derive(PartialEq)]
enum Direction {
    North,
    South,
    East,
    West
}


#[derive(Clone, Debug)]
struct Cell {
    x: u32,
    y: u32,
    exits: [bool; 4],
}

#[derive(Clone, Debug)]
pub struct City {
    id: usize,
    width: u32,
    height: u32,
    cells: Vec<Vec<Cell>>,
}

impl City {
    fn new(width: u32, height: u32) -> City {
        City{ id: 0, width, height,
            cells: (0..width).map(move |x| {
                (0..height).map(move |y| Cell { x, y, exits: [false; 4] }).collect()
            }).collect()
        }

    }
    fn forward(&self, x: u32, y: u32, d: Direction) -> Option<(u32, u32, Direction)> {
        match d {
            Direction::North if y > 0 => Some((x, y - 1, d)),
            Direction::South if y < self.height - 1 => Some((x, y + 1, d)),
            Direction::West if x > 0 => Some((x - 1, y, d)),
            Direction::East if x < self.width - 1 => Some((x + 1, y, d)),
            _ => None,
        }
    }

    fn from(_: &str) -> City {
        City::new(1024, 1024)
    }

    //fn get_index(&self, x: u32, y: u32, d: u32) -> u32 {
    //    (y * self.width * 4) + (x * 4) + d;
    //}

    //fn get_nodes(&self) -> u32 {
    //    self.width * self.height * 4
    //}

    //fn create_edges(&self) -> Vec<Edge> {

    //}
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

#[cfg(test)]
mod tests {

    use {City, Direction};

    #[test]
    fn test_forward() {
        let city = City::new(3, 3);

        assert!(city.forward(0, 1, Direction::North) == Some((0, 0, Direction::North)));
        assert!(city.forward(0, 0, Direction::North) == None);
        assert!(city.forward(0, 1, Direction::South) == Some((0, 2, Direction::South)));
        assert!(city.forward(0, 2, Direction::South) == None);
        assert!(city.forward(1, 0, Direction::West) == Some((0, 0, Direction::West)));
        assert!(city.forward(0, 0, Direction::West) == None);
        assert!(city.forward(1, 0, Direction::East) == Some((2, 0, Direction::East)));
        assert!(city.forward(2, 0, Direction::East) == None);

    }
}
