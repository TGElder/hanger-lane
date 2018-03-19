extern crate rand;
extern crate network;

use std::sync::{Arc, RwLock};
use version::{Version, Publisher, Local};
use super::{City, Vehicle, Traffic, Cell};
use network::Network;
use rand::{Rng, ThreadRng};

#[derive(Clone)]
pub struct Occupancy {
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

pub trait SimulationStep {
    fn step(&self, state: SimulationState) -> SimulationState;
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
            if rng.gen_range(0, 1000) != 0 && vehicle.location != *self.city.destinations.get(vehicle.destination).unwrap() {
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

pub trait VehicleUpdate {
    fn update(&self, vehicle: &mut Vehicle, occupancy: &mut Occupancy, rng: &mut ThreadRng);
}

pub struct MoveVehicle {
    city: Arc<City>,
    network: Network,
    costs: Vec<Vec<Option<u32>>>,
}

impl VehicleUpdate for MoveVehicle {
    fn update(&self, vehicle: &mut Vehicle, occupancy: &mut Occupancy, rng: &mut ThreadRng) {
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
        if let Some(lowest_cost) = lowest_cost {
            if lowest_cost < costs.get(node) {
                // Get some neighbour with lowest cost
                let candidates: Vec<usize> = free_neighbours.iter().cloned()
                    .filter(|n| costs.get(*n) == lowest_cost)
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

pub struct UpdateVehicles {
    updates: Vec<Box<VehicleUpdate>>,
}


impl SimulationStep for UpdateVehicles {
    fn step(&self, state: SimulationState) -> SimulationState {
        let mut traffic = state.traffic;
        let mut occupancy = state.occupancy;
        let mut rng = state.rng;
        for mut vehicle in traffic.vehicles.iter_mut() {
            for update in self.updates.iter() {
                update.update(&mut vehicle, &mut occupancy, &mut rng);
            }
        }
        SimulationState{ traffic, occupancy, rng}
    }
}

pub struct Simulation {
    steps: Vec<Box<SimulationStep>>,
}

impl Simulation {

    fn new(city: &Arc<City>) -> Simulation {
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

    fn step(&self, state: SimulationState) -> SimulationState {

        let mut state = state;
        for step in self.steps.iter() {
            state = step.step(state);
        }
        state
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
