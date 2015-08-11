#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

extern crate piston_window;
extern crate rand;
extern crate time;

use piston_window::{PistonWindow, WindowSettings,
                    PressEvent, Button, Key,
                    UpdateEvent,
                    RenderEvent};

mod board;
mod tetrimino;
mod render;

use board::Board;

fn main() {
    const CELL_SIZE: f64 = 20.0;

    let mut rng = rand::thread_rng();
    let mut board = Board::new(&mut rng);

    let window_width: u32 = (CELL_SIZE as u32) * (board.height() as u32) * 2;
    let window_height: u32 = (CELL_SIZE as u32) * (board.height() as u32) * 2;

    let window: PistonWindow = WindowSettings::new("FPT", [window_width, window_height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    for e in window {
        if let Some(_args) = e.render_args() {
            e.draw_2d(|ctx, g2d| render::render(ctx, g2d, CELL_SIZE, &board));
        }

        if let Some(_args) = e.update_args() {
            board.fall(1000);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up    | Key::W | Key::K => { board.advance(true); }
                Key::Down  | Key::S | Key::J => { board.advance(false); }
                Key::Left  | Key::A | Key::H => { board.rotate(true); }
                Key::Right | Key::D | Key::L => { board.rotate(false); }
                Key::Space => { board.fall(0); }
                _ => {}
            }
        }
    }
}
