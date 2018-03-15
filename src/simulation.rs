extern crate rand;
extern crate network;

use std::sync::mpsc::Receiver;
use std::sync::Arc;
use version::{Version, Publisher, Local};
use super::{City, Traffic, Cell};
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
                println!("Edges");
                let edges = city.create_edges();
                println!("Network");
                let network = Network::new(node_count, &edges);
                println!("Dijkstra");
                use super::Direction;
                let costs = network.dijkstra(target);

                let mut occupancy = vec![vec![false; city.width as usize]; city.height as usize];
                clear_matrix(&mut occupancy);

                println!("Setting occupancy");
                for vehicle in self.traffic.vehicles.iter_mut() {
                    *occupancy.get_mut(vehicle.x as usize).unwrap().get_mut(vehicle.y as usize).unwrap() = true;
                }

                while self.running {
                    println!("Simulating traffic with city version {}", city.id);
                    println!("Clearing occupancy");

                    for vehicle in self.traffic.vehicles.iter_mut() {
                        // Find node that vehicle occupies
                        let node = city.get_index(vehicle);
                        // Find adjacent nodes (easy using network)
                        let neighbours: Vec<u32> = network.get_out(node).iter().map(|e| e.to).collect();
                        // Filter this to free nodes
                        let free_neighbours: Vec<u32> = neighbours.iter().cloned()
                            .filter(|n| {
                                let cell = city.get_cell(*n);
                                !occupancy.get(cell.x as usize).unwrap().get(cell.y as usize).unwrap()
                            }).collect();
                        // Get lowest cost of neighbour
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
                                
                                *occupancy.get_mut(vehicle.x as usize).unwrap().get_mut(vehicle.y as usize).unwrap() = false;
                                let cell = city.get_cell(*selected);
                                vehicle.x = cell.x;
                                vehicle.y = cell.y;
                                vehicle.d = cell.d;
                                    
                                if (*selected != target) {
                                    *occupancy.get_mut(vehicle.x as usize).unwrap().get_mut(vehicle.y as usize).unwrap() = true;
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



fn clear_matrix(matrix: &mut Vec<Vec<bool>>) {
    for row in matrix {
        for cell in row {
            *cell = false;
        }
    }


}

