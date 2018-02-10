use std::sync::{Arc, RwLock};

pub type Version<T> = Arc<RwLock<Option<Arc<T>>>>;

pub struct Publisher<T: Clone> {
    latest: Version<T>,
}

impl <T: Clone> Publisher<T> {

    pub fn new(latest: &Version<T>) -> Publisher<T> {
        Publisher {
            latest: Arc::clone(&latest),
        }
    }

    pub fn publish(&mut self, t: &T) {
        let publish = t.clone();
        let publish = Arc::new(publish);
        let mut latest = self.latest.write().unwrap();
        *latest = Some(Arc::clone(&publish));
    }

}

pub struct Local<T> {
    pub local: Option<Arc<T>>,
    latest: Version<T>,
}

impl <T: Clone> Local<T> {

    pub fn new(latest: &Version<T>) -> Local<T> {
        Local {
            local: None,
            latest: Arc::clone(&latest),
        }
    }

    pub fn update(&mut self) {
        match *self.latest.read().unwrap() {
            Some(ref p) => {
                if (match self.local {
                        Some(ref l) => !Arc::ptr_eq(p, l), // No point cloning the reference if it still points to the same value
                        None => true,
                    }
                ) {
                    self.local = Some(Arc::clone(p));
                }
            },
            None => self.local = None,
        }
    }

}

