extern crate rand; 
use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use version::Publisher;
use simulation::Simulator;
use super::{Cell, City, DIRECTIONS};
use graphics::Graphics;
use editor::Editor;
use rand::{Rng, ThreadRng};

pub struct UI {
}

impl UI {
    
    pub fn launch() {

        let mut rng = rand::thread_rng();
        let mut sources = vec![];
        let mut destinations = vec![];

        for _ in 0..1 {
            sources.push(get_random_cell(&mut rng));
        }

        for _ in 0..64 {
            destinations.push(get_random_cell(&mut rng));
        }

        let city = City::with_all_roads(512, 512, sources, destinations);
        let city_version = Arc::new(RwLock::new(None));

        let mut city_publisher = Publisher::new(&city_version);
        city_publisher.publish(&city);

        let traffic = Arc::new(RwLock::new(None));

        let sim_run = Arc::new(RwLock::new(true));
        let sim_shutdown = Arc::new(RwLock::new(false));
        let mut sim = Simulator::new(&city_version, &traffic, Arc::clone(&sim_run), Arc::clone(&sim_shutdown));
        let mut graphics = Graphics::new(&city_version, &traffic, "Hanger Lane", 1024, 1024);
        let mut editor = Editor::new(&city_version);

        let sim_handle = thread::spawn(move || {
            sim.run();
        });

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            *sim_run.write().unwrap() = true;

            thread::sleep(Duration::from_secs(60));
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

fn get_random_cell(rng: &mut ThreadRng) -> Cell {
    Cell {
        x: rng.gen_range(0, 512),
        y: rng.gen_range(0, 512),
        d: DIRECTIONS[rng.gen_range(0, 4)],
    }
}

