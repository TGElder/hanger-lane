extern crate rand;
extern crate network;

use std::sync::mpsc::Receiver;
use version::{Version, Publisher, Local};
use super::{City, Traffic};
use network::{Network, Edge};

pub enum SimulationMessage {
    Start,
    Pause,
    Shutdown,
}


struct SimData<'a> {
    edges: Vec<Edge>,
    network: Network<'a>,
    costs: Vec<Option<u32>>,
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

            if let Some(ref city) = self.city.local {
                println!("Edges");
                let edges = city.create_edges();
                println!("Network");
                let network = Network::new(city.get_num_nodes(), &edges);
                println!("Dijkstra");
                let costs = network.dijkstra(0);

                while (true) {
                    println!("Simulating traffic with city version {}", city.id);
                    for vehicle in self.traffic.vehicles.iter_mut() {
                        // Find node that vehicle occupies
                        let node = city.get_index(vehicle);
                        // Find adjacent nodes (easy using network)
                        let neighbours: Vec<u32> = network.get_out(node).iter().map(|e| e.to).collect();
                        // Filter this to free nodes
                        // Get lowest cost node
                        // Work out cell corresponding to this node
                    }
                    self.traffic.id += 1;
                    println!("{}", self.traffic.id);
                    self.traffic_publisher.publish(&self.traffic);
                }

            }
            else {
                while (true) {
                    println!("No city to simulate");
                }
            }
            
            //match self.rx.try_recv() {
            //    Ok(m) => {
            //        match m {
            //            SimulationMessage::Start => {
            //                println!("Starting simulation");
            //                self.running = true;
            //            },
            //            SimulationMessage::Pause => {
            //                println!("Pausing simulation");
            //                self.running = false;
            //            },
            //            SimulationMessage::Shutdown => {
            //                println!("Shutting down simulation");
            //                self.shutting_down = true;
            //            },
            //        }
            //    },
            //    _ => (),
            //}

            //if self.running {

            //    if (self.city.update()) {


            //    }



            //}
        }
    }

    fn evolve(&mut self, network: &Network) {

        if let Some(ref city) = self.city.local {

            for vehicle in self.traffic.vehicles.iter_mut() {
                // Find node that vehicle occupies
                let node = city.get_index(vehicle);
                // Find adjacent nodes (easy using network)
                let neighbours: Vec<u32> = network.get_out(node).iter().map(|e| e.to).collect();
                // Filter this to free nodes
                // Get lowest cost node
                // Work out cell corresponding to this node
            }
        }

    }

}

