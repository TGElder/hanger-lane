extern crate hanger_lane;
extern crate rand; 
extern crate network;
extern crate clap;

use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc, RwLock};
use std::fs::File;
use std::io::prelude::*;
use std::cell::RefCell;
use hanger_lane::{
    Vehicle, 
    Traffic,
    version::{Version, Publisher},
    simulation::*,
    occupancy::Occupancy,
    city::City,
    graphics::Graphics,
    steps::{
        lookahead_driver::LookaheadDriver,
        block_occupier::{VehicleFree, VehicleOccupy},
        delay::Delay,
        traffic_lights::{Timer, TrafficLights}
    },
    city_map::create_city
};
use network::Network;
use rand::Rng;
use clap::{App, Arg, ArgMatches};

fn main() {
    let args = get_args();

    let mut f = File::open(args.value_of("file").unwrap()).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Failed to read file");
    let city = create_city(&contents);
    let city_version = Arc::new(RwLock::new(None));
    let mut city_publisher = Publisher::new(&city_version);
    city_publisher.publish(&city);

    let traffic_version = Arc::new(RwLock::new(None));

    let mut graphics = Graphics::new(&city_version,
                                     &traffic_version,
                                     "Hanger Lane",
                                     args.value_of("window_width").unwrap().parse().unwrap(),
                                     args.value_of("window_height").unwrap().parse().unwrap(),
                                     args.value_of("grid_size").unwrap().parse().unwrap());

    let run = Arc::new(RwLock::new(true));
    let shutdown = Arc::new(RwLock::new(false));
    let sim_handle = setup_simulator(&run, &shutdown, city, traffic_version, args);

    *run.write().unwrap() = true;

    // Window needs to be created in main thread
    graphics.run();

    *run.write().unwrap() = false;
    *shutdown.write().unwrap() = true;
    sim_handle.join().unwrap();
}

fn get_args() -> ArgMatches<'static> {
    App::new("Hanger Lane Traffic Simulator")
        .version("0.0.1")
        .author("Thomas Elder <tgelder@gmail.com>")
        .arg(Arg::with_name("file")
            .help("Map of city to simulate")
            .required(true)
            .index(1))
        .arg(Arg::with_name("window_width")
             .help("Width of window in pixels")
             .long("window_width")
             .default_value("1024"))
        .arg(Arg::with_name("window_height")
             .help("Height of window in pixels")
             .long("window_height")
             .default_value("1024"))
        .arg(Arg::with_name("grid_size")
             .help("Size in pixels of squares representing vehicles")
             .long("grid_size")
             .default_value("12"))
        .arg(Arg::with_name("spawn_frequency")
             .help("On each step, each source will produce a vehicle with probability 1/spawn_frequency")
             .long("spawn_frequency")
             .default_value("8"))
        .arg(Arg::with_name("lookahead")
             .help("Vehicles will lookahead by this number of cells (to get around obstacles). Higher values reduce performance.")
             .long("lookahead")
             .default_value("3"))
        .arg(Arg::with_name("step_ms")
             .help("Length of each step in milliseconds")
             .long("step_ms")
             .default_value("25"))
        .arg(Arg::with_name("odd_cycle_steps")
             .help("Odd traffic light cycles last this many steps")
             .long("odd_cycle_steps")
             .default_value("8"))
        .arg(Arg::with_name("even_cycle_steps")
             .help("Even traffic light cycles last this many steps")
             .long("even_cycle_steps")
             .default_value("50"))
        .get_matches()
}

fn setup_simulator(run: &Arc<RwLock<bool>>,
                   shutdown: &Arc<RwLock<bool>>,
                   city: City,
                   traffic_version: Version<Traffic>,
                   args: ArgMatches<'static>) -> JoinHandle<()> {
    let run = Arc::clone(&run);
    let shutdown = Arc::clone(&shutdown);
    thread::spawn(move || {
        let mut occupancy = Occupancy::new(city.get_num_nodes());
        let city_arc = Arc::new(city);
        let mut sim = Simulator::new(setup_simulation(&city_arc, &mut occupancy, args), &traffic_version, run, shutdown);
        sim.run(setup_simulation_state(occupancy));
    })
}

