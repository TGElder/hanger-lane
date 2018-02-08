use std::sync::{Arc, Mutex};

pub struct Master<T: Clone> {
    pub master: T,
    published: Arc<Mutex<Option<Arc<T>>>>,
}

impl <T: Clone> Master<T> {

    pub fn new(t: T) -> Master<T> {
        Master {
            master: t,
            published: Arc::new(Mutex::new(None)),
        }
    }

    pub fn publish(&mut self) {
        let mut published = self.published.lock().unwrap();
        let publish = self.master.clone();
        let publish = Arc::new(publish);
        *published = Some(Arc::clone(&publish));
    }

}

pub struct Local<T> {
    pub local: Option<Arc<T>>,
    published: Arc<Mutex<Option<Arc<T>>>>,
}

impl <T: Clone> Local<T> {

    pub fn new(master: &Master<T>) -> Local<T> {
        Local {
            local: None,
            published: Arc::clone(&master.published),
        }
    }

    pub fn update(&mut self) {
        match *self.published.lock().unwrap() {
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

