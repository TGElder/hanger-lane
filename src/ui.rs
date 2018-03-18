use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use version::Publisher;
use simulation::Simulator;
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

        let sim_run = Arc::new(RwLock::new(true));
        let sim_shutdown = Arc::new(RwLock::new(false));
        let mut sim = Simulator::new(&city, &traffic, Arc::clone(&sim_run), Arc::clone(&sim_shutdown));
        let mut graphics = Graphics::new(&city, &traffic, "Hanger Lane", 1024, 1024);
        let mut editor = Editor::new(&city);

        let sim_handle = thread::spawn(move || {
            sim.run();
        });

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            *sim_run.write().unwrap() = true;

            thread::sleep(Duration::from_secs(30));
            editor.run();

            thread::sleep(Duration::from_secs(1));
            *sim_run.write().unwrap() = false;
        });

        // Window needs to be created in main thread
        graphics.run();

        *sim_shutdown.write().unwrap() = true;
        sim_handle.join().unwrap();

    }
}

