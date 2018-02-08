use std::sync::{Arc, RwLock};

pub struct Master<T: Clone> {
    pub master: T,
    published: Arc<RwLock<Option<Arc<T>>>>,
}

impl <T: Clone> Master<T> {

    pub fn new(t: T) -> Master<T> {
        Master {
            master: t,
            published: Arc::new(RwLock::new(None)),
        }
    }

    pub fn publish(&mut self) {
        let publish = self.master.clone();
        let publish = Arc::new(publish);
        let mut published = self.published.write().unwrap();
        *published = Some(Arc::clone(&publish));
    }

}

pub struct Local<T> {
    pub local: Option<Arc<T>>,
    published: Arc<RwLock<Option<Arc<T>>>>,
}

impl <T: Clone> Local<T> {

    pub fn new(master: &Master<T>) -> Local<T> {
        Local {
            local: None,
            published: Arc::clone(&master.published),
        }
    }

    pub fn update(&mut self) {
        match *self.published.read().unwrap() {
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

