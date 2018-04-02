use std::cell::{Cell, RefCell};
use occupancy::Occupancy;
use simulation::{SimulationState, SimulationStep};

pub trait Timer {
    fn ready(&mut self) -> bool;
    fn reset(&mut self);
}

pub struct TrafficLights {
    nodes: Vec<Vec<usize>>,
    timer: RefCell<Box<Timer>>,
    cycle: Cell<usize>,
}

impl TrafficLights {

    pub fn new(nodes: Vec<Vec<usize>>, timer: RefCell<Box<Timer>>, occupancy: &mut Occupancy) -> TrafficLights {
        let out = TrafficLights{ nodes, timer, cycle: Cell::new(0) };
        out.lock_all(occupancy);
        out.unlock(0, occupancy);
        out
    }

    fn unlock(&self, cycle: usize, occupancy: &mut Occupancy) {
        for node in self.nodes[cycle].iter() {
            occupancy.unlock(*node);
        }
    }

    fn lock(&self, cycle: usize, occupancy: &mut Occupancy) {
        for node in self.nodes[cycle].iter() {
            occupancy.lock(*node);
        }
    }

    fn lock_all(&self, occupancy: &mut Occupancy) {
        for cycle in 0..self.nodes.len() {
            self.lock(cycle, occupancy);
        }
    }

}

impl SimulationStep for TrafficLights {
    fn step(&self, state: SimulationState) -> SimulationState {
        let mut occupancy = state.occupancy;
        let mut timer = self.timer.borrow_mut();
        if timer.ready() {
            let next_cycle = (self.cycle.get() + 1) % self.nodes.len();
            self.lock(self.cycle.get(), &mut occupancy);
            self.unlock(next_cycle, &mut occupancy);
            self.cycle.set(next_cycle);
            timer.reset();
        }
        SimulationState{ occupancy, ..state}
    }
    
}

#[cfg(test)]
mod tests {

    extern crate rand;

    use std::cell::RefCell;
    use Traffic;
    use steps::traffic_lights::{TrafficLights, Timer};
    use occupancy::Occupancy;
    use rand::Rng;
    use simulation::{SimulationStep, SimulationState};

    pub struct MockTimer {
    }

    impl Timer for MockTimer {
        fn ready(&mut self) -> bool {
            true
        }

        fn reset(&mut self) {
        }
    }

    #[test]
    fn should_initialise_with_first_cycle_unlocked() {
        let mut occupancy = Occupancy::new(6);
        TrafficLights::new(vec![vec![1, 3], vec![2, 4]],
                                                RefCell::new(Box::new(MockTimer{})),
                                                &mut occupancy);
        assert!(occupancy.is_unlocked(0));
        assert!(occupancy.is_unlocked(1));
        assert!(!occupancy.is_unlocked(2));
        assert!(occupancy.is_unlocked(3));
        assert!(!occupancy.is_unlocked(4));
        assert!(occupancy.is_unlocked(5));
    }

    #[test]
    fn cycle_once() {
        let traffic = Traffic{ id: 0, vehicles: vec![] };
        let rng: Box<Rng> = Box::new(rand::thread_rng());
        let mut occupancy = Occupancy::new(6);
        let traffic_lights = TrafficLights::new(vec![vec![1, 3], vec![2, 4]],
                                                RefCell::new(Box::new(MockTimer{})),
                                                &mut occupancy);
        let mut state = SimulationState{ traffic, rng, occupancy };
        state = traffic_lights.step(state);
        let occupancy = state.occupancy;

        assert!(occupancy.is_unlocked(0));
        assert!(!occupancy.is_unlocked(1));
        assert!(occupancy.is_unlocked(2));
        assert!(!occupancy.is_unlocked(3));
        assert!(occupancy.is_unlocked(4));
        assert!(occupancy.is_unlocked(5));
    }
    
    #[test]
    fn cycle_twice() {
        let traffic = Traffic{ id: 0, vehicles: vec![] };
        let rng: Box<Rng> = Box::new(rand::thread_rng());
        let mut occupancy = Occupancy::new(6);
        let traffic_lights = TrafficLights::new(vec![vec![1, 3], vec![2, 4]],
                                                RefCell::new(Box::new(MockTimer{})),
                                                &mut occupancy);
        let mut state = SimulationState{ traffic, rng, occupancy };
        state = traffic_lights.step(state);
        state = traffic_lights.step(state);
        let occupancy = state.occupancy;

        assert!(occupancy.is_unlocked(0));
        assert!(occupancy.is_unlocked(1));
        assert!(!occupancy.is_unlocked(2));
        assert!(occupancy.is_unlocked(3));
        assert!(!occupancy.is_unlocked(4));
        assert!(occupancy.is_unlocked(5));
    }

}
