use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use version::{Version, Publisher, Local};
use simulation::Simulation;
use super::{City, Traffic};

pub struct UI {
}

impl UI {
    
    pub fn launch() {
        let city = Arc::new(RwLock::new(None));
        let mut city_publisher = Publisher::new(&city);
        city_publisher.publish(&City::from("a city"));

        let traffic = Arc::new(RwLock::new(None));

        let mut sim = Simulation::new(&city, 1024*1024, &traffic);
        let mut graphics = Graphics::new(&city, &traffic);

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

    fn new(city: &Version<City>, traffic: &Version<Traffic>) -> Graphics {
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
