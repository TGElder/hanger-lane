extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use version::{Version, Local};
use super::{City, Traffic};
use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{ GlGraphics, OpenGL };

pub struct Graphics {
    city: Local<City>,
    traffic: Local<Traffic>,
    window: Window,
    graphics: GlGraphics,
}

impl Graphics{

    pub fn new(city: &Version<City>,
               traffic: &Version<Traffic>,
               title: &str,
               width: u32,
               height: u32) -> Graphics {
        let opengl = OpenGL::V3_2;
        Graphics {
            city: Local::new(city),
            traffic: Local::new(traffic),
            window: Graphics::create_window(title, width, height, opengl),
            graphics: Graphics::create_graphics(opengl),
        }
    }

    pub fn create_window(title: &str, width: u32, height: u32, opengl: OpenGL) -> Window {
        WindowSettings::new(title, [width, height])
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap()
    }

    pub fn create_graphics(opengl: OpenGL) -> GlGraphics {
        GlGraphics::new(opengl)
    }

    pub fn run(&mut self) {

        let mut events = Events::new(EventSettings::new());

        while let Some(e) = events.next(&mut self.window) {
            //if let Some(r) = e.render_args() {
            //    
            //}

            //if let Some(u) = e.update_args() {
            //    self.city.update();
            //    self.traffic.update();
            //}
        }




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
