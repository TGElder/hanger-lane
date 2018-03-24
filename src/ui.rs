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

pub struct UI {
}

impl UI {
    
    pub fn launch() {

        const WIDTH: usize = 512;
        const HEIGHT: usize = 512;

        let mut rng: Box<Rng> = Box::new(rand::thread_rng());
        let mut sources = vec![];
        let mut destinations = vec![];

        for _ in 0..64 {
            sources.push(rng.gen_range(0, WIDTH * HEIGHT * 4));
        }

        for _ in 0..64 {
            destinations.push(rng.gen_range(0, WIDTH * HEIGHT * 4));
        }

        let city = City::with_all_roads(WIDTH, HEIGHT, sources, destinations);
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
        costs.push(network.dijkstra(*destination));
    }
    let add_vehicles = Box::new(SpawnVehicles{city: Arc::clone(&city)});
    let vehicle_updates: Vec<Box<VehicleUpdate>> = vec![
        Box::new(VehicleFree{}),
        Box::new(LookaheadDriver::new(network, costs)),
        Box::new(VehicleOccupy{}),
    ];
    let update_vehicles = Box::new(UpdateVehicles{updates: vehicle_updates});
    let remove_vehicles = Box::new(RemoveVehicles{});

    Simulation{ steps: vec![add_vehicles, update_vehicles, remove_vehicles] }
}

pub struct SpawnVehicles {
    city: Arc<City>,
}

impl SimulationStep for SpawnVehicles {
    fn step(&self, state: SimulationState) -> SimulationState {
        let mut traffic = state.traffic;
        let mut rng = state.rng;
        for source in self.city.sources.iter() {
            if state.occupancy.is_free(*source) {
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
        let mut occupancy = state.occupancy;
        let rng = state.rng;
        let mut vehicles_next = vec![];
        for vehicle in traffic.vehicles {
            if vehicle.location != vehicle.destination {
                vehicles_next.push(vehicle.clone());
            }
            else {
                occupancy.free(vehicle.location);
            }
        }
        traffic.vehicles = vehicles_next;
        SimulationState{traffic, occupancy, rng}
    }
}


pub struct VehicleFree {
}

impl VehicleUpdate for VehicleFree {
    fn update(&self, vehicle: &mut Vehicle, occupancy: &mut Occupancy, _rng: &mut Box<Rng>) {
        let start = 4 * (vehicle.location / 4);

        for offset in 0..4 {
            occupancy.free(start + offset);
        }
    }
}

pub struct VehicleOccupy {
}

impl VehicleUpdate for VehicleOccupy {
    fn update(&self, vehicle: &mut Vehicle, occupancy: &mut Occupancy, _rng: &mut Box<Rng>) {
        if &vehicle.location != &vehicle.destination {
            let start = 4 * (vehicle.location / 4);

            for offset in 0..4 {
                occupancy.occupy(start + offset);
            }
        }
    }
}

