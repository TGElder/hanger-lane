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
        let city = Arc::new(city);

        let mut sim = Simulation::new(1024*1024);
        let mut graphics = Graphics { city: Some(city), traffic: Local::new(&sim.traffic) };
        let sim_handle = thread::spawn(move || {
            loop {
                sim.step();
                thread::sleep(Duration::from_millis(1));
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
    city: Option<Arc<City>>,
    traffic: Local<Traffic>,
}

impl Graphics{
    fn run(&mut self) {

        self.traffic.update();

        match self.traffic.local {
            Some(ref t) => println!("Drawing with traffic version {}", t.id),
            None => println!("Drawing without traffic"),
        }

    }
}



struct Editor {
    city: City,
}
