use network::Network;
use occupancy::Occupancy;
use simulation::VehicleUpdate;
use Vehicle;
use rand::Rng;

pub struct LookaheadDriver {
    lookahead: usize,
    network: Network,
    costs: Vec<Vec<Option<u32>>>,
}

impl LookaheadDriver {

    pub fn new(lookahead: usize, network: Network, costs: Vec<Vec<Option<u32>>>) -> LookaheadDriver {
        LookaheadDriver{ lookahead, network, costs }
    }

    fn extend(&self, path: &Vec<usize>, occupancy: &Occupancy) -> Vec<Vec<usize>> {
        let neighbours: Vec<usize> = self.network.get_out(*path.last().unwrap()).iter().map(|n| n.to).collect();
        let free_neighbours: Vec<usize> = neighbours.into_iter()
            .filter(|n| { occupancy.is_free(*n) && !path.contains(n) })
            .collect();
        let mut out = vec![];
        for neighbour in free_neighbours {
            let mut neighbour_path = path.clone();
            neighbour_path.push(neighbour);
            out.push(neighbour_path);
        }
        out
    }

    fn extend_all(&self, paths: &mut Vec<Vec<usize>>, length_to_extend: usize, occupancy: &Occupancy) -> Vec<Vec<usize>> {
        let mut paths_out = vec![];
        for path in paths.iter() {
            if path.len() == length_to_extend {
                paths_out.append(&mut self.extend(path, occupancy));
            }
        }
        paths_out.append(paths);
        paths_out
    }

}

impl VehicleUpdate for LookaheadDriver {
    fn update(&self, vehicle: &mut Vehicle, occupancy: &mut Occupancy, rng: &mut Box<Rng>) {
        let costs = &self.costs[vehicle.destination_index];
        let node = vehicle.location;
        let mut paths = vec![vec![node]];
        for i in 0..self.lookahead {
            paths = self.extend_all(&mut paths, i + 1, &occupancy);
        }
        let lowest_cost = paths.iter()
            .map(|p| costs[*p.last().unwrap()])
            .min();
        if let Some(lowest_cost) = lowest_cost {
            if lowest_cost < costs[node] {
                let candidates: Vec<Vec<usize>> = paths.into_iter()
                    .filter(|p| costs[*p.last().unwrap()] == lowest_cost)
                    .collect();
                let shortest = candidates.iter()
                    .map(|p| p.len())
                    .min().unwrap();
                let candidates: Vec<usize> = candidates.into_iter()
                    .filter(|p| p.len() == shortest)
                    .map(|p| p[1])
                    .collect();
                vehicle.location = *rng.choose(&candidates).unwrap();
            }
        }
    }
    
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::Vehicle;
    use network::{Edge, Network};
    use simulation::VehicleUpdate;
    use steps::lookahead_driver::LookaheadDriver;
    use occupancy::Occupancy;
    use rand::Rng;

    fn get_test_driver(lookahead: usize, destination: usize) -> LookaheadDriver {
        let edges = Edge::create_grid(4, 4, 1, Edge::create_4_neighbour_deltas());
        let network = Network::new(16, &edges);
        let costs = vec![network.dijkstra(destination)];
        LookaheadDriver::new(lookahead, network, costs)
    }

    fn init(lookahead: usize, vehicle: usize, destination: usize) -> (LookaheadDriver, Vehicle, Occupancy, Box<Rng>) {
        let driver = get_test_driver(lookahead, destination);
        let vehicle = Vehicle{ location: vehicle, destination, destination_index: 0 };
        let occupancy = Occupancy::new(16);
        let rng: Box<Rng> = Box::new(rand::thread_rng());
        (driver, vehicle, occupancy, rng)
    }
    
    #[test]
    fn no_obstructions() {
        let (driver, mut vehicle, mut occupancy, mut rng) = init(3, 1, 13);

        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 5);
    }

    #[test]
    fn lookahead_required_for_obstruction() {
        let (driver, mut vehicle, mut occupancy, mut rng) = init(3, 1, 13);

        occupancy.occupy(4);
        occupancy.occupy(5);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 2);
    }

    #[test]
    fn lookahead_not_enough_for_obstruction() {
        let (driver, mut vehicle, mut occupancy, mut rng) = init(2, 1, 13);

        occupancy.occupy(4);
        occupancy.occupy(5);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 1);
    }

    #[test]
    fn full_lookahead_not_required_for_obstruction() {
        let (driver, mut vehicle, mut occupancy, mut rng) = init(4, 1, 13);

        occupancy.occupy(4);
        occupancy.occupy(5);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 2);
    }
    
    #[test]
    fn no_route() {
        let (driver, mut vehicle, mut occupancy, mut rng) = init(3, 1, 13);

        occupancy.occupy(4);
        occupancy.occupy(5);
        occupancy.occupy(6);
        occupancy.occupy(7);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 1);
    }

    #[test]
    fn two_routes_a() {
        let (driver, mut vehicle, mut occupancy, mut rng) = init(3, 1, 13);

        occupancy.occupy(5);
        occupancy.occupy(6);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 0);
    }

    #[test]
    fn two_routes_b() {
        let (driver, mut vehicle, mut occupancy, mut rng) = init(3, 2, 13);

        occupancy.occupy(5);
        occupancy.occupy(6);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 1 || vehicle.location == 3);
    }

    #[test]
    fn two_routes_c() {
        let (driver, mut vehicle, mut occupancy, mut rng) = init(3, 2, 14);

        occupancy.occupy(5);
        occupancy.occupy(6);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 3);
    }
    
    #[test]
    fn adjacent_to_goal() {
        let (driver, mut vehicle, mut occupancy, mut rng) = init(3, 0, 1);

        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        println!("###{}", vehicle.location);
        assert!(vehicle.location == 1);
    }

    #[test]
    fn on_goal() {
        let (driver, mut vehicle, mut occupancy, mut rng) = init(3, 0, 0);

        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 0);
    }

    #[test]
    fn goal_blocked() {
        let (driver, mut vehicle, mut occupancy, mut rng) = init(3, 0, 1);

        occupancy.occupy(1);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 0);
    }

    #[test]
    fn position_blocked() {
        let (driver, mut vehicle, mut occupancy, mut rng) = init(3, 0, 1);

        occupancy.occupy(0);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 1);
    }

    #[test]
    fn all_the_way() {
        let (driver, mut vehicle, mut occupancy, mut rng) = init(3, 1, 13);

        occupancy.occupy(4);
        occupancy.occupy(5);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 2);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 6);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 10);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 9 || vehicle.location == 14);
        driver.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(vehicle.location == 13);
    }

}
