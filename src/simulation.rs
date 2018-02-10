use std::sync::mpsc::Receiver;
use version::{Version, Publisher, Local};
use super::{City, Traffic};

pub enum SimulationMessage {
    Start,
    Pause,
}

pub struct Simulation {
    rx: Receiver<SimulationMessage>,
    city: Local<City>,
    traffic: Traffic,
    traffic_publisher: Publisher<Traffic>,
    running: bool,
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
        }
    }

    pub fn run(&mut self) {

        loop {
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
                    }
                },
                _ => (),
            }

            if self.running {

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

}

