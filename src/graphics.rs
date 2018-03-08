extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use version::{Version, Local};
use super::{Cell, City, Traffic};
use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow as Window;
use graphics::graphics::Context;
use self::opengl_graphics::{ GlGraphics, OpenGL };
use std::sync::Arc;

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
            .fullscreen(true)
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
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if let Some(_) = e.update_args() {
                self.city.update();
                self.traffic.update();
            }
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

    fn render(&mut self, args: &RenderArgs) {
        use graphics::graphics::clear;
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let traffic = &self.traffic;

        self.graphics.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
            traffic.local.render(gl, &c);
        })
    }

}

trait Render {
    fn render(&self, graphics: &mut GlGraphics, context: &Context);
}

impl <T: Render> Render for Option<T> {
    fn render(&self, graphics: &mut GlGraphics, context: &Context) {
        match self {
            &Some(ref t) => t.render(graphics, context),
            &None => (),
        }
    }
}

impl <T: Render> Render for Arc<T> {
    fn render(&self, graphics: &mut GlGraphics, context: &Context) {
        let t: &T = &self;
        t.render(graphics, context)
    }
}

impl Render for Traffic {
    fn render(&self, graphics: &mut GlGraphics, context: &Context) {
        for vehicle in self.vehicles.iter() {
            vehicle.render(graphics, context);
        }
    }
}

impl Render for Cell {

    fn render(&self, graphics: &mut GlGraphics, context: &Context) {
        use graphics::graphics::rectangle;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(self.x as f64/34f64, self.y as f64/61f64, 2.0);
        rectangle(BLACK, square, context.transform, graphics);
    }
}

