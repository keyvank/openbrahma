use glutin_window::GlutinWindow as Window;
use graphics::*;
use openbrahma::geometry::Vector;
use openbrahma::World;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const SCALE: f64 = 3.0;

pub struct App<'a> {
    gl: GlGraphics,
    world: &'a mut World,
}

impl<'a> App<'a> {
    fn render(&mut self, args: &RenderArgs) {
        let circs = self
            .world
            .objects
            .values()
            .map(|o| {
                (
                    o.trans.trans * SCALE,
                    o.body.shape().bounding_circle().r * SCALE,
                )
            })
            .collect::<Vec<_>>();

        self.gl.draw(args.viewport(), |c, gl| {
            let center = Vector(args.window_size[0] / 2.0, args.window_size[1] / 2.0);

            clear(BLACK, gl);

            for (p, r) in circs {
                let square = rectangle::square(0.0, 0.0, r);
                let pos = center + p;
                let transform = c.transform.trans(pos.0, pos.1).trans(-r / 2.0, -r / 2.0);
                ellipse(WHITE, square, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.world.update();
    }
}

pub fn simulate<'a>(world: &'a mut World) {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Open Brahma", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        world: world,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
