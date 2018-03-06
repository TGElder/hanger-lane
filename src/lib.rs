extern crate rand;

mod version;
mod simulation;
mod graphics;
mod editor;
pub mod ui;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug)]
pub struct City {
    id: usize,
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl City {
    fn new(width: u32, height: u32) -> City {

        let directions = [Direction::North, Direction::East, Direction::South, Direction::West];

        let mut cells = Vec::with_capacity((width * height * 4) as usize);

        for d in directions.iter() {
            for y in 0..height {
                for x in 0..width {
                    cells.push( Cell { x, y, d: d.clone() } );
                }
            }
        }

        City{ id: 0, width, height, cells }
    }

    fn forward(&self, &Cell{ref x, ref y, ref d}: &Cell) -> Option<&Cell> {
        match d {
            &Direction::North if *y > 0 => Some(self.get_index(x, &(y - 1), d)),
            &Direction::South if *y < self.height - 1 => Some(self.get_index(x, &(y + 1), d)),
            &Direction::West if *x > 0 => Some(self.get_index(&(x - 1), y, d)),
            &Direction::East if *x < self.width - 1 => Some(self.get_index(&(x + 1), y, d)),
            _ => None,
        }.map(|s| self.cells.get(s).unwrap())
    }

    fn from(_: &str) -> City {
        City::new(1024, 1024)
    }

    fn get_index(&self, x: &u32, y: &u32, d: &Direction) -> usize {

        fn get_direction_index(d: &Direction) -> u32 {
            match d {
                &Direction::North => 0,
                &Direction::East => 1,
                &Direction::South => 2,
                &Direction::West => 3,
            }
        }

        (x + (y * self.width) + (get_direction_index(d) * self.width * self.height)) as usize
    }

    //fn get_nodes(&self) -> u32 { self.width * self.height * 4 } fn create_edges(&self) -> Vec<Edge> {

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

    use {Cell, City, Direction};

    #[test]
    fn test_forward() {
        let city = City::new(3, 3);

        assert!(city.forward(&city.cells.get(city.get_index(&0, &1, &Direction::North)).unwrap())
                == Some(&city.cells.get(city.get_index(&0, &0, &Direction::North)).unwrap()));
        //assert!(city.forward(&Cell{x: 0, y: 1, d: Direction::North}) == Some(Cell{x: 0, y: 0, d: Direction::North}));
    //    assert!(city.forward(&Cell{x: 0, y: 0, d: Direction::North}) == None);
    //    assert!(city.forward(&Cell{x: 0, y: 1, d: Direction::South}) == Some(Cell{x: 0, y: 2, d: Direction::South}));
    //    assert!(city.forward(&Cell{x: 0, y: 2, d: Direction::South}) == None);
    //    assert!(city.forward(&Cell{x: 1, y: 0, d: Direction::West}) == Some(Cell{x: 0, y: 0, d: Direction::West}));
    //    assert!(city.forward(&Cell{x: 0, y: 0, d: Direction::West}) == None);
    //    assert!(city.forward(&Cell{x: 1, y: 0, d: Direction::East}) == Some(Cell{x: 2, y: 0, d: Direction::East}));
    //    assert!(city.forward(&Cell{x: 2, y: 0, d: Direction::East}) == None);
    }

    #[test]
    fn test_get_index() {
        let city = City::new(5, 3);

        for (index, cell) in city.cells.iter().enumerate() {
            assert!(city.get_index(&cell.x, &cell.y, &cell.d) == index);
        }
    }
}
