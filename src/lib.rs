mod version;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use version::{Master, Local};

struct Cell {
    index: usize,
}

struct City {
    cells: Vec<Cell>,
}

impl City {
    fn new(size: usize) -> City {
        City{ cells: (0..size).map(|i| Cell{ index: i}).collect() }
    }

    fn from(file: &str) -> City {
        City::new(1048576)
    }
}

#[derive(Clone, Debug)]
struct Vehicle {
    x: u32,
    y: u32,
}

#[derive(Clone, Debug)]
struct Traffic {
    id: usize,
    vehicles: Vec<Vehicle>,
}

impl Traffic {
    fn new(size: usize) -> Traffic {
        Traffic{ id: 0, vehicles: (0..size).map(|i| Vehicle{ x: 0, y: 0 }).collect() }
    }
}

struct Simulation {
    //city: City,
    traffic: Master<Traffic>,
}

impl Simulation {

    fn new(vehicles: usize) -> Simulation {
        Simulation{ traffic: Master::new(Traffic::new(vehicles)) }
    }

    fn step(&mut self) {
        self.traffic.master.id += 1;
        self.traffic.publish();
    }

}


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
