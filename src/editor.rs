use super::City;
use super::version::{Version, Publisher};

pub struct Editor {
    city_publisher: Publisher<City>,
}

impl Editor {

    pub fn new(city: &Version<City>) -> Editor {
        Editor {
            city_publisher: Publisher::new(city),
        }
    }

    pub fn run(&mut self) {
        let mut city = City::from("another city");
        city.id = 1;

        self.city_publisher.publish(&city);
    }
}