extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use noise::{NoiseFn, OpenSimplex};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 600.0;

const CELL_SIZE: f64 = 10.0;

const NOISE_STEP: f64 = 0.2;
const Z_STEP: f64 = 1.0;

pub struct App {
    gl: GlGraphics,
    z: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.4, 0.4, 0.4, 1.0];
        const LINE_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

        let z = self.z.clone();
        self.gl.draw(args.viewport(), |c, gl| {
            let noise = OpenSimplex::new();

            clear(BACKGROUND, gl);

            for x in 0..(WIDTH / CELL_SIZE) as i64 {
                for y in 0..(HEIGHT / CELL_SIZE) as i64 {
                    let transform = c
                        .transform
                        .trans(x as f64 * CELL_SIZE, y as f64 * CELL_SIZE);

                    let dots = vec![
                        noise
                            .get([x as f64 * NOISE_STEP, y as f64 * NOISE_STEP, z])
                            .signum() as i64,
                        noise
                            .get([(x + 1) as f64 * NOISE_STEP, y as f64 * NOISE_STEP, z])
                            .signum() as i64,
                        noise
                            .get([(x + 1) as f64 * NOISE_STEP, (y + 1) as f64 * NOISE_STEP, z])
                            .signum() as i64,
                        noise
                            .get([x as f64 * NOISE_STEP, (y + 1) as f64 * NOISE_STEP, z])
                            .signum() as i64,
                    ];

                    let num = square_to_num(dots);
                    let lines = match num {
                        1 | 14 => vec![[0.0, CELL_SIZE / 2.0, CELL_SIZE / 2.0, CELL_SIZE]],
                        2 | 13 => vec![[CELL_SIZE / 2.0, CELL_SIZE, CELL_SIZE, CELL_SIZE / 2.0]],
                        3 | 12 => vec![[0.0, CELL_SIZE / 2.0, CELL_SIZE, CELL_SIZE / 2.0]],
                        4 => vec![[CELL_SIZE / 2.0, 0.0, CELL_SIZE, CELL_SIZE / 2.0]],
                        5 => vec![
                            [CELL_SIZE / 2.0, CELL_SIZE, CELL_SIZE, CELL_SIZE / 2.0],
                            [0.0, CELL_SIZE / 2.0, CELL_SIZE / 2.0, 0.0],
                        ],
                        6 | 9 => vec![[CELL_SIZE / 2.0, 0.0, CELL_SIZE / 2.0, CELL_SIZE]],
                        7 | 8 => vec![[0.0, CELL_SIZE / 2.0, CELL_SIZE / 2.0, 0.0]],
                        10 => vec![
                            [0.0, CELL_SIZE / 2.0, CELL_SIZE / 2.0, CELL_SIZE],
                            [CELL_SIZE / 2.0, 0.0, CELL_SIZE, CELL_SIZE / 2.0],
                        ],
                        11 => vec![[CELL_SIZE / 2.0, 0.0, CELL_SIZE, CELL_SIZE / 2.0]],
                        _ => vec![],
                    };

                    for l in lines {
                        line(LINE_COLOR, 1.0, l, transform, gl);
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.z += args.dt * Z_STEP;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("marching squares", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        z: 0.0,
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

fn square_to_num(square: Vec<i64>) -> i64 {
    let mut sum = 0;
    for i in (0..square.len()).rev() {
        if square[square.len() - 1 - i] == 1 {
            sum += (2 as i64).pow((i) as u32);
        }
    }
    sum
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_to_num() {
        let one = vec![0, 0, 0, 1];
        let three = vec![0, 0, 1, 1];

        assert_eq!(square_to_num(one), 1);
        assert_eq!(square_to_num(three), 3);
    }
}
