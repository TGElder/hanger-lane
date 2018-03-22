use network::Network;
use simulation::{Occupancy, VehicleUpdate};
use Vehicle;
use rand::Rng;

pub struct LookaheadDriver {
    network: Network,
    costs: Vec<Vec<Option<u32>>>,
}

impl LookaheadDriver {

    pub fn new(network: Network, costs: Vec<Vec<Option<u32>>>) -> LookaheadDriver {
        LookaheadDriver{ network, costs }
    }

    fn extend(&self, path: &Vec<usize>, occupancy: &Occupancy) -> Vec<Vec<usize>> {
        let neighbours: Vec<usize> = self.network.get_out(*path.last().unwrap()).iter().map(|n| n.to).collect();
        let free_neighbours: Vec<usize> = neighbours.iter().cloned()
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

    fn extend_all(&self, paths: &Vec<Vec<usize>>, occupancy: &Occupancy) -> Vec<Vec<usize>> {
        let mut paths_out = vec![];
        for path in paths {
            paths_out.append(&mut self.extend(path, occupancy));
        }
        paths_out
    }

}

impl VehicleUpdate for LookaheadDriver {
    fn update(&self, vehicle: &mut Vehicle, occupancy: &mut Occupancy, rng: &mut Box<Rng>) {
        let costs = self.costs.get(vehicle.destination_index).unwrap();
        let node = vehicle.location;
        let mut paths_0 = vec![vec![node]];
        let mut paths_1 = self.extend_all(&paths_0, &occupancy);
        let mut paths_2 = self.extend_all(&paths_1, &occupancy);
        let mut paths_3 = self.extend_all(&paths_2, &occupancy);
        let mut paths = vec![];
        paths.append(&mut paths_0);
        paths.append(&mut paths_1);
        paths.append(&mut paths_2);
        paths.append(&mut paths_3);

        let lowest_cost = paths.iter()
            .map(|p| costs.get(*p.last().unwrap()).unwrap())
            .min();
        if let Some(lowest_cost) = lowest_cost {
            if lowest_cost < costs.get(node).unwrap() {
                // Get some neighbour with lowest cost
                let candidates: Vec<usize> = paths.iter().cloned()
                    .filter(|p| costs.get(*p.last().unwrap()).unwrap() == lowest_cost)
                    .map(|p| *p.get(1).unwrap())
                    .collect();
                vehicle.location = *rng.choose(&candidates).unwrap();
            }
        }
    }
    
}

#[cfg(test)]
mod tests {

    use network::{Edge, Network};

    #[test]
    fn test_a() {
        let edges = Edge::create_grid(2, 4, 1, Edge::create_4_neighbour_deltas());
        let network = Network::new(8, &edges);
        let costs = network.dijkstra(7);

    }

}
