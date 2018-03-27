use std::fs::File;
use std::io::prelude::*;
use {Cell, Direction, DIRECTIONS};

#[derive(Clone, Debug, PartialEq)]
pub struct Road {
    x: usize,
    y: usize,
    entry: Direction,
    exit: Direction,
}

impl Road {

    pub fn new(x: usize, y: usize, entry: Direction, exit: Direction) -> Road {
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
    width: usize,
    height: usize,
    roads: Vec<Road>,
    pub sources: Vec<usize>,
    pub destinations: Vec<usize>,
}

use network::Edge;

impl City {
    pub fn new(width: usize, height: usize, sources: Vec<usize>, destinations: Vec<usize>) -> City {
        City{ id: 0, width, height, roads: vec![], sources, destinations }
    }

    pub fn with_all_roads(width: usize, height: usize, sources: Vec<usize>, destinations: Vec<usize>) -> City {

        let mut roads = Vec::with_capacity((width * height * 12));

        for exit in DIRECTIONS.iter() {
            for (_, entry) in DIRECTIONS.iter().enumerate().filter(|&(i, _)| *exit != DIRECTIONS[(i + 2) % 4]) {
                for y in 0..height {
                    for x in 0..width {
                        roads.push(Road::new(x, y, *entry, *exit));
                    }
                }
            }
        }

        City { id: 0, width, height, roads, sources, destinations }
    }

    pub fn from_map_file(width: usize, height: usize, file: String) -> City {
        let mut f = File::open(file).expect("File not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Failed to read file");

        let mut sources = vec![];
        let mut destinations = vec![];
        let mut roads = vec![];
        let mut y = 0;
        for line in contents.split("\n") {
            let mut x = 0;
            for cell in line.split(",") {
                for symbol in cell.chars() {
                    let direction = match symbol {
                        '^' | 'N' | 'n' => Direction::North,
                        '>' | 'E' | 'e' => Direction::East,
                        'v' | 'S' | 's' => Direction::South,
                        '<' | 'W' | 'w' => Direction::West,
                        _ => panic!("Unexpected character [{}]", symbol)
                    };
                    match symbol {
                        '^' | '>' | 'v' | '<' => {
                            for entry in DIRECTIONS.iter() {
                                roads.push(Road::new(x, y, *entry, direction));
                            }
                        },
                        'N' | 'E' | 'S' | 'W' => {
                            sources.push(Cell::new(x, y, direction));
                        },
                        'n' | 'e' | 's' | 'w' => {
                            destinations.push(Cell::new(x, y, direction));
                        },
                        _ => panic!("Unexpected character [{}]", symbol)
                    }
                }
                x += 1;
            }
            y += 1;
        }

        let mut out = City{ id: 0, width, height, roads, sources: vec![], destinations: vec![]};

        for source in sources {
            let index = out.get_index(&source);
            out.sources.push(index);
        }
        for destination in destinations {
            let index = out.get_index(&destination);
            out.destinations.push(index);
        }

        out 
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

    pub fn get_index(&self, &Cell{ref x, ref y, ref d}: &Cell) -> usize {

        fn get_direction_index(d: &Direction) -> usize {
            match d {
                &Direction::North => 0,
                &Direction::East => 1,
                &Direction::South => 2,
                &Direction::West => 3,
            }
        }

        get_direction_index(d) + (x * 4) + (y * 4 * self.width) 
    }

    pub fn get_cell(&self, index: usize) -> Cell {
        let y = index / (4 * self.width);
        let r = index % (4 * self.width);
        let x = r / 4;
        let d = r % 4;
        Cell::new(x, y, DIRECTIONS[d])
    }

    pub fn get_num_nodes(&self) -> usize {
		self.width * self.height * 4
	}

	pub fn create_edges(&self) -> Vec<Edge> {
        let mut out = vec![];
        for road in self.roads.iter() {
            if let Some(forward) = self.forward(&road.get_exit()) {
                out.push(Edge::new(self.get_index(&road.get_start()), self.get_index(&forward), 1));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {

    use {Cell, Road, City, Direction, DIRECTIONS};
    use network::Edge;
    use hamcrest::prelude::*;

    #[test]
    fn test_forward() {
        let city = City::new(3, 3, vec![], vec![]);

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
    fn test_get_index_get_cell() {
        let city = City::new(5, 3, vec![], vec![]);

        let mut cells = Vec::with_capacity((city.width * city.height * 4));

        for y in 0..city.height {
            for x in 0..city.width {
                for d in DIRECTIONS.iter() {
                cells.push( Cell { x, y, d: d.clone() } );
                }
            }
        }

        for (index, cell) in cells.iter().enumerate() {
            assert!(city.get_index(&cell) == index);
            assert!(city.get_cell(index) == *cell);
        }
    }

    #[test]
    fn test_create_edges() {
        let mut city = City::new(3, 3, vec![], vec![]);

        city.roads = vec![
            Road::new(1, 0, Direction::North, Direction::East),
            Road::new(2, 0, Direction::East, Direction::South),
            Road::new(2, 1, Direction::South, Direction::West),
            Road::new(1, 1, Direction::West, Direction::North)
        ];

        let actual = city.create_edges();
        let expected = vec![
            Edge::new(4, 9, 1),
            Edge::new(9, 22, 1),
            Edge::new(22, 19, 1),
            Edge::new(19, 4, 1)
        ];

        assert_that!(&actual.iter().collect(), contains(expected.iter().collect()).exactly());

    }

    #[test]
    fn test_with_all_roads() {
        let city = City::with_all_roads(1, 1, vec![], vec![]);

        let expected = vec![
            Road::new(0, 0, Direction::North, Direction::East),
            Road::new(0, 0, Direction::North, Direction::North),
            Road::new(0, 0, Direction::North, Direction::West),
            Road::new(0, 0, Direction::East, Direction::North),
            Road::new(0, 0, Direction::East, Direction::East),
            Road::new(0, 0, Direction::East, Direction::South),
            Road::new(0, 0, Direction::South, Direction::East),
            Road::new(0, 0, Direction::South, Direction::South),
            Road::new(0, 0, Direction::South, Direction::West),
            Road::new(0, 0, Direction::West, Direction::North),
            Road::new(0, 0, Direction::West, Direction::West),
            Road::new(0, 0, Direction::West, Direction::South),
        ];

        assert_that!(&city.roads.iter().collect(), contains(expected.iter().collect()).exactly());
        
        let city = City::with_all_roads(5, 3, vec![], vec![]);

        assert_that!(city.roads.len(), is(equal_to(180)));
    }

}
