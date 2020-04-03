use glutin_window::GlutinWindow as Window;
use graphics::*;
use openbrahma::World;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

pub struct App<'a> {
    gl: GlGraphics,
    world: &'a mut World,
}

impl<'a> App<'a> {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let square = rectangle::square(0.0, 0.0, 50.0);
            let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
            let transform = c.transform.trans(x, y).trans(-25.0, -25.0);
            rectangle(WHITE, square, transform, gl);
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
