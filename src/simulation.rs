extern crate rand;
extern crate network;

use std::sync::mpsc::Receiver;
use std::sync::Arc;
use version::{Version, Publisher, Local};
use super::{City, Traffic, Cell, Direction};
use network::Network;
use rand::Rng;

pub enum SimulationMessage {
    Start,
    Pause,
    Shutdown,
}

pub struct Simulation {
    rx: Receiver<SimulationMessage>,
    city: Local<City>,
    traffic: Traffic,
    traffic_publisher: Publisher<Traffic>,
    running: bool,
    shutting_down: bool,
}

impl Simulation {

    pub fn new(rx: Receiver<SimulationMessage>,
               city: &Version<City>,
               vehicles: usize,
               traffic: &Version<Traffic>) -> Simulation {
        Simulation{
            rx,
            city: Local::new(city),
            traffic: Traffic::new(vehicles),
            traffic_publisher: Publisher::new(traffic),
            running: false,
            shutting_down: false,
        }
    }


    pub fn run(&mut self) {

        let mut rng = rand::thread_rng();

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
                let target = city.get_index(&Cell{x: 256, y: 256, d: Direction::East});
                let node_count = city.get_num_nodes();
                let edges = city.create_edges();
                let network = Network::new(node_count, &edges);
                let costs = network.dijkstra(target);
                let mut occupancy = Occupancy::new(city, &self.traffic.vehicles);

                while self.running {
                    println!("Simulating traffic with city version {}", city.id);
                    println!("Clearing occupancy");

                    for vehicle in self.traffic.vehicles.iter_mut() {
                        let node = city.get_index(vehicle);
                        let neighbours: Vec<u32> = network.get_out(node).iter().map(|e| e.to).collect();
                        let free_neighbours: Vec<u32> = neighbours.iter().cloned()
                            .filter(|n| {
                                let cell = city.get_cell(*n);
                                !occupancy.is_free(cell.x as usize, cell.y as usize)
                            }).collect();
                        let lowest_cost = free_neighbours.iter()
                            .map(|n| costs.get(*n as usize))
                            .min();
                        if let Some(lowest_cost) = lowest_cost {
                            if lowest_cost < costs.get(node as usize) {
                                // Get some neighbour with lowest cost
                                let candidates: Vec<u32> = free_neighbours.iter().cloned()
                                    .filter(|n| costs.get(*n as usize) == lowest_cost)
                                    .collect();
                                let selected = rng.choose(&candidates).unwrap();
                                
                                occupancy.free(vehicle);
                                let cell = city.get_cell(*selected);
                                vehicle.x = cell.x;
                                vehicle.y = cell.y;
                                vehicle.d = cell.d;
                                    
                                if *selected != target {
                                    occupancy.occupy(vehicle);
                                }
                            }
                        }
                    }
                    self.traffic.id += 1;
                    self.traffic_publisher.publish(&self.traffic);
                    self.check_messages();
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
                    SimulationMessage::Start => {
                        println!("Starting simulation");
                        self.running = true;
                    },
                    SimulationMessage::Pause => {
                        println!("Pausing simulation");
                        self.running = false;
                    },
                    SimulationMessage::Shutdown => {
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


