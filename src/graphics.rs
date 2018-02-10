use version::{Version, Local};
use super::{City, Traffic};

pub struct Graphics {
    city: Local<City>,
    traffic: Local<Traffic>,
}

impl Graphics{

    pub fn new(city: &Version<City>, traffic: &Version<Traffic>) -> Graphics {
        Graphics {
            city: Local::new(city),
            traffic: Local::new(traffic),
        }
    }

    pub fn run(&mut self) {

        self.city.update();
        self.traffic.update();

        //match self.city.local {
        //    Some(ref c) => println!("Drawing with city version {}", c.id),
        //    None => println!("Drawing without city"),
        //}

        //match self.traffic.local {
        //    Some(ref t) => println!("Drawing with traffic version {}", t.id),
        //    None => println!("Drawing without traffic"),
        //}

    }
}
