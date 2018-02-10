use version::{Master, Local};
use super::{City, Traffic};

pub struct Simulation {
    city: Local<City>,
    pub traffic: Master<Traffic>,
}

impl Simulation {

    pub fn new(city: &Master<City>, vehicles: usize) -> Simulation {
        Simulation{
           city: Local::new(city),
           traffic: Master::new(Traffic::new(vehicles)),
        }
    }

    pub fn step(&mut self) {

        self.city.update();

        match self.city.local {
            Some(ref c) => {
                println!("Simulating traffic with city version {}", c.id);
                self.traffic.master.id += 1;
                self.traffic.publish();
            },
            None => println!("Simulating without city"),
        }
    }

}

