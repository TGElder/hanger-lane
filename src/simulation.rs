extern crate rand;
extern crate network;

use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};
use version::{Version, Publisher, Local};
use super::{City, Traffic, Cell, Direction};
use network::Network;
use rand::{Rng, ThreadRng};

#[derive(Clone)]
pub struct SimulationState {
    traffic: Traffic,
    occupancy: Occupancy,
    rng: ThreadRng,
}

pub struct Simulation {
    target: usize,
    city: Arc<City>,
    network: Network,
    costs: Vec<Option<u32>>,
}

impl Simulation {

    fn new(city: &Arc<City>) -> Simulation {
        let target = city.get_index(&Cell{x: 256, y: 256, d: Direction::East});
        let city = Arc::clone(city);
        let network = Network::new(city.get_num_nodes(), &city.create_edges());
        let costs = network.dijkstra(target);

        Simulation{ target, city, network, costs }
    }

    fn step(&self, state: SimulationState) -> SimulationState {
        let mut traffic = state.traffic;
        traffic.id += 1;
        let mut occupancy = state.occupancy;
        let mut rng = state.rng;

        for vehicle in traffic.vehicles.iter_mut() {
            let node = self.city.get_index(vehicle);
            let neighbours: Vec<usize> = self.network.get_out(node).iter().map(|e| e.to).collect();
            let free_neighbours: Vec<usize> = neighbours.iter().cloned()
                .filter(|n| {
                    let cell = self.city.get_cell(*n);
                    occupancy.is_free(cell.x, cell.y)
                }).collect();
            let lowest_cost = free_neighbours.iter()
                .map(|n| self.costs.get(*n))
                .min();
            if let Some(lowest_cost) = lowest_cost {
                if lowest_cost < self.costs.get(node) {
                    // Get some neighbour with lowest cost
                    let candidates: Vec<usize> = free_neighbours.iter().cloned()
                        .filter(|n| self.costs.get(*n) == lowest_cost)
                        .collect();
                    let selected = rng.choose(&candidates).unwrap();

                    occupancy.free(vehicle);
                        
                    *vehicle = self.city.get_cell(*selected);
                        
                    if *selected != self.target {
                        occupancy.occupy(vehicle);
                    }
                }
            }
        }
        SimulationState{ traffic, occupancy, rng }
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
                let traffic = Traffic::new(65536);
                let occupancy = Occupancy::new(city, &traffic.vehicles);
                let mut state = SimulationState{ traffic, occupancy, rng: rand::thread_rng()  };

                println!("Stepping");
                while *self.running.read().unwrap() {
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

#[derive(Clone)]
struct Occupancy {
    occupancy: Vec<Vec<bool>>,
}

impl Occupancy {

    fn new(city: &City, vehicles: &Vec<Cell>) -> Occupancy {
        let occupancy = vec![vec![false; city.width as usize]; city.height as usize];
        let mut out = Occupancy{ occupancy };
        for vehicle in vehicles.iter() {
            out.occupy(vehicle);
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


