use city::{City, Road};
use {Cell, Direction, DIRECTIONS, get_opposite};

pub fn create_city(text: &str) -> City {
    let width = text.split("\n").nth(0).unwrap().split(",").count();
    let height = text.split("\n").count();
    let mut city = City::new(width, height);
    let transactions = parse_map(text);
    let max_source_group = transactions.iter().filter_map(|t| {
        match t {
            &Transaction::AddSource(group, _) => Some(group),
            _ => None,
        }
    }).max();
    if let Some(group) = max_source_group {
        city.sources = vec![vec![]; group + 1];
    }
    let max_destination_group = transactions.iter().filter_map(|t| {
        match t {
            &Transaction::AddDestination(group, _) => Some(group),
            _ => None,
        }
    }).max();
    if let Some(group) = max_destination_group {
        city.destinations = vec![vec![]; group + 1];
    }
    let max_light_group = transactions.iter().filter_map(|t| {
        match t {
            &Transaction::AddTrafficLight(group, _) => Some(group),
            _ => None,
        }
    }).max();
    if let Some(group) = max_light_group {
        city.lights = vec![vec![]; group + 1];
    }

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
        _ => match (chars[0], chars[1]) {
            (entry, exit)
                if is_valid_destination_symbol(&entry) && is_valid_destination_symbol(&exit)
                    => parse_road(x, y, entry, exit),
            ('S', direction) => vec![parse_source(x, y, direction, text[2..].parse::<usize>().unwrap())],
            ('D', direction) => vec![parse_destination(x, y, direction, text[2..].parse::<usize>().unwrap())],
            ('T', direction) => vec![parse_traffic_light(x, y, direction, text[2..].parse::<usize>().unwrap())],
            (_, _) => panic!("Unknown symbol {}", text),
        },
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

fn parse_source(x: usize, y: usize, direction: char, group: usize) -> Transaction {
    let direction = get_direction(direction);
    Transaction::AddSource(group, Cell::new(x, y, direction))
}

fn parse_destination(x: usize, y: usize, direction: char, group: usize) -> Transaction {
    let direction = get_direction(direction);
    Transaction::AddDestination(group, Cell::new(x, y, direction))
}

fn parse_traffic_light(x: usize, y: usize, direction: char, group: usize) -> Transaction {
    let direction = get_direction(direction);
    Transaction::AddTrafficLight(group, Cell::new(x, y, direction))
}

#[derive(PartialEq)]
enum Transaction {
    AddRoad(Road),
    AddSource(usize, Cell),
    AddDestination(usize, Cell),
    AddTrafficLight(usize, Cell),
}

fn apply(transaction: Transaction, mut city: City) -> City {
    match transaction {
        Transaction::AddRoad(road) => city.roads.push(road),
        Transaction::AddSource(group, cell) => {
            let index = city.get_index(&cell);
            city.sources[group].push(index);
        },
        Transaction::AddDestination(group, cell) => {
            let index = city.get_index(&cell);
            city.destinations[group].push(index);
        },
        Transaction::AddTrafficLight(group, cell) => {
            let index = city.get_index(&cell);
            city.lights[group].push(index);
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
        let city = City::new(4, 4);
        let add_road = Transaction::AddRoad(Road::new(1, 3, Direction::East, Direction::South));
        let city = apply(add_road, city);
        assert!(city.roads.len() == 1);
        assert!(city.roads[0] == Road::new(1, 3, Direction::East, Direction::South));
    }

    #[test]
    fn test_add_source() {
        let mut city = City::new(4, 4);
        city.sources.push(vec![]);
        city.sources.push(vec![]);
        let add_source = Transaction::AddSource(1, Cell{ x: 1, y: 3, d: Direction::South });
        let city = apply(add_source, city);
        assert!(city.sources[1].len() == 1);
        assert!(city.get_cell(city.sources[1][0]) == Cell{ x: 1, y: 3, d: Direction::South});
    }

    #[test]
    fn test_add_destination() {
        let mut city = City::new(4, 4);
        city.destinations.push(vec![]);
        city.destinations.push(vec![]);
        let add_destination = Transaction::AddDestination(1, Cell{ x: 1, y: 3, d: Direction::South });
        let city = apply(add_destination, city);
        assert!(city.destinations[1].len() == 1);
        assert!(city.get_cell(city.destinations[1][0]) == Cell{ x: 1, y: 3, d: Direction::South});
    }

    #[test]
    fn test_add_traffic_light() {
        let mut city = City::new(4, 4);
        city.lights.push(vec![]);
        city.lights.push(vec![]);
        let add_traffic_light = Transaction::AddTrafficLight(1, Cell{ x: 1, y: 3, d: Direction::South });
        let city = apply(add_traffic_light, city);
        assert!(city.lights[1].len() == 1);
        assert!(city.get_cell(city.lights[1][0]) == Cell{ x: 1, y: 3, d: Direction::South});
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
        let transactions = parse_symbol(1, 3, "Sv123");
        assert!(transactions == vec![Transaction::AddSource(123, Cell{ x: 1, y: 3, d: Direction::South })]);
    }

    #[test]
    fn test_parse_destination() {
        let transactions = parse_symbol(1, 3, "Dv7");
        assert!(transactions == vec![Transaction::AddDestination(7, Cell{ x: 1, y: 3, d: Direction::South })]);
    }

    #[test]
    fn test_parse_traffic_light() {
        let transactions = parse_symbol(1, 3, "Tv7");
        assert!(transactions == vec![Transaction::AddTrafficLight(7, Cell{ x: 1, y: 3, d: Direction::South })]);
    }

    #[test]
    fn test_parse_cell() {
        let transactions = parse_cell(1, 3, "Sv6 S>3");
        assert!(transactions == vec![
                Transaction::AddSource(6, Cell{ x: 1, y: 3, d: Direction::South }),
                Transaction::AddSource(3, Cell{ x: 1, y: 3, d: Direction::East })
        ]);
    }

    #[test]
    fn test_parse_row() {
        let transactions = parse_row(1, "Sv4,S>88");
        assert!(transactions == vec![
                Transaction::AddSource(4, Cell{ x: 0, y: 1, d: Direction::South }),
                Transaction::AddSource(88, Cell{ x: 1, y: 1, d: Direction::East })
        ]);
    }

    #[test]
    fn test_parse_map() {
        let transactions = parse_map(",D^1\nSv101,");
        assert!(transactions == vec![
                Transaction::AddDestination(1, Cell{ x: 1, y: 0, d: Direction::North }),
                Transaction::AddSource(101, Cell{ x: 0, y: 1, d: Direction::South })
        ]);
    }

    #[test]
    fn test_create_city() {
        let city = create_city(",D^13 T<3\nSv6,<<");
        assert!(city.width == 2);
        assert!(city.height == 2);
        assert!(city.get_cell(city.sources[6][0]) == Cell::new(0, 1, Direction::South));
        assert!(city.get_cell(city.destinations[13][0]) == Cell::new(1, 0, Direction::North));
        assert!(city.get_cell(city.lights[3][0]) == Cell::new(1, 0, Direction::West));
        assert!(city.roads == vec![Road::new(1, 1, Direction::West, Direction::West)]);
    }

    #[test]
    fn multiple_sources_same_group() {
        let city = create_city(",S^0\nSv0,<<");
        assert!(city.sources[0].len() == 2);
        assert!(city.get_cell(city.sources[0][0]) == Cell::new(1, 0, Direction::North));
        assert!(city.get_cell(city.sources[0][1]) == Cell::new(0, 1, Direction::South));
    }

    #[test]
    fn multiple_destinations_same_group() {
        let city = create_city(",D^0\nDv0,<<");
        assert!(city.destinations[0].len() == 2);
        assert!(city.get_cell(city.destinations[0][0]) == Cell::new(1, 0, Direction::North));
        assert!(city.get_cell(city.destinations[0][1]) == Cell::new(0, 1, Direction::South));
    }

    
}

