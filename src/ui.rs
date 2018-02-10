use std::thread;
use std::sync::Arc;
use std::time::Duration;
use version::{Master, Local};
use simulation::Simulation;
use super::{City, Traffic};

pub struct UI {
}

impl UI {
    
    pub fn launch() {
        let city = City::from("city");
        let mut city = Master::new(city);
        city.publish();

        let mut sim = Simulation::new(&city, 1024*1024);
        let mut graphics = Graphics::new(&city, &sim.traffic);

        let sim_handle = thread::spawn(move || {
            loop {
                sim.step();
            }
        });

        
        let graphics_handle = thread::spawn(move || {
            loop {
                graphics.run();
            }
        });

        sim_handle.join();

    }
}

struct Graphics {
    city: Local<City>,
    traffic: Local<Traffic>,
}

impl Graphics{

    fn new(city: &Master<City>, traffic: &Master<Traffic>) -> Graphics {
        Graphics {
            city: Local::new(city),
            traffic: Local::new(traffic),
        }
    }


    fn run(&mut self) {

        self.city.update();
        self.traffic.update();

        match self.city.local {
            Some(ref c) => println!("Drawing with city version {}", c.id),
            None => println!("Drawing without city"),
        }

        match self.traffic.local {
            Some(ref t) => println!("Drawing with traffic version {}", t.id),
            None => println!("Drawing without traffic"),
        }

    }
}



struct Editor {
    city: City,
}