fn setup_simulation_state(occupancy: Occupancy) -> SimulationState {
    let traffic = Traffic{ id: 0, vehicles: vec![] };
    SimulationState{ traffic, occupancy, rng: Box::new(rand::thread_rng()) }
}

fn setup_simulation(city: &Arc<City>,
                    occupancy: &mut Occupancy,
                    args: ArgMatches<'static>) -> Simulation {
    let spawn_frequency = args.value_of("spawn_frequency").unwrap().parse().unwrap();
    let lookahead = args.value_of("lookahead").unwrap().parse().unwrap();
    let step_ms = args.value_of("step_ms").unwrap().parse().unwrap();
    let traffic_light_even_cycle_steps = args.value_of("even_cycle_steps").unwrap().parse().unwrap();
    let traffic_light_odd_cycle_steps = args.value_of("odd_cycle_steps").unwrap().parse().unwrap();

    let network = Network::new(city.get_num_nodes(), &city.create_edges());
    let mut costs = Vec::with_capacity(city.destinations.len());
    for destination in city.destinations.iter() {
        costs.push(network.dijkstra(destination.clone()));
    }
    let add_vehicles = Box::new(SpawnVehicles{city: Arc::clone(&city), block_size: 4, frequency: spawn_frequency});
    let vehicle_updates: Vec<Box<VehicleUpdate>> = vec![
        Box::new(VehicleFree::new(4)),
        Box::new(LookaheadDriver::new(lookahead, network, costs)),
        Box::new(VehicleOccupy::new(4)),
    ];
    let update_vehicles = Box::new(UpdateVehicles{updates: vehicle_updates});
    let remove_vehicles = Box::new(RemoveVehicles{});
    let delay = Box::new(Delay::new(step_ms));

    if city.lights.len() > 0 {
        let traffic_lights = Box::new(TrafficLights::new(city.lights.clone(),
            RefCell::new(Box::new(CounterTimer::new(vec![traffic_light_even_cycle_steps, traffic_light_odd_cycle_steps]))),
            occupancy));
        Simulation{ steps: vec![traffic_lights, add_vehicles, update_vehicles, remove_vehicles, delay] }
    }
    else {
        Simulation{ steps: vec![add_vehicles, update_vehicles, remove_vehicles, delay] }
    }
}

pub struct SpawnVehicles {
    city: Arc<City>,
    block_size: usize,
    frequency: usize,
}

impl SimulationStep for SpawnVehicles {
    fn step(&self, state: SimulationState) -> SimulationState {
        let mut traffic = state.traffic;
        let mut occupancy = state.occupancy;
        let mut rng = state.rng;
        for source in self.city.sources.iter() {
            if rng.gen_range(0, self.frequency) == 0 {
                let candidates: Vec<usize> = source.iter()
                   .cloned()
                   .filter(|s| occupancy.is_unlocked(*s))
                   .collect();
                if candidates.len() > 0 {
                    let location = rng.choose(&candidates).unwrap();
                    let destination_index = rng.gen_range(0, self.city.destinations.len());
                    let destination = self.city.destinations.get(destination_index).unwrap().clone();
                    traffic.vehicles.push(Vehicle{ location: *location, destination, destination_index });
                    let start = self.block_size * (*location / self.block_size);
                    for offset in 0..self.block_size {
                        occupancy.lock(start + offset);
                    }
                }
            }
        }
        SimulationState{traffic, occupancy, rng}
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
            if !vehicle.destination.contains(&vehicle.location) {
                vehicles_next.push(vehicle.clone());
            }
        }
        traffic.vehicles = vehicles_next;
        SimulationState{traffic, occupancy, rng}
    }
}

pub struct CounterTimer {
    counter: usize,
    cycle: usize,
    targets: Vec<usize>,
}

impl CounterTimer {
    pub fn new(targets: Vec<usize>) -> CounterTimer {
        CounterTimer{ counter: 0, cycle: 0, targets }
    }
}

impl Timer for CounterTimer {
    fn ready(&mut self) -> bool {
        let out = self.counter >= self.targets[self.cycle];
        self.counter += 1;
        out
    }

    fn reset(&mut self) {
        self.counter = 0;
        self.cycle = (self.cycle + 1) % self.targets.len();
    }
}
