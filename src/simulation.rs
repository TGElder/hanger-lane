extern crate rand;
extern crate network;

use std::sync::mpsc::Receiver;
use std::sync::Arc;
use version::{Version, Publisher, Local};
use super::{City, Traffic, Cell, Direction};
use network::{Network, Edge};
use rand::{Rng, ThreadRng};

pub enum SimulatorMessage {
    Start,
    Pause,
    Shutdown,
}

pub struct Simulator {
    rx: Receiver<SimulatorMessage>,
    city: Local<City>,
    vehicles: usize,
    traffic_publisher: Publisher<Traffic>,
    running: bool,
    shutting_down: bool,
}

impl Simulator {

    pub fn new(rx: Receiver<SimulatorMessage>,
               city: &Version<City>,
               vehicles: usize,
               traffic: &Version<Traffic>) -> Simulator {
        Simulator{
            rx,
            city: Local::new(city),
            vehicles,
            traffic_publisher: Publisher::new(traffic),
            running: false,
            shutting_down: false,
        }
    }


    pub fn run(&mut self) {

        while !self.shutting_down {

            while !self.running {
                println!("Paused");
                self.check_messages();
            }

            self.city.update();

            let city = match self.city.local {
                Some(ref city) => Some(Arc::clone(city)),
                None => None,
            };

            if let Some(ref city) = city {
                let edges = city.create_edges();
                let mut simulation = Simulation::new(city, &edges, &self.vehicles);

                while self.running {
                    simulation.step();
                    self.check_messages();
                    self.traffic_publisher.publish(&simulation.traffic);
                }

            }
            else {
                while self.running {
                    println!("No city to simulate");
                    self.check_messages();
                }
            }
        }

    }

    fn check_messages(&mut self) {
        match self.rx.try_recv() {
            Ok(m) => {
                match m {
                    SimulatorMessage::Start => {
                        println!("Starting simulation");
                        self.running = true;
                    },
                    SimulatorMessage::Pause => {
                        println!("Pausing simulation");
                        self.running = false;
                    },
                    SimulatorMessage::Shutdown => {
                        println!("Shutting down simulation");
                        self.running = false;
                        self.shutting_down = true;
                    },
                }
            },
            _ => (),
        }

    }

}

struct Simulation<'a> {
    rng: ThreadRng,
    traffic: Traffic,
    city: &'a Arc<City>,
    target: u32,
    node_count: u32,
    network: Network<'a>,
    costs: Vec<Option<u32>>,
    occupancy: Occupancy,
}

impl <'a> Simulation<'a> {

    fn new(city: &'a Arc<City>, edges: &'a Vec<Edge>, vehicles: &usize) -> Simulation<'a> {
        let target = city.get_index(&Cell{x: 256, y: 256, d: Direction::East});
        let node_count = city.get_num_nodes();
        let network = Network::new(node_count, edges);
        let costs = network.dijkstra(target);
        let traffic = Traffic::new(*vehicles);
        let mut occupancy = Occupancy::new(city, &traffic.vehicles);
        Simulation {
            rng: rand::thread_rng(),
            traffic,
            city,
            target,
            node_count,
            network,
            costs,
            occupancy
        }
    }

    fn step(&mut self) {
        println!("Simulating traffic with city version {}", self.city.id);
        println!("Clearing occupancy");

        for vehicle in self.traffic.vehicles.iter_mut() {
            let node = self.city.get_index(vehicle);
            let neighbours: Vec<u32> = self.network.get_out(node).iter().map(|e| e.to).collect();
            let free_neighbours: Vec<u32> = neighbours.iter().cloned()
                .filter(|n| {
                    let cell = self.city.get_cell(*n);
                    !self.occupancy.is_free(cell.x as usize, cell.y as usize)
                }).collect();
            let lowest_cost = free_neighbours.iter()
                .map(|n| self.costs.get(*n as usize))
                .min();
            if let Some(lowest_cost) = lowest_cost {
                if lowest_cost < self.costs.get(node as usize) {
                    // Get some neighbour with lowest cost
                    let candidates: Vec<u32> = free_neighbours.iter().cloned()
                        .filter(|n| self.costs.get(*n as usize) == lowest_cost)
                        .collect();
                    let selected = self.rng.choose(&candidates).unwrap();
                    
                    self.occupancy.free(vehicle);
                    let cell = self.city.get_cell(*selected);
                    vehicle.x = cell.x;
                    vehicle.y = cell.y;
                    vehicle.d = cell.d;
                        
                    if *selected != self.target {
                        self.occupancy.occupy(vehicle);
                    }
                }
            }
        }
        self.traffic.id += 1;
    }

}

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
    
    fn is_free(&self, x: usize, y: usize) -> &bool {
        self.occupancy.get(x).unwrap().get(y).unwrap()
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


