use simulation::VehicleUpdate;
use Vehicle;
use occupancy::Occupancy;
use rand::Rng;

pub struct VehicleFree {
    block_size: usize,
}

impl VehicleFree {
    pub fn new(block_size: usize) -> VehicleFree {
        VehicleFree{ block_size }
    }
}

impl VehicleUpdate for VehicleFree {
    fn update(&self, vehicle: &mut Vehicle, occupancy: &mut Occupancy, _rng: &mut Box<Rng>) {
        let start = self.block_size * (vehicle.location / self.block_size);

        for offset in 0..self.block_size {
            occupancy.unlock(start + offset);
        }
    }
}

pub struct VehicleOccupy {
    block_size: usize,
}

impl VehicleOccupy {
    pub fn new(block_size: usize) -> VehicleOccupy {
        VehicleOccupy{ block_size }
    }
}

impl VehicleUpdate for VehicleOccupy {
    fn update(&self, vehicle: &mut Vehicle, occupancy: &mut Occupancy, _rng: &mut Box<Rng>) {
        if !vehicle.destination.contains(&vehicle.location) {
            let start = self.block_size * (vehicle.location / self.block_size);

            for offset in 0..self.block_size {
                occupancy.lock(start + offset);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::VehicleFree;
    use super::VehicleOccupy;
    use simulation::VehicleUpdate;
    use Vehicle;
    use occupancy::Occupancy;
    use rand::Rng;

    #[test]
    fn free_then_occupy_start_of_range() {
        let free = VehicleFree::new(3);
        let occupy = VehicleOccupy::new(3);
        let mut vehicle = Vehicle{ location: 0, destination: vec![1], destination_index: 0 };
        let mut occupancy = Occupancy::new(9);
        occupancy.remove_all_locks(0);
        let mut rng: Box<Rng> = Box::new(rand::thread_rng());
        occupy.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(!occupancy.is_unlocked(0));
        assert!(!occupancy.is_unlocked(1));
        assert!(!occupancy.is_unlocked(2));
    }

    #[test]
    fn free_then_occupy_mid_range() {
        let free = VehicleFree::new(4);
        let occupy = VehicleOccupy::new(4);
        let mut vehicle = Vehicle{ location: 5, destination: vec![6], destination_index: 0 };
        let mut occupancy = Occupancy::new(12);
        occupancy.remove_all_locks(0);
        let mut rng: Box<Rng> = Box::new(rand::thread_rng());
        occupy.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(!occupancy.is_unlocked(4));
        assert!(!occupancy.is_unlocked(5));
        assert!(!occupancy.is_unlocked(6));
        assert!(!occupancy.is_unlocked(7));
    }

    #[test]
    fn free_then_occupy_end_of_range() {
        let free = VehicleFree::new(5);
        let occupy = VehicleOccupy::new(5);
        let mut vehicle = Vehicle{ location: 10, destination: vec![11], destination_index: 0 };
        let mut occupancy = Occupancy::new(15);
        occupancy.remove_all_locks(0);
        let mut rng: Box<Rng> = Box::new(rand::thread_rng());
        occupy.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(!occupancy.is_unlocked(10));
        assert!(!occupancy.is_unlocked(11));
        assert!(!occupancy.is_unlocked(12));
        assert!(!occupancy.is_unlocked(13));
        assert!(!occupancy.is_unlocked(14));
    }

    #[test]
    fn occupy_then_free_start_of_range() {
        let free = VehicleFree::new(3);
        let occupy = VehicleOccupy::new(3);
        let mut vehicle = Vehicle{ location: 0, destination: vec![1], destination_index: 0 };
        let mut occupancy = Occupancy::new(9);
        let mut rng: Box<Rng> = Box::new(rand::thread_rng());
        occupy.update(&mut vehicle, &mut occupancy, &mut rng);
        free.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(occupancy.is_unlocked(0));
        assert!(occupancy.is_unlocked(1));
        assert!(occupancy.is_unlocked(2));
    }

    #[test]
    fn occupy_then_free_mid_range() {
        let free = VehicleFree::new(4);
        let occupy = VehicleOccupy::new(4);
        let mut vehicle = Vehicle{ location: 5, destination: vec![6], destination_index: 0 };
        let mut occupancy = Occupancy::new(12);
        let mut rng: Box<Rng> = Box::new(rand::thread_rng());
        occupy.update(&mut vehicle, &mut occupancy, &mut rng);
        free.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(occupancy.is_unlocked(4));
        assert!(occupancy.is_unlocked(5));
        assert!(occupancy.is_unlocked(6));
        assert!(occupancy.is_unlocked(7));
    }

    #[test]
    fn occupy_then_free_end_of_range() {
        let free = VehicleFree::new(5);
        let occupy = VehicleOccupy::new(5);
        let mut vehicle = Vehicle{ location: 10, destination: vec![11], destination_index: 0 };
        let mut occupancy = Occupancy::new(15);
        let mut rng: Box<Rng> = Box::new(rand::thread_rng());
        occupy.update(&mut vehicle, &mut occupancy, &mut rng);
        free.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(occupancy.is_unlocked(10));
        assert!(occupancy.is_unlocked(11));
        assert!(occupancy.is_unlocked(12));
        assert!(occupancy.is_unlocked(13));
        assert!(occupancy.is_unlocked(14));
    }

    #[test]
    fn should_not_occupy_destination() {
        let free = VehicleFree::new(3);
        let occupy = VehicleOccupy::new(3);
        let mut vehicle = Vehicle{ location: 0, destination: vec![0, 1], destination_index: 0 };
        let mut occupancy = Occupancy::new(9);
        occupancy.remove_all_locks(0);
        let mut rng: Box<Rng> = Box::new(rand::thread_rng());
        occupy.update(&mut vehicle, &mut occupancy, &mut rng);
        assert!(occupancy.is_unlocked(0));
        assert!(occupancy.is_unlocked(1));
        assert!(occupancy.is_unlocked(2));
    }
}
