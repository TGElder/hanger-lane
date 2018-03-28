use city::City;
use {Cell, Direction};


fn parse_map(city: City, text: String) -> City {
    city
}


fn parse_row(city: City, y: usize, text: String) -> City {
    city
}

fn parse_cell(city: City, x: usize, y: usize, text: String) -> City {
    city
}

fn parse_road(city: City, x: usize, y: usize, text: String) -> City {
    city
}

fn parse_source(x: usize, y: usize, text: String) -> Box<Manipulation> {
    let chars: Vec<char> = text.chars().collect();
    let direction = get_direction(chars[1]);
    Box::new(AddSource{ cell: Cell::new(x, y, direction) })
}

fn parse_destination(x: usize, y: usize, text: String) -> Box<Manipulation> {
    let chars: Vec<char> = text.chars().collect();
    let direction = get_direction(chars[1]);
    Box::new(AddDestination{ cell: Cell::new(x, y, direction) })
}

trait Manipulation {
    fn manipulate(city: City) -> City;
}

struct AddSource {
    cell: Cell,
}

impl Manipulation for AddSource {
    fn manipulate(self, mut city: City) -> City {
        let index = city.get_index(&self.cell);
        city.sources.push(index);
        city
    }
}

struct AddDestination {
    cell: Cell,
}

impl Manipulation for AddDestination {
    fn manipulate(self, mut city: City) -> City {
        let index = city.get_index(&self.cell);
        city.destinations.push(index);
        city
    }
}

fn get_direction(character: char) -> Direction {
    match character {
        '^' => Direction::North,
        '>' => Direction::East,
        'v' => Direction::South,
        '<' => Direction::West,
        _ => panic!("Was expecting one of ^, >, v, < - got {}", character),
    }
}

#[cfg(test)]
mod tests {

    use Direction;
    use city_map::*;

    #[test]
    fn test_get_direction() {
        assert!(get_direction('^') == Direction::North);
        assert!(get_direction('>') == Direction::East);
        assert!(get_direction('v') == Direction::South);
        assert!(get_direction('<') == Direction::West);
    }

    #[test]
    #[should_panic]
    fn test_invalid_direction() {
        get_direction('#');
    }

    #[test]
    fn test_add_source() {
        let city = City::new(4, 4, vec![], vec![]);
        let manipulation = AddSource{ cell: Cell{ x: 1, y: 3, d: Direction::South }};
        let city = manipulation.manipulate(city);
        assert!(city.sources.len() == 1);
        assert!(city.get_cell(city.sources[0]) == Cell{ x: 1, y: 3, d: Direction::South});
    }

    
}

