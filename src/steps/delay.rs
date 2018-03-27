use simulation::{SimulationStep, SimulationState};
use std::thread;
use std::time::Duration;

pub struct Delay {
    milliseconds: u64,
}

impl Delay {
    pub fn new(milliseconds: u64) -> Delay {
        Delay{ milliseconds }
    }
}

impl SimulationStep for Delay {
    fn step(&self, state: SimulationState) -> SimulationState {
        thread::sleep(Duration::from_millis(self.milliseconds));
        state
    }
}
