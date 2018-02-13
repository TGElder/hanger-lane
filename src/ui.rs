use std::thread;
use std::sync::{Arc, RwLock, mpsc};
use std::time::Duration;
use version::Publisher;
use simulation::{Simulation, SimulationMessage};
use super::City;
use graphics::Graphics;
use editor::Editor;

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
        let mut graphics = Graphics::new(&city, &traffic, "Hanger Lane", 512, 512);

        let sim_handle = thread::spawn(move || {
            sim.run();
        });

        let graphics_handle = thread::spawn(move || {
            graphics.run();
        });

        thread::sleep(Duration::from_secs(1));
        sim_tx.send(SimulationMessage::Start).unwrap();

        thread::sleep(Duration::from_secs(1));
        let mut editor = Editor::new(&city);
        editor.run();

        thread::sleep(Duration::from_secs(1));
        sim_tx.send(SimulationMessage::Pause).unwrap();

        sim_handle.join().unwrap();
        graphics_handle.join().unwrap();

    }
}

