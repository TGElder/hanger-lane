#[derive(Clone)]
pub struct Occupancy {
    occupancy: Vec<usize>,
}

impl Occupancy {

    pub fn new(node_count: usize) -> Occupancy {
        let occupancy = vec![0; node_count];
        Occupancy{ occupancy }
    }
    
    pub fn is_unlocked(&self, index: usize) -> bool {
        self.occupancy[index] == 0
    }

    pub fn remove_all_locks(&mut self, index: usize) {
        self.occupancy[index] = 0;
    }

    pub fn unlock(&mut self, index: usize) {
        if self.occupancy[index] == 0 {
            panic!("No locks left to remove");
        }
        self.occupancy[index] -= 1;
    }

    pub fn lock(&mut self, index: usize) {
        self.occupancy[index] += 1;
    }

}

#[cfg(test)]
mod tests {

    use occupancy::Occupancy;

    #[test]
    fn remove_all_locks() {
        let mut occupancy = Occupancy::new(10);
        occupancy.remove_all_locks(7);
        assert!(occupancy.is_unlocked(7));
    }

    #[test]
    fn lock() {
        let mut occupancy = Occupancy::new(10);
        occupancy.remove_all_locks(7);
        occupancy.lock(7);
        assert!(!occupancy.is_unlocked(7));
    }

    #[test]
    fn lock_then_unlock() {
        let mut occupancy = Occupancy::new(10);
        occupancy.remove_all_locks(7);
        occupancy.lock(7);
        occupancy.unlock(7);
        assert!(occupancy.is_unlocked(7));
    }
    
    #[test]
    fn double_lock() {
        let mut occupancy = Occupancy::new(10);
        occupancy.remove_all_locks(7);
        occupancy.lock(7);
        occupancy.lock(7);
        assert!(!occupancy.is_unlocked(7));
        occupancy.unlock(7);
        assert!(!occupancy.is_unlocked(7));
        occupancy.unlock(7);
        assert!(occupancy.is_unlocked(7));
    }

    #[test]
    #[should_panic]
    fn remove_too_many_locks() {
        let mut occupancy = Occupancy::new(10);
        occupancy.remove_all_locks(7);
        occupancy.unlock(7);
    }

}

