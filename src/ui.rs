use std::thread;
use std::sync::{Arc, RwLock, mpsc};
use std::time::Duration;
use version::{Version, Publisher, Local};
use simulation::{Simulation, SimulationMessage};
use super::{City, Traffic};

pub struct UI {
}

impl UI {
    
    pub fn launch() {
        let city = Arc::new(RwLock::new(None));
        let mut city_publisher = Publisher::new(&city);
        city_publisher.publish(&City::from("a city"));

        let traffic = Arc::new(RwLock::new(None));

        let (sim_tx, sim_rx) = mpsc::channel();
        let mut sim = Simulation::new(sim_rx, &city, 1024*1024, &traffic);
        let mut graphics = Graphics::new(&city, &traffic);

        let sim_handle = thread::spawn(move || {
            sim.run();
        });

        
        let graphics_handle = thread::spawn(move || {
            loop {
                graphics.run();
            }
        });

        thread::sleep(Duration::from_secs(1));
        sim_tx.send(SimulationMessage::Start).unwrap();

        thread::sleep(Duration::from_secs(1));
        let mut editor = Editor::new(&city);
        editor.run();

        thread::sleep(Duration::from_secs(1));
        sim_tx.send(SimulationMessage::Pause).unwrap();

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

        //match self.city.local {
        //    Some(ref c) => println!("Drawing with city version {}", c.id),
        //    None => println!("Drawing without city"),
        //}

        //match self.traffic.local {
        //    Some(ref t) => println!("Drawing with traffic version {}", t.id),
        //    None => println!("Drawing without traffic"),
        //}

    }
}



struct Editor {
    city_publisher: Publisher<City>,
}

impl Editor {

    fn new(city: &Version<City>) -> Editor {
        Editor {
            city_publisher: Publisher::new(city),
        }
    }

    fn run(&mut self) {
        let mut city = City::from("another city");
        city.id = 1;

        self.city_publisher.publish(&city);
    }
}

