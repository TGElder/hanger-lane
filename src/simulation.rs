use version::{Version, Publisher, Local};
use super::{City, Traffic};

enum SimulationMessage {
    Start,
    Pause,
}

pub struct Simulation {
    city: Local<City>,
    traffic: Traffic,
    pub traffic_publisher: Publisher<Traffic>,
}

impl Simulation {

    pub fn new(city: &Version<City>, vehicles: usize, traffic: &Version<Traffic>) -> Simulation {
        Simulation{
           city: Local::new(city),
           traffic: Traffic::new(vehicles),
           traffic_publisher: Publisher::new(traffic),
        }
    }

    pub fn step(&mut self) {

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

