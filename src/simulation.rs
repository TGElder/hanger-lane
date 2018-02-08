use version::{Master, Local};
use super::{City, Traffic};

pub struct Simulation {
    //city: City,
    pub traffic: Master<Traffic>,
}

impl Simulation {

    pub fn new(vehicles: usize) -> Simulation {
        Simulation{ traffic: Master::new(Traffic::new(vehicles)) }
    }

    pub fn step(&mut self) {
        self.traffic.master.id += 1;
        self.traffic.publish();
    }

}

