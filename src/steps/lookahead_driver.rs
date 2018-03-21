use std::sync::Arc;
use City;
use network::Network;
use simulation::{Occupancy, VehicleUpdate};
use Vehicle;
use rand::Rng;

pub struct LookaheadDriver {
    city: Arc<City>,
    network: Network,
    costs: Vec<Vec<Option<u32>>>,
}

impl LookaheadDriver {

    pub fn new(city: Arc<City>, network: Network, costs: Vec<Vec<Option<u32>>>) -> LookaheadDriver {
        LookaheadDriver{ city, network, costs }
    }

    fn extend(&self, path: &Vec<usize>, occupancy: &Occupancy) -> Vec<Vec<usize>> {
        let neighbours: Vec<usize> = self.network.get_out(*path.last().unwrap()).iter().map(|n| n.to).collect();
        let free_neighbours: Vec<usize> = neighbours.iter().cloned()
            .filter(|n| {
                let cell = self.city.get_cell(*n);
                occupancy.is_free(cell.x, cell.y) && !path.contains(n)
            }).collect();
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
        let costs = self.costs.get(vehicle.destination).unwrap();
        let node = self.city.get_index(&vehicle.location);
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
            .map(|p| costs.get(*p.last().unwrap()))
            .min();
        if let Some(lowest_cost) = lowest_cost {
            if lowest_cost < costs.get(node) {
                // Get some neighbour with lowest cost
                let candidates: Vec<usize> = paths.iter().cloned()
                    .filter(|p| costs.get(*p.last().unwrap()) == lowest_cost)
                    .map(|p| *p.get(1).unwrap())
                    .collect();
                let selected = rng.choose(&candidates).unwrap();

                vehicle.location = self.city.get_cell(*selected);
            }
        }
    }
    
}

#[cfg(tests)]
mod tests {

    #[test]
    fn test_a() {
        assert!(true);
    }

}
