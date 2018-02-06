use std::thread;
use std::sync::{Arc, mpsc};
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

enum Message {
    CityUpdated(Arc<City>),
    TrafficUpdated(Arc<Traffic>),
}

struct Simulation {
    //city: City,
    traffic: Traffic,
    listeners: Vec<Sender<Message>>,
}

impl Simulation {

    fn new(vehicles: usize) -> Simulation {
        Simulation{ traffic: Traffic::new(vehicles), listeners: vec![] }
    }

    fn step(&mut self) {
        let traffic_done = self.traffic.clone();
        let traffic_done = Arc::new(traffic_done);
        for listener in self.listeners.iter() {
            listener.send(Message::TrafficUpdated(Arc::clone(&traffic_done)));
        }
    }

}


struct UI {
}

impl UI {
    
    fn run() {
        let city = City::from("city");
        let city = Arc::new(city);

        let (tx, rx) = mpsc::channel();
        let mut sim = Simulation::new(1024);
        sim.listeners.push(Sender::clone(&tx));
        let sim_handle = thread::spawn(move || {
            loop {
                sim.step();
            }
        });
        
        let mut graphics = Graphics { city: Some(city), traffic: None, rx, i: 0 };
        let graphics_handle = thread::spawn(move || {
            loop {
                graphics.run();
            }
        });

        tx.send(Message::CityUpdated(Arc::new(City::from("some other city"))));

        sim_handle.join();

    }
}

struct Graphics {
    city: Option<Arc<City>>,
    traffic: Option<Arc<Traffic>>,
    rx: Receiver<Message>,
    i: u32,
}

impl Graphics{
    fn run(&mut self) {

        for message in self.rx.iter() {
            match message {
                Message::CityUpdated(c) => {
                    println!("Updating city in Graphics");
                    self.city = Some(c);
                },
                Message::TrafficUpdated(t) => {
                    println!("Updating traffic in Graphics {}", self.i);
                    self.traffic = Some(t);
                    self.i += 1;
                },
                _ => panic!("Unsupported message"),
            }
        }
    }
}



struct Editor {
    city: City,
}
