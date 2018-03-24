extern crate rand;
extern crate network;

use std::sync::{Arc, RwLock};
use version::{Version, Publisher};
use super::{Vehicle, Traffic};
use rand::Rng;
use occupancy::Occupancy;

pub struct SimulationState {
    pub traffic: Traffic,
    pub occupancy: Occupancy,
    pub rng: Box<Rng>,
}

pub trait SimulationStep {
    fn step(&self, state: SimulationState) -> SimulationState;
}

pub trait VehicleUpdate {
    fn update(&self, vehicle: &mut Vehicle, occupancy: &mut Occupancy, rng: &mut Box<Rng>);
}

pub struct UpdateVehicles {
    pub updates: Vec<Box<VehicleUpdate>>,
}

impl SimulationStep for UpdateVehicles {
    fn step(&self, state: SimulationState) -> SimulationState {
        let mut traffic = state.traffic;
        let mut occupancy = state.occupancy;
        let mut rng = state.rng;
        for mut vehicle in traffic.vehicles.iter_mut() {
            for update in self.updates.iter() {
                update.update(&mut vehicle, &mut occupancy, &mut rng);
            }
        }
        SimulationState{ traffic, occupancy, rng}
    }
}

pub struct Simulation {
    pub steps: Vec<Box<SimulationStep>>,
}

impl Simulation {

    fn step(&self, state: SimulationState) -> SimulationState {

        let mut state = state;
        for step in self.steps.iter() {
            state = step.step(state);
        }
        state
    }
}


pub struct Simulator {
    simulation: Simulation,
    traffic_publisher: Publisher<Traffic>,
    running: Arc<RwLock<bool>>,
    shutting_down: Arc<RwLock<bool>>,
}

impl Simulator {

    pub fn new(simulation: Simulation,
               traffic: &Version<Traffic>,
               running: Arc<RwLock<bool>>,
               shutting_down: Arc<RwLock<bool>>) -> Simulator {
        Simulator{
            simulation,
            traffic_publisher: Publisher::new(traffic),
            running,
            shutting_down,
        }
    }


    pub fn run(&mut self, state: SimulationState) {

        let mut state = state;

        while !*self.shutting_down.read().unwrap() {

            while !*self.running.read().unwrap() {
                println!("Paused");
            }

            while *self.running.read().unwrap() {
                state = self.simulation.step(state);
                self.traffic_publisher.publish(&state.traffic);
            }
        }

    }

}
