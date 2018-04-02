extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use version::{Version, Local};
use Traffic;
use city::City;
use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow as Window;
use graphics::graphics::Context;
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
            .fullscreen(false)
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

        if let Some(ref traffic) = self.traffic.local {
            if let Some(ref city) = self.city.local {
                self.graphics.draw(args.viewport(), |c, gl| {
                    clear(WHITE, gl);
                    render_traffic(city, traffic, gl, &c);
                })
            }
        }
                
    }
}

fn render_traffic(city: &City, traffic: &Traffic, graphics: &mut GlGraphics, context: &Context) {
    use graphics::graphics::rectangle;

    const VEHICLE_SIZE: f64 = 12.0;
    const COLOURS: [[f32; 4]; 64] = [
        [0.45, 0.11, 0.72, 1.0],
        [0.43, 0.44, 0.17, 1.0],
        [0.86, 0.23, 0.04, 1.0],
        [0.96, 0.99, 0.09, 1.0],
        [0.35, 0.72, 0.66, 1.0],
        [0.99, 0.03, 0.24, 1.0],
        [0.92, 0.82, 0.2, 1.0],
        [0.24, 0.71, 0.38, 1.0],
        [0.92, 0.63, 0.12, 1.0],
        [0.83, 0.76, 0.12, 1.0],
        [0.25, 0.89, 0.74, 1.0],
        [0.44, 0.2, 0.09, 1.0],
        [0.02, 0.0, 0.8, 1.0],
        [0.38, 0.36, 0.33, 1.0],
        [0.89, 0.02, 0.8, 1.0],
        [0.32, 0.17, 0.07, 1.0],
        [0.66, 0.95, 0.9, 1.0],
        [0.41, 0.85, 0.63, 1.0],
        [0.69, 0.09, 0.29, 1.0],
        [0.06, 0.93, 0.4, 1.0],
        [0.35, 0.08, 0.99, 1.0],
        [0.78, 0.25, 0.38, 1.0],
        [0.89, 0.81, 0.47, 1.0],
        [0.31, 0.62, 0.63, 1.0],
        [0.01, 0.23, 0.12, 1.0],
        [0.89, 0.1, 0.9, 1.0],
        [0.29, 0.05, 0.37, 1.0],
        [0.71, 0.39, 0.02, 1.0],
        [0.53, 0.1, 0.85, 1.0],
        [0.46, 0.48, 0.33, 1.0],
        [0.45, 0.31, 0.96, 1.0],
        [0.55, 0.91, 0.23, 1.0],
        [0.92, 0.95, 0.62, 1.0],
        [0.72, 0.27, 0.51, 1.0],
        [0.11, 0.01, 0.03, 1.0],
        [0.14, 0.27, 0.05, 1.0],
        [0.55, 0.27, 0.39, 1.0],
        [0.75, 0.96, 0.14, 1.0],
        [0.24, 0.46, 0.7, 1.0],
        [0.41, 0.08, 0.16, 1.0],
        [0.06, 0.02, 0.25, 1.0],
        [0.32, 0.56, 0.71, 1.0],
        [0.2, 0.97, 0.54, 1.0],
        [0.07, 0.05, 0.16, 1.0],
        [0.64, 0.29, 0.46, 1.0],
        [0.5, 0.1, 0.66, 1.0],
        [0.56, 0.19, 0.76, 1.0],
        [0.89, 0.81, 0.62, 1.0],
        [0.21, 0.48, 0.26, 1.0],
        [0.39, 0.29, 0.41, 1.0],
        [0.19, 0.13, 0.18, 1.0],
        [0.5, 0.65, 0.65, 1.0],
        [0.45, 0.12, 0.11, 1.0],
        [0.24, 0.46, 0.99, 1.0],
        [0.59, 0.41, 0.89, 1.0],
        [0.04, 0.27, 0.57, 1.0],
        [0.63, 0.61, 0.7, 1.0],
        [0.6, 0.63, 0.59, 1.0],
        [0.4, 0.81, 0.02, 1.0],
        [0.32, 0.24, 0.01, 1.0],
        [0.87, 0.46, 0.86, 1.0],
        [0.17, 0.02, 0.54, 1.0],
        [0.17, 0.43, 0.36, 1.0],
        [0.08, 0.82, 0.02, 1.0]
    ];

    for vehicle in traffic.vehicles.iter() {
        let cell = city.get_cell(vehicle.location);
        let square = rectangle::square(cell.x as f64 * VEHICLE_SIZE, cell.y as f64 * VEHICLE_SIZE, VEHICLE_SIZE);
        rectangle(COLOURS[vehicle.destination_index % 64], square, context.transform, graphics);

    }
}
