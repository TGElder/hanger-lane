extern crate rand;
extern crate network;
#[cfg(test)] #[macro_use] extern crate hamcrest;

mod version;
mod simulation;
mod graphics;
mod editor;
pub mod ui;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West
}


#[derive(Clone, Debug, PartialEq)]
struct Cell {
    x: u32,
    y: u32,
    d: Direction,
}

impl Cell {

    pub fn new(x: u32, y: u32, d: Direction) -> Cell {
        Cell{x, y, d}
    }

}

#[derive(Clone, Debug)]
struct Road {
    x: u32,
    y: u32,
    entry: Direction,
    exit: Direction,
}

impl Road {

    pub fn new(x: u32, y: u32, entry: Direction, exit: Direction) -> Road {
        Road{x, y, entry, exit}
    }

    pub fn get_start(&self) -> Cell {
        Cell{x: self.x, y: self.y, d: self.entry}
    }

    pub fn get_exit(&self) -> Cell {
        Cell{x: self.x, y: self.y, d: self.exit}
    }

}

#[derive(Clone, Debug)]
pub struct City {
    id: usize,
    width: u32,
    height: u32,
    roads: Vec<Road>,
}

use network::Edge;

impl City {
    fn new(width: u32, height: u32) -> City {
        City{ id: 0, width, height, roads: vec![] }
    }

    fn from(_: &str) -> City {
        City::new(1024, 1024)
    }

    fn forward(&self, &Cell{ref x, ref y, ref d}: &Cell) -> Option<Cell> {

        match *d {
            Direction::North if *y > 0 => Some(Cell::new(*x, *y - 1, *d)),
            Direction::South if *y < self.height - 1 => Some(Cell::new(*x, *y + 1, *d)),
            Direction::West if *x > 0 => Some(Cell::new(*x - 1, *y, *d)),
            Direction::East if *x < self.width - 1 => Some(Cell::new(*x + 1, *y, *d)),
            _ => None,
        }
    }

    fn get_index(&self, &Cell{ref x, ref y, ref d}: &Cell) -> u32 {

        fn get_direction_index(d: &Direction) -> u32 {
            match d {
                &Direction::North => 0,
                &Direction::East => 1,
                &Direction::South => 2,
                &Direction::West => 3,
            }
        }

        x + (y * self.width) + (get_direction_index(d) * self.width * self.height)
    }

    fn get_num_nodes(&self) -> u32 {
		self.width * self.height * 4
	}

	fn create_edges(&self) -> Vec<Edge> {
        let mut out = vec![];
        for road in self.roads.iter() {
            if let Some(forward) = self.forward(&road.get_exit()) {
                out.push(Edge::new(self.get_index(&road.get_start()), self.get_index(&forward), 1));
            }
        }
        out
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

#[cfg(test)]
mod tests {

    use {Cell, Road, City, Direction};
    use network::Edge;
    use hamcrest::prelude::*;

    #[test]
    fn test_forward() {
        let city = City::new(3, 3);

        assert!(city.forward(&Cell{x: 0, y: 1, d: Direction::North}) == Some(Cell{x: 0, y: 0, d: Direction::North}));
        assert!(city.forward(&Cell{x: 0, y: 0, d: Direction::North}) == None);
        assert!(city.forward(&Cell{x: 0, y: 1, d: Direction::South}) == Some(Cell{x: 0, y: 2, d: Direction::South}));
        assert!(city.forward(&Cell{x: 0, y: 2, d: Direction::South}) == None);
        assert!(city.forward(&Cell{x: 1, y: 0, d: Direction::West}) == Some(Cell{x: 0, y: 0, d: Direction::West}));
        assert!(city.forward(&Cell{x: 0, y: 0, d: Direction::West}) == None);
        assert!(city.forward(&Cell{x: 1, y: 0, d: Direction::East}) == Some(Cell{x: 2, y: 0, d: Direction::East}));
        assert!(city.forward(&Cell{x: 2, y: 0, d: Direction::East}) == None);
    }

    #[test]
    fn test_get_index() {
        let city = City::new(5, 3);

		let directions = [Direction::North, Direction::East, Direction::South, Direction::West];

        let mut cells = Vec::with_capacity((city.width * city.height * 4) as usize);

        for d in directions.iter() {
            for y in 0..city.height {
                for x in 0..city.width {
                    cells.push( Cell { x, y, d: d.clone() } );
                }
            }
        }

        for (index, cell) in cells.iter().enumerate() {
            assert!(city.get_index(&cell) == index as u32);
        }
    }

    #[test]
    fn test_create_edges() {
        let mut city = City::new(3, 3);

        city.roads = vec![
            Road::new(1, 0, Direction::North, Direction::East),
            Road::new(2, 0, Direction::East, Direction::South),
            Road::new(2, 1, Direction::South, Direction::West),
            Road::new(1, 1, Direction::West, Direction::North)
        ];

        let actual = city.create_edges();
        let expected = vec![
            Edge::new(31, 1, 1),
            Edge::new(1, 11, 1),
            Edge::new(11, 23, 1),
            Edge::new(23, 31, 1)
        ];

        assert_that!(&actual.iter().collect(), contains(expected.iter().collect()).exactly());

    }
}
