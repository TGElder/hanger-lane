use std::thread;
use std::sync::{Arc, RwLock, mpsc};
use std::sync::mpsc::Sender;
use std::time::Duration;
use version::Publisher;
use simulation::{Simulator, SimulatorMessage};
use super::City;
use graphics::Graphics;
use editor::Editor;

pub struct UI {
}

impl UI {
    
    pub fn launch() {
        let city = Arc::new(RwLock::new(None));
        let mut city_publisher = Publisher::new(&city);
        city_publisher.publish(&City::with_all_roads(512, 512));

        let traffic = Arc::new(RwLock::new(None));

        let (sim_tx, sim_rx) = mpsc::channel();
        let mut sim = Simulator::new(sim_rx, &city, 8192, &traffic);
        let mut graphics = Graphics::new(&city, &traffic, "Hanger Lane", 1024, 1024);
        let mut editor = Editor::new(&city);

        let sim_handle = thread::spawn(move || {
            sim.run();
        });

        let sim_tx_2 = Sender::clone(&sim_tx);
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            sim_tx_2.send(SimulatorMessage::Start).unwrap();

            thread::sleep(Duration::from_secs(30));
            editor.run();

            thread::sleep(Duration::from_secs(1));
            sim_tx_2.send(SimulatorMessage::Pause).unwrap();
        });

        // Window needs to be created in main thread
        graphics.run();
        sim_tx.send(SimulatorMessage::Shutdown).unwrap();

        sim_handle.join().unwrap();

    }
}

