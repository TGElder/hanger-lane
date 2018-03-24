#[derive(Clone)]
pub struct Occupancy {
    occupancy: Vec<bool>,
}

impl Occupancy {

    pub fn new(node_count: usize) -> Occupancy {
        let occupancy = vec![false; node_count];
        Occupancy{ occupancy }
    }
    
    pub fn is_free(&self, index: usize) -> bool {
        !self.occupancy.get(index).unwrap()
    }

    fn set(&mut self, index: usize, value: bool) {
        *self.occupancy.get_mut(index).unwrap() = value;
    }

    pub fn free(&mut self, index: usize) {
        self.set(index, false);
    }

    pub fn occupy(&mut self, index: usize) {
        self.set(index, true);
    }

}

#[cfg(test)]
mod tests {

    use occupancy::Occupancy;

    #[test]
    fn free_then_occupy() {
        let mut occupancy = Occupancy::new(10);
        occupancy.free(7);
        occupancy.occupy(7);
        assert!(!occupancy.is_free(7));
    }

    #[test]
    fn occupy_then_free() {
        let mut occupancy = Occupancy::new(10);
        occupancy.occupy(7);
        occupancy.free(7);
        assert!(occupancy.is_free(7));
    }

}

