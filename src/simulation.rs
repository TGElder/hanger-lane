extern crate rand;
extern crate network;

use std::sync::{Arc, RwLock};
use version::{Version, Publisher, Local};
use super::{City, Vehicle, Traffic, Cell, Direction};
use network::Network;
use rand::{Rng, ThreadRng};

#[derive(Clone)]
struct Occupancy {
    occupancy: Vec<Vec<bool>>,
}

impl Occupancy {

    fn new(city: &City, vehicles: &Vec<Vehicle>) -> Occupancy {
        let occupancy = vec![vec![false; city.width as usize]; city.height as usize];
        let mut out = Occupancy{ occupancy };
        for vehicle in vehicles.iter() {
            out.occupy(&vehicle.location);
        }
        out
    }
    
    fn is_free(&self, x: usize, y: usize) -> bool {
        !self.occupancy.get(x).unwrap().get(y).unwrap()
    }

    fn set(&mut self, vehicle: &Cell, value: bool) {
        *self.occupancy.get_mut(vehicle.x as usize).unwrap().get_mut(vehicle.y as usize).unwrap() = value;
    }

    fn free(&mut self, vehicle: &Cell) {
        self.set(vehicle, false);
    }

    fn occupy(&mut self, vehicle: &Cell) {
        self.set(vehicle, true);
    }

}

#[derive(Clone)]
pub struct SimulationState {
    traffic: Traffic,
    occupancy: Occupancy,
    rng: ThreadRng,
}

pub struct Simulation {
    target: Cell,
    city: Arc<City>,
    network: Network,
    costs: Vec<Vec<Option<u32>>>,
}

impl Simulation {

    fn new(city: &Arc<City>) -> Simulation {
        let target = Cell{x: 256, y: 256, d: Direction::East};
        let city = Arc::clone(city);
        let network = Network::new(city.get_num_nodes(), &city.create_edges());
        let mut costs = Vec::with_capacity(city.destinations.len());
        for destination in city.destinations.iter() {
            costs.push(network.dijkstra(city.get_index(&destination)));
        }

        Simulation{ target, city, network, costs }
    }

    fn step(&self, state: SimulationState) -> SimulationState {
        let mut traffic = state.traffic;
        traffic.id += 1;
        let mut occupancy = state.occupancy;
        let mut rng = state.rng;

        traffic.vehicles.append(&mut self.get_new_vehicles(&occupancy, &mut rng));
        for vehicle in traffic.vehicles.iter_mut() {
            if let Some(next_location) = self.get_next_location(&vehicle, &occupancy, &mut rng) {
                occupancy.free(&vehicle.location);
                occupancy.occupy(&next_location);
                vehicle.location = next_location;
            }
        }
        traffic.vehicles = self.remove_vehicles(&mut traffic.vehicles, &mut occupancy);

        SimulationState{ traffic, occupancy, rng }
    }

    fn get_next_location(&self, vehicle: &Vehicle, occupancy: &Occupancy, rng: &mut ThreadRng) -> Option<Cell> {
        let costs = self.costs.get(vehicle.destination).unwrap();
        let node = self.city.get_index(&vehicle.location);
        let neighbours: Vec<usize> = self.network.get_out(node).iter().map(|e| e.to).collect();
        let free_neighbours: Vec<usize> = neighbours.iter().cloned()
            .filter(|n| {
                let cell = self.city.get_cell(*n);
                occupancy.is_free(cell.x, cell.y)
            }).collect();
        let lowest_cost = free_neighbours.iter()
            .map(|n| costs.get(*n))
            .min();
        match lowest_cost {
            Some(lowest_cost) if lowest_cost < costs.get(node) => {
                // Get some neighbour with lowest cost
                let candidates: Vec<usize> = free_neighbours.iter().cloned()
                    .filter(|n| costs.get(*n) == lowest_cost)
                    .collect();
                let selected = rng.choose(&candidates).unwrap();

                Some(self.city.get_cell(*selected))
            },
            _ => None
        }
    }

    fn get_new_vehicles(&self, occupancy: &Occupancy, rng: &mut ThreadRng) -> Vec<Vehicle> {
        let mut out = Vec::with_capacity(self.city.sources.len());
        for source in self.city.sources.iter() {
            if (occupancy.is_free(source.x, source.y)) {
                out.push(Vehicle{ location: source.clone(), destination: rng.gen_range(0, self.city.destinations.len()) });
            }
        }
        out
    }

    fn remove_vehicles(&self, vehicles: &mut Vec<Vehicle>, occupancy: &mut Occupancy) -> Vec<Vehicle> {
        let mut out = vec![];
        for vehicle in vehicles {
            if vehicle.location != *self.city.destinations.get(vehicle.destination).unwrap() {
                out.push(vehicle.clone());
            }
            else {
                occupancy.free(&vehicle.location);
            }
        }
        out
    }



}


pub struct Simulator {
    city: Local<City>,
    traffic_publisher: Publisher<Traffic>,
    running: Arc<RwLock<bool>>,
    shutting_down: Arc<RwLock<bool>>,
}

impl Simulator {

    pub fn new(city: &Version<City>,
               traffic: &Version<Traffic>,
               running: Arc<RwLock<bool>>,
               shutting_down: Arc<RwLock<bool>>) -> Simulator {
        Simulator{
            city: Local::new(city),
            traffic_publisher: Publisher::new(traffic),
            running,
            shutting_down,
        }
    }


    pub fn run(&mut self) {

        while !*self.shutting_down.read().unwrap() {

            while !*self.running.read().unwrap() {
                println!("Paused");
            }

            println!("Updating city");
            self.city.update();

            if let Some(ref city) = self.city.local {

                println!("Setting up sim");
                let sim = Simulation::new(city);
                let traffic = Traffic{ id: 0, vehicles: vec![] };
                let occupancy = Occupancy::new(city, &traffic.vehicles);
                let mut state = SimulationState{ traffic, occupancy, rng: rand::thread_rng()  };

                while *self.running.read().unwrap() {
                    println!("Stepping");
                    state = sim.step(state);
                    self.traffic_publisher.publish(&state.traffic);
                }

            }
            else {
                while !*self.running.read().unwrap() {
                    println!("No city to simulate");
                }
            }
        }

    }

}
