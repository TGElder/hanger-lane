use city::{City, Road};
use {Cell, Direction, DIRECTIONS, get_opposite};

pub fn create_city(text: &str) -> City {
    let width = text.split("\n").nth(0).unwrap().split(",").count();
    let height = text.split("\n").count();
    let mut city = City::new(width, height, vec![], vec![]);
    for transaction in parse_map(text) {
        city = apply(transaction, city);
    }
    city
}

fn parse_map(text: &str) -> Vec<Transaction> {
    text.split("\n").enumerate().flat_map(|(y, t)| parse_row(y, t)).collect()
}

fn parse_row(y: usize, text: &str) -> Vec<Transaction> {
    text.split(",").enumerate().flat_map(|(x, t)| parse_cell(x, y, t)).collect()
}

fn parse_cell(x: usize, y: usize, text: &str) -> Vec<Transaction> {
    text.split(" ").flat_map(|t| parse_symbol(x, y, t)).collect()
}

fn parse_symbol(x: usize, y: usize, text: &str) -> Vec<Transaction> {
    let chars: Vec<char> = text.chars().collect();
    match chars.len() {
        0 => vec![],
        2 => match (chars[0], chars[1]) {
            (entry, exit)
                if is_valid_destination_symbol(&entry) && is_valid_destination_symbol(&exit)
                    => parse_road(x, y, entry, exit),
            ('S', direction) => vec![parse_source(x, y, direction)],
            ('D', direction) => vec![parse_destination(x, y, direction)],
            (_, _) => panic!("Unknown symbol {}", text),
        },
        _ => panic!("Unknown symbol {}", text),
    }
}

fn is_valid_destination_symbol(symbol: &char) -> bool {
    const VALID: [char; 5] = ['^', '>', 'v', '<', '*'];
    VALID.contains(symbol)
}

fn parse_road(x: usize, y: usize, entry_symbol: char, exit_symbol: char) -> Vec<Transaction> {
    let entries = match entry_symbol {
        '*' => DIRECTIONS.to_vec(),
        c => vec![get_direction(c)],
    };
    let exits = match exit_symbol {
        '*' => DIRECTIONS.to_vec(),
        c => vec![get_direction(c)],
    };
    let mut out: Vec<Transaction> = vec![];
    for entry in entries.iter() {
        for exit in exits.iter().filter(|d| *entry != get_opposite(*d)) {
            out.push(Transaction::AddRoad(Road::new(x, y, *entry, *exit)));
        }
    }
    out
}

fn parse_source(x: usize, y: usize, direction: char) -> Transaction {
    let direction = get_direction(direction);
    Transaction::AddSource(Cell::new(x, y, direction))
}

fn parse_destination(x: usize, y: usize, direction: char) -> Transaction {
    let direction = get_direction(direction);
    Transaction::AddDestination(Cell::new(x, y, direction))
}

#[derive(PartialEq)]
enum Transaction {
    AddRoad(Road),
    AddSource(Cell),
    AddDestination(Cell),
}

