extern crate rand; 
extern crate network;

use std::thread;
use std::sync::{Arc, RwLock};
use version::Publisher;
use occupancy::Occupancy;
use simulation::Simulator;
use super::{City, Vehicle, Traffic};
use graphics::Graphics;
use network::Network;
use rand::Rng;
use simulation::*;
use steps::lookahead_driver::LookaheadDriver;
use steps::block_occupier::{VehicleFree, VehicleOccupy};
use steps::delay::Delay;

pub struct UI {
}

impl UI {
    
    pub fn launch() {

        const WIDTH: usize = 12;
        const HEIGHT: usize = 12;

        let city = City::from_map_file(WIDTH, HEIGHT, String::from("roundabout.csv"));
        let city_version = Arc::new(RwLock::new(None));

        let mut city_publisher = Publisher::new(&city_version);
        city_publisher.publish(&city);

        let traffic_version = Arc::new(RwLock::new(None));

        let sim_run = Arc::new(RwLock::new(true));
        let sim_shutdown = Arc::new(RwLock::new(false));
        let mut graphics = Graphics::new(&city_version, &traffic_version, "Hanger Lane", 1024, 1024);

        let sim_run_2 = Arc::clone(&sim_run);
        let sim_shutdown_2 = Arc::clone(&sim_shutdown);
        let sim_handle = thread::spawn(move || {
            let city_arc = Arc::new(city);
            let mut sim = Simulator::new(setup_simulation(&city_arc), &traffic_version, sim_run_2, sim_shutdown_2);
            sim.run(setup_simulation_state(&city_arc));
        });

        *sim_run.write().unwrap() = true;

        // Window needs to be created in main thread
        graphics.run();

        *sim_run.write().unwrap() = false;
        *sim_shutdown.write().unwrap() = true;
        sim_handle.join().unwrap();
    }
}

fn setup_simulation_state(city: &Arc<City>) -> SimulationState {
    let traffic = Traffic{ id: 0, vehicles: vec![] };
    let occupancy = Occupancy::new(city.get_num_nodes());
    SimulationState{ traffic, occupancy, rng: Box::new(rand::thread_rng()) }
}

fn setup_simulation(city: &Arc<City>) -> Simulation {
    let network = Network::new(city.get_num_nodes(), &city.create_edges());
    let mut costs = Vec::with_capacity(city.destinations.len());
    for destination in city.destinations.iter() {
        costs.push(network.dijkstra(vec![*destination]));
    }
    let add_vehicles = Box::new(SpawnVehicles{city: Arc::clone(&city)});
    let vehicle_updates: Vec<Box<VehicleUpdate>> = vec![
        Box::new(VehicleFree::new(4)),
        Box::new(LookaheadDriver::new(2, network, costs)),
        Box::new(VehicleOccupy::new(4)),
    ];
    let update_vehicles = Box::new(UpdateVehicles{updates: vehicle_updates});
    let remove_vehicles = Box::new(RemoveVehicles{});
    let delay = Box::new(Delay::new(50));

    Simulation{ steps: vec![add_vehicles, update_vehicles, remove_vehicles, delay] }
}

pub struct SpawnVehicles {
    city: Arc<City>,
}

impl SimulationStep for SpawnVehicles {
    fn step(&self, state: SimulationState) -> SimulationState {
        let mut traffic = state.traffic;
        let mut rng = state.rng;
        for source in self.city.sources.iter() {
            if rng.gen_range(0, 3) == 0 && state.occupancy.is_free(*source) {
                let destination_index = rng.gen_range(0, self.city.destinations.len());
                let destination = self.city.destinations.get(destination_index).unwrap();
                traffic.vehicles.push(Vehicle{ location: *source, destination: *destination, destination_index });
            }
        }
        SimulationState{traffic, rng, ..state}
    }
}

pub struct RemoveVehicles {
}

impl SimulationStep for RemoveVehicles {
    fn step(&self, state: SimulationState) -> SimulationState {
        let mut traffic = state.traffic;
        let occupancy = state.occupancy;
        let rng = state.rng;
        let mut vehicles_next = vec![];
        for vehicle in traffic.vehicles {
            if vehicle.location != vehicle.destination {
                vehicles_next.push(vehicle.clone());
            }
        }
        traffic.vehicles = vehicles_next;
        SimulationState{traffic, occupancy, rng}
    }
}
