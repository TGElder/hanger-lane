use std::thread;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Sender, Receiver};

fn main() {
    UI::run();
}

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
    working: Traffic,
    master: Arc<Mutex<Option<Arc<Traffic>>>>,
}

impl Simulation {

    fn new(vehicles: usize, master: Arc<Mutex<Option<Arc<Traffic>>>>) -> Simulation {
        Simulation{ working: Traffic::new(vehicles), master }
    }

    fn step(&mut self) {
        self.working.id += 1;
        let mut in_master = self.master.lock().unwrap();
        let master = self.working.clone();
        let master = Arc::new(master);
        *in_master = Some(Arc::clone(&master));
    }

}


struct UI {
}

impl UI {
    
    fn run() {
        let city = City::from("city");
        let city = Arc::new(city);
        let traffic_master = Arc::new(Mutex::new(None));

        let mut sim = Simulation::new(1, Arc::clone(&traffic_master));
        let sim_handle = thread::spawn(move || {
            loop {
                sim.step();
            }
        });

        
        let mut graphics = Graphics { city: Some(city), traffic: LocalCopy::new(&traffic_master), i: 0 };
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
    traffic: LocalCopy<Traffic>,
    i: u32,
}

struct LocalCopy<T> {
    local: Option<Arc<T>>,
    master: Arc<Mutex<Option<Arc<T>>>>,
}

impl <T> LocalCopy<T> {

    fn new(master: &Arc<Mutex<Option<Arc<T>>>>) -> LocalCopy<T> {
        LocalCopy { local: None, master: Arc::clone(master) }
    }

    fn update(&mut self) {
        match *self.master.lock().unwrap() {
            Some(ref t) => self.local = Some(Arc::clone(t)),
            None => self.local = None,
        }
    }

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