fn apply(transaction: Transaction, mut city: City) -> City {
    match transaction {
        Transaction::AddRoad(road) => city.roads.push(road),
        Transaction::AddSource(cell) => {
            let index = city.get_index(&cell);
            city.sources.push(index);
        },
        Transaction::AddDestination(cell) => {
            let index = city.get_index(&cell);
            city.destinations.push(index);
        },
    }
    city
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
    fn test_add_road() {
        let city = City::new(4, 4, vec![], vec![]);
        let add_road = Transaction::AddRoad(Road::new(1, 3, Direction::East, Direction::South));
        let city = apply(add_road, city);
        assert!(city.roads.len() == 1);
        assert!(city.roads[0] == Road::new(1, 3, Direction::East, Direction::South));
    }

    #[test]
    fn test_add_source() {
        let city = City::new(4, 4, vec![], vec![]);
        let add_source = Transaction::AddSource(Cell{ x: 1, y: 3, d: Direction::South });
        let city = apply(add_source, city);
        assert!(city.sources.len() == 1);
        assert!(city.get_cell(city.sources[0]) == Cell{ x: 1, y: 3, d: Direction::South});
    }

    #[test]
    fn test_add_destination() {
        let city = City::new(4, 4, vec![], vec![]);
        let add_destination = Transaction::AddDestination(Cell{ x: 1, y: 3, d: Direction::South });
        let city = apply(add_destination, city);
        assert!(city.destinations.len() == 1);
        assert!(city.get_cell(city.destinations[0]) == Cell{ x: 1, y: 3, d: Direction::South});
    }

    #[test]
    fn test_parse_road_simple() {
        let transactions = parse_symbol(1, 3, ">v");
        assert!(transactions == vec![Transaction::AddRoad(Road::new(1, 3, Direction::East, Direction::South))]);
    }

    #[test]
    fn test_parse_road_entry_wildcard() {
        let transactions = parse_symbol(1, 3, "*v");
        assert!(transactions.len() == 3);
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::East, Direction::South))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::South, Direction::South))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::West, Direction::South))));
    }

    #[test]
    fn test_parse_road_exit_wildcard() {
        let transactions = parse_symbol(1, 3, "^*");
        assert!(transactions.len() == 3);
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::North, Direction::North))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::North, Direction::East))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::North, Direction::West))));
    }

    #[test]
    fn test_parse_road_double_wildcard() {
        let transactions = parse_symbol(1, 3, "**");
        assert!(transactions.len() == 12);
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::North, Direction::North))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::North, Direction::East))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::North, Direction::West))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::East, Direction::East))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::East, Direction::South))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::East, Direction::North))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::South, Direction::South))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::South, Direction::West))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::South, Direction::East))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::West, Direction::West))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::West, Direction::North))));
        assert!(transactions.contains(&Transaction::AddRoad(Road::new(1, 3, Direction::West, Direction::South))));
    }

    #[test]
    fn test_parse_empty() {
        let transactions = parse_symbol(1, 3, "");
        assert!(transactions == vec![]);
    }

    #[test]
    fn test_parse_source() {
        let transactions = parse_symbol(1, 3, "Sv");
        assert!(transactions == vec![Transaction::AddSource(Cell{ x: 1, y: 3, d: Direction::South })]);
    }

    #[test]
    fn test_parse_destination() {
        let transactions = parse_symbol(1, 3, "Dv");
        assert!(transactions == vec![Transaction::AddDestination(Cell{ x: 1, y: 3, d: Direction::South })]);
    }

    #[test]
    fn test_parse_cell() {
        let transactions = parse_cell(1, 3, "Sv S>");
        assert!(transactions == vec![
                Transaction::AddSource(Cell{ x: 1, y: 3, d: Direction::South }),
                Transaction::AddSource(Cell{ x: 1, y: 3, d: Direction::East })
        ]);
    }

    #[test]
    fn test_parse_row() {
        let transactions = parse_row(1, "Sv,S>");
        assert!(transactions == vec![
                Transaction::AddSource(Cell{ x: 0, y: 1, d: Direction::South }),
                Transaction::AddSource(Cell{ x: 1, y: 1, d: Direction::East })
        ]);
    }

    #[test]
    fn test_parse_map() {
        let transactions = parse_map(",D^\nSv,");
        assert!(transactions == vec![
                Transaction::AddDestination(Cell{ x: 1, y: 0, d: Direction::North }),
                Transaction::AddSource(Cell{ x: 0, y: 1, d: Direction::South })
        ]);
    }

    #[test]
    fn test_create_city() {
        let city = create_city(",D^\nSv,<<");
        assert!(city.width == 2);
        assert!(city.height == 2);
        assert!(city.get_cell(city.sources[0]) == Cell::new(0, 1, Direction::South));
        assert!(city.get_cell(city.destinations[0]) == Cell::new(1, 0, Direction::North));
        assert!(city.roads == vec![Road::new(1, 1, Direction::West, Direction::West)]);
    }

    
}

