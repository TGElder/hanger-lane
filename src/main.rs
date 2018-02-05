use std::thread;
use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Sender, Receiver};

fn main() {
   let city = City::new(1048576);
   let (tx, rx) = mpsc::channel();
   let mut sim = Simulation::new(1024);
   sim.listeners.push(tx);
   thread::spawn(move || {
       loop {
           sim.step();
       }
   });

   for message in rx {
       println!(".");
   }
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
}

#[derive(Clone)]
struct Vehicle {
    x: u32,
    y: u32,
}

#[derive(Clone)]
struct Traffic {
    vehicles: Vec<Vehicle>,
}

impl Traffic {
    fn new(size: usize) -> Traffic {
        Traffic{ vehicles: (0..size).map(|i| Vehicle{ x: 0, y: 0 }).collect() }
    }
}

struct Simulation {
    //city: City,
    traffic: Traffic,
    listeners: Vec<Sender<Arc<Traffic>>>,
}

impl Simulation {

    fn new(vehicles: usize) -> Simulation {
        Simulation{ traffic: Traffic::new(vehicles), listeners: vec![] }
    }

    fn step(&mut self) {
        let traffic_done = self.traffic.clone();
        let traffic_done = Arc::new(traffic_done);
        for listener in self.listeners.iter() {
            listener.send(Arc::clone(&traffic_done));
        }
    }

}


struct UI {
    city: City,
    traffic: Traffic,
    simulation: Simulation,
}

struct Graphics {
    city: City,
    traffic: Traffic,
}

struct Editor {
    city: City,
}
