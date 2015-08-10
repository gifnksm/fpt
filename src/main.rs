#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings,
                    PressEvent, Button, Key,
                    UpdateEvent,
                    RenderEvent, Transformed, polygon, rectangle};
use piston_window::types::Color;

const FIELD_WIDTH: usize  = 12;
const FIELD_HEIGHT: usize = 22;

const BLACK:   Color = [0.0, 0.0, 0.0, 1.0];
const WHITE:   Color = [1.0, 1.0, 1.0, 1.0];
const GRAY:    Color = [0.5, 0.5, 0.5, 1.0];

const CYAN:    Color = [0.0, 1.0, 1.0, 1.0];
const YELLOW:  Color = [1.0, 1.0, 0.0, 1.0];
const LIME:    Color = [0.0, 1.0, 0.0, 1.0];
const RED:     Color = [1.0, 0.0, 0.0, 1.0];
const BLUE:    Color = [0.0, 0.0, 1.0, 1.0];
const ORANGE:  Color = [1.0, 0.5, 0.0, 1.0];
const MAGENTA: Color = [1.0, 0.0, 1.0, 1.0];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tetrimino {
    I, O, S, Z, J, L, T
}

impl Tetrimino {
    fn color(&self) -> Color {
        use Tetrimino as T;

        match *self {
            T::I => CYAN,
            T::O => YELLOW,
            T::S => LIME,
            T::Z => RED,
            T::J => BLUE,
            T::L => ORANGE,
            T::T => MAGENTA
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Block(Tetrimino)
}

impl Cell {
    fn color(&self) -> Color {
        use Cell as C;

        match *self {
            C::Empty => BLACK,
            C::Wall => GRAY,
            C::Block(b) => b.color()
        }
    }
}

fn main() {
    const BORDER_RATIO: f64 = 0.1;
    const CELL_SIZE:    f64 = 20.0;
    const TOP_LEFT:     [f64; 2] = [0.0, 0.0];
    const TOP_RIGHT:    [f64; 2] = [1.0, 0.0];
    const BOTTOM_LEFT:  [f64; 2] = [0.0, 1.0];
    const BOTTOM_RIGHT: [f64; 2] = [1.0, 1.0];
    const LIGHT_RATIO: f32 = 0.6;
    const DARK_RATIO:  f32 = 0.6;

    const WINDOW_WIDTH: u32 = (CELL_SIZE as u32) * ((FIELD_HEIGHT + 2) as u32) * 2;
    const WINDOW_HEIGHT: u32 = (CELL_SIZE as u32) * ((FIELD_HEIGHT + 2) as u32) * 2;

    let window: PistonWindow = WindowSettings::new("FPT", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut deg = 180.0;
    let mut x0 = (FIELD_WIDTH as f64) / 2.0;
    let mut y0 = 0.0;
    let mut xdir = 1.0;

    for e in window {
        if let Some(_args) = e.render_args() {
            let square = rectangle::square(BORDER_RATIO, BORDER_RATIO, 1.0 - BORDER_RATIO * 2.0);

            e.draw_2d(|ctx, gl| {
                let vsz = ctx.get_view_size();

                piston_window::clear(Cell::Empty.color(), gl);

                let board_trans = ctx.transform
                    .trans(vsz[0] / 2.0, vsz[1] / 2.0)
                    .scale(CELL_SIZE, CELL_SIZE)
                    .rot_deg(deg);

                for ix in 0..FIELD_WIDTH+2 {
                    for iy in 0..FIELD_HEIGHT+2 {
                        let (dx, dy) = (ix as f64 - (x0 as usize as f64), iy as f64 - (y0 as usize as f64));

                        let transform = board_trans.trans(dx, dy);

                        let cell = if ix == (x0 as usize) && iy == (y0 as usize) {
                            Cell::Block(Tetrimino::S)
                        } else if ix == 0 || ix == FIELD_WIDTH + 1 || iy == FIELD_HEIGHT + 1 {
                            Cell::Wall
                        } else if ix == 1 {
                            Cell::Block(Tetrimino::T)
                        } else {
                            Cell::Empty
                        };

                        if cell != Cell::Empty {
                            let color = cell.color();
                            let light_color = [1.0 * LIGHT_RATIO + color[0] * (1.0 - LIGHT_RATIO),
                                               1.0 * LIGHT_RATIO + color[1] * (1.0 - LIGHT_RATIO),
                                               1.0 * LIGHT_RATIO + color[2] * (1.0 - LIGHT_RATIO),
                                               1.0];
                            polygon(light_color, &[TOP_LEFT, BOTTOM_LEFT, TOP_RIGHT], transform, gl);

                            let dark_color = [0.0 * DARK_RATIO + color[0] * (1.0 - DARK_RATIO),
                                              0.0 * DARK_RATIO + color[1] * (1.0 - DARK_RATIO),
                                              0.0 * DARK_RATIO + color[2] * (1.0 - DARK_RATIO),
                                              1.0];
                            polygon(dark_color, &[BOTTOM_RIGHT, BOTTOM_LEFT, TOP_RIGHT], transform, gl);

                            rectangle(color, square, transform, gl);
                        }
                    }
                }
            });
        }

        if let Some(args) = e.update_args() {
            deg += 0.0 * args.dt;
            x0 += 1.0 * xdir * args.dt;
            y0 += 1.0 * args.dt;
            if x0 < 1.0 {
                xdir = 1.0;
                x0 = 1.0
            } else if x0 > (FIELD_WIDTH + 1) as f64 {
                xdir = -1.0;
                x0 = (FIELD_WIDTH + 1) as f64;
            }
            if y0 > (FIELD_HEIGHT + 1) as f64 {
                y0 = 0.0;
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Left => { deg += 90.0; }
                Key::Right => { deg -= 90.0; }
                _ => {}
            }
        }
    }
}
