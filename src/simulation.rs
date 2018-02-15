extern crate rand;

use std::sync::mpsc::Receiver;
use version::{Version, Publisher, Local};
use super::{City, Traffic};

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
                            self.shutting_down = true;
                        },
                    }
                },
                _ => (),
            }

            if self.running {

                self.evolve();

                self.city.update();

                match self.city.local {
                    Some(ref c) => {
                        println!("Simulating traffic with city version {}", c.id);
                        self.traffic.id += 1;
                        println!("{}", self.traffic.id);
                        self.traffic_publisher.publish(&self.traffic);
                    },
                    None => println!("Simulating without city"),
                }

            }
        }
    }

    fn evolve(&mut self) {
        for vehicle in self.traffic.vehicles.iter_mut() {
            vehicle.x = (vehicle.x as i32 + vehicle.vx as i32) as u16;
            vehicle.y = (vehicle.y as i32 + vehicle.vy as i32) as u16;
        }
    }

}

