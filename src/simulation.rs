extern crate rand;
extern crate network;

use std::sync::mpsc::Receiver;
use version::{Version, Publisher, Local};
use super::{City, Traffic, Cell};
use network::{Network, Edge};

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

        while !self.shutting_down {

            while(!self.running) {
                println!("Paused");
                check_messages(&mut self.rx, &mut self.running, &mut self.shutting_down);
            }

            self.city.update();

            let city = &self.city.local;

            if let &Some(ref city) = city {
                let node_count = city.get_num_nodes();
                println!("Edges");
                let edges = city.create_edges();
                println!("Network");
                let network = Network::new(city.get_num_nodes(), &edges);
                println!("Dijkstra");
                let costs = network.dijkstra(32896);

                let mut occupancy = vec![vec![false; city.width as usize]; city.height as usize];

                while (self.running) {
                    println!("Simulating traffic with city version {}", city.id);
                    println!("Clearing occupancy");
                    clear_matrix(&mut occupancy);

                    println!("Setting occupancy");
                    for vehicle in self.traffic.vehicles.iter_mut() {
                        *occupancy.get_mut(vehicle.x as usize).unwrap().get_mut(vehicle.y as usize).unwrap() = true;
                    }

                    for vehicle in self.traffic.vehicles.iter_mut() {
                        // Find node that vehicle occupies
                        let node = city.get_index(vehicle);
                        println!("Node: {}", node);
                        // Find adjacent nodes (easy using network)
                        let neighbours: Vec<u32> = network.get_out(node).iter().map(|e| e.to).collect();
                        println!("Neighbours: {:?}", neighbours);
                        // Filter this to free nodes
                        let free_neighbours: Vec<u32> = neighbours.iter().cloned()
                            .filter(|n| {
                                let cell = city.get_cell(*n);
                                !occupancy.get(cell.x as usize).unwrap().get(cell.y as usize).unwrap()
                            }).collect();
                        println!("Free neighbours: {:?}", free_neighbours);
                        // Get lowest cost node
                        let lowest_cost = free_neighbours.iter().cloned()
                            .min_by(|a, b| costs.get(*a as usize).unwrap().cmp(costs.get(*b as usize).unwrap()));
                        println!("Lowest cost: {:?}", lowest_cost);
                        // Work out cell corresponding to this node
                        if let Some(lowest_cost) = lowest_cost {
                            if costs.get(lowest_cost as usize).unwrap() < costs.get(node as usize).unwrap() {
                                *occupancy.get_mut(vehicle.x as usize).unwrap().get_mut(vehicle.y as usize).unwrap() = false;
                                let cell = city.get_cell(lowest_cost);
                                vehicle.x = cell.x;
                                vehicle.y = cell.y;
                                vehicle.d = cell.d;
                                *occupancy.get_mut(vehicle.x as usize).unwrap().get_mut(vehicle.y as usize).unwrap() = true;
                            }
                        }
                    }
                    self.traffic.id += 1;
                    println!("{}", self.traffic.id);
                    self.traffic_publisher.publish(&self.traffic);
                    check_messages(&mut self.rx, &mut self.running, &mut self.shutting_down);
                }

            }
            else {
                while (self.running) {
                    println!("No city to simulate");
                    check_messages(&mut self.rx, &mut self.running, &mut self.shutting_down);
                }
            }

            
        }
    }

}

fn check_messages(rx: &mut Receiver<SimulationMessage>, running: &mut bool, shutting_down: &mut bool) {
    match rx.try_recv() {
        Ok(m) => {
            match m {
                SimulationMessage::Start => {
                    println!("Starting simulation");
                    *running = true;
                },
                SimulationMessage::Pause => {
                    println!("Pausing simulation");
                    *running = false;
                },
                SimulationMessage::Shutdown => {
                    println!("Shutting down simulation");
                    *shutting_down = true;
                },
            }
        },
        _ => (),
    }

}


fn clear_matrix(matrix: &mut Vec<Vec<bool>>) {
    for row in matrix {
        for cell in row {
            *cell = false;
        }
    }


}

