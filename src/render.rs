use piston_window::{self, Context, G2d, Transformed, polygon, rectangle};
use piston_window::types::Color;
use rand::Rng;

use board::{Board, Cell};
use tetrimino::Shape as MinoShape;

const BORDER_RATIO: f64 = 0.1;
const TOP_LEFT:     [f64; 2] = [0.0, 0.0];
const TOP_RIGHT:    [f64; 2] = [1.0, 0.0];
const BOTTOM_LEFT:  [f64; 2] = [0.0, 1.0];
const BOTTOM_RIGHT: [f64; 2] = [1.0, 1.0];
const LIGHT_RATIO: f32 = 0.6;
const DARK_RATIO:  f32 = 0.6;

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

impl MinoShape {
    fn color(&self) -> Color {
        use tetrimino::Shape as T;

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

impl Cell {
    fn color(&self) -> Color {
        use board::Cell as C;

        match *self {
            C::Empty => BLACK,
            C::Wall => GRAY,
            C::Block(b) => b.color()
        }
    }
}

fn compose_color(c0: Color, c1: Color, c1_alpha: f32) -> Color {
    debug_assert!(0.0 <= c1_alpha && c1_alpha <= 1.0);
    let c0_alpha = 1.0 - c1_alpha;
    [c0[0] * c0_alpha + c1[0] * c1_alpha,
     c0[1] * c0_alpha + c1[1] * c1_alpha,
     c0[2] * c0_alpha + c1[2] * c1_alpha,
     1.0]
}

pub fn render<R: Rng>(ctx: Context, g2d: &mut G2d, scale: f64, board: &Board<R>) {
    let square = rectangle::square(BORDER_RATIO, BORDER_RATIO, 1.0 - BORDER_RATIO * 2.0);

    let vsz = ctx.get_view_size();

    piston_window::clear(Cell::Empty.color(), g2d);

    let deg = board.rotation() as f64 * 90.0 - 180.0;

    let board_trans = ctx.transform
        .trans(vsz[0] / 2.0, vsz[1] / 2.0)
        .scale(scale, scale)
        .rot_deg(deg);

    for ix in 0..board.width() {
        for iy in 0..board.height() {
            let x0 = board.x() as f64 + 0.5;
            let y0 = board.y() as f64 + 0.5;
            let (dx, dy) = (ix as f64 - x0, iy as f64 - y0);
            let transform = board_trans.trans(dx, dy);

            let cell = board[(ix, iy)];
            if cell != Cell::Empty {
                let color = cell.color();
                let light_color = compose_color(color, WHITE, LIGHT_RATIO);
                let dark_color  = compose_color(color, BLACK, DARK_RATIO);

                polygon(light_color, &[TOP_LEFT, BOTTOM_LEFT, TOP_RIGHT], transform, g2d);
                polygon(dark_color, &[BOTTOM_RIGHT, BOTTOM_LEFT, TOP_RIGHT], transform, g2d);
                rectangle(color, square, transform, g2d);
            }
        }
    }
}
