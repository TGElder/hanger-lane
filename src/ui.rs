extern crate rand; 
extern crate network;

use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use version::Publisher;
use simulation::Simulator;
use super::{Cell, City, Vehicle, Traffic, DIRECTIONS};
use graphics::Graphics;
use editor::Editor;
use rand::{Rng, ThreadRng};
use network::Network;
use simulation::*;

pub struct UI {
}

impl UI {
    
    pub fn launch() {

        let mut rng = rand::thread_rng();
        let mut sources = vec![];
        let mut destinations = vec![];

        for _ in 0..64 {
            sources.push(get_random_cell(&mut rng));
        }

        for _ in 0..64 {
            destinations.push(get_random_cell(&mut rng));
        }

        let city = City::with_all_roads(512, 512, sources, destinations);
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
    let occupancy = Occupancy::new(&city, &traffic.vehicles);
    SimulationState{ traffic, occupancy, rng: rand::thread_rng() }
}

fn setup_simulation(city: &Arc<City>) -> Simulation {
    let network = Network::new(city.get_num_nodes(), &city.create_edges());
    let mut costs = Vec::with_capacity(city.destinations.len());
    for destination in city.destinations.iter() {
        costs.push(network.dijkstra(city.get_index(&destination)));
    }
    let add_vehicles = Box::new(SpawnVehicles{city: Arc::clone(&city)});
    let vehicle_updates: Vec<Box<VehicleUpdate>> = vec![
        Box::new(VehicleFree{}),
        Box::new(MoveVehicle{
            city: Arc::clone(city),
            network,
            costs}),
        Box::new(VehicleOccupy{city: Arc::clone(city)}),
    ];
    let update_vehicles = Box::new(UpdateVehicles{updates: vehicle_updates});
    let remove_vehicles = Box::new(RemoveVehicles{city: Arc::clone(city)});

    Simulation{ steps: vec![add_vehicles, update_vehicles, remove_vehicles] }
}

fn get_random_cell(rng: &mut ThreadRng) -> Cell {
    Cell {
        x: rng.gen_range(0, 512),
        y: rng.gen_range(0, 512),
        d: DIRECTIONS[rng.gen_range(0, 4)],
    }
}

pub struct SpawnVehicles {
    city: Arc<City>,
}

impl SimulationStep for SpawnVehicles {
    fn step(&self, state: SimulationState) -> SimulationState {
        let mut traffic = state.traffic;
        let mut rng = state.rng;
        for source in self.city.sources.iter() {
            if state.occupancy.is_free(source.x, source.y) {
                traffic.vehicles.push(Vehicle{ location: source.clone(), destination: rng.gen_range(0, self.city.destinations.len()) });
            }
        }
        SimulationState{traffic, rng, ..state}
    }
}

pub struct RemoveVehicles {
    city: Arc<City>,
}

impl SimulationStep for RemoveVehicles {
    fn step(&self, state: SimulationState) -> SimulationState {
        let mut traffic = state.traffic;
        let mut occupancy = state.occupancy;
        let mut rng = state.rng;
        let mut vehicles_next = vec![];
        for vehicle in traffic.vehicles {
            if rng.gen_range(0, 400) != 0 && vehicle.location != *self.city.destinations.get(vehicle.destination).unwrap() {
                vehicles_next.push(vehicle.clone());
            }
            else {
                occupancy.free(&vehicle.location);
            }
        }
        traffic.vehicles = vehicles_next;
        SimulationState{traffic, occupancy, rng}
    }
}

pub struct MoveVehicle {
    city: Arc<City>,
    network: Network,
    costs: Vec<Vec<Option<u32>>>,
}

impl MoveVehicle {

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

impl VehicleUpdate for MoveVehicle {
    fn update(&self, vehicle: &mut Vehicle, occupancy: &mut Occupancy, rng: &mut ThreadRng) {
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

pub struct VehicleFree {
}

impl VehicleUpdate for VehicleFree {
    fn update(&self, vehicle: &mut Vehicle, occupancy: &mut Occupancy, _rng: &mut ThreadRng) {
        occupancy.free(&vehicle.location);
    }
}

pub struct VehicleOccupy {
    city: Arc<City>,
}

impl VehicleUpdate for VehicleOccupy {
    fn update(&self, vehicle: &mut Vehicle, occupancy: &mut Occupancy, _rng: &mut ThreadRng) {
        if &vehicle.location != self.city.destinations.get(vehicle.destination).unwrap() {
            occupancy.occupy(&vehicle.location);
        }
    }
}

