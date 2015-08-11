use std::ops::Index;
use rand::Rng;
use time::{self, Tm};

use tetrimino::Shape as MinoShape;
use tetrimino::{Tetrimino, Point};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Wall,
    Block(MinoShape)
}

impl Default for Cell {
    fn default() -> Cell { Cell::Empty }
}

const BOARD_WIDTH: usize  = 12;
const BOARD_HEIGHT: usize = 20;

#[derive(Copy, Clone, Debug)]
struct Moving {
    tetrimino: Tetrimino,
    rotation: i32,
    x: i32,
    y: i32
}

impl Moving {
    fn new_random<R: Rng>(rng: &mut R, x0: i32, y0: i32) -> Moving {
        Moving {
            tetrimino: rng.gen(),
            rotation: 0,
            x: x0, y: y0
        }
    }

    fn advance(mut self, is_forward: bool) -> Moving {
        match (self.rotation, is_forward) {
            (0, true) | (2, false) => self.y += 1,
            (1, true) | (3, false) => self.x += 1,
            (2, true) | (0, false) => self.y -= 1,
            (3, true) | (1, false) => self.x -= 1,
            _ => unreachable!()
        }
        self
    }

    fn fall(mut self) -> Moving {
        self.y += 1;
        self
    }

    fn rotate(mut self, is_acw: bool) -> Moving {
        let tet = self.tetrimino;
        if is_acw {
            self.rotation += 1;
        } else {
            self.rotation -= 1;
        }
        let num_rotates = tet.rotates.len() as i32;
        if self.rotation < 0 {
            self.rotation += num_rotates;
        }
        if self.rotation >= num_rotates {
            self.rotation -= num_rotates;
        }
        self
    }

    fn shape(self) -> MinoShape {
        self.tetrimino.shape
    }
    fn base(self) -> Point {
        self.tetrimino.rotates[self.rotation as usize].base
    }
    fn points(self) -> [Point; 4] {
        self.tetrimino.rotates[self.rotation as usize].points
    }
}

type Matrix = [[Cell; BOARD_HEIGHT]; BOARD_WIDTH];
const EMPTY_MATRIX: Matrix = [[Cell::Empty; BOARD_HEIGHT]; BOARD_WIDTH];

#[derive(Debug)]
pub struct Board<'a, R: Rng + 'a> {
    rng: &'a mut R,
    moving: Option<Moving>,
    fixed_cells: Matrix,
    merged_cells: Matrix,
    last_fall: Tm,
    finished: bool
}

impl<'a, R: Rng> Board<'a, R> {
    pub fn new(rng: &'a mut R) -> Board<'a, R> {
        let mut mat = EMPTY_MATRIX;
        for y in 0..BOARD_HEIGHT {
            mat[0][y] = Cell::Wall;
            mat[BOARD_WIDTH - 1][y] = Cell::Wall;
        }
        for x in 1..BOARD_WIDTH-1 {
            mat[x][BOARD_HEIGHT - 1] = Cell::Wall;
        }

        Board {
            rng: rng,
            moving: None,
            fixed_cells: mat,
            merged_cells: mat,
            last_fall: time::now(),
            finished: false
        }
    }

    pub fn rotate(&mut self, is_acw: bool) {
        if let Some(mv) = self.moving {
            let new_mv = mv.rotate(is_acw);
            if self.can_locate(new_mv) {
                self.moving = Some(new_mv);
            }
        }

        self.merge_boards();
    }

    pub fn advance(&mut self, is_forward: bool) {
        if let Some(mv) = self.moving {
            let new_mv = mv.advance(is_forward);
            if self.can_locate(new_mv) {
                self.moving = Some(new_mv);
            }
        }

        self.merge_boards();
    }

    pub fn fall(&mut self, thresh_in_msec: i64) {
        let now = time::now();
        if (now - self.last_fall).num_milliseconds() < thresh_in_msec {
            return
        }
        self.last_fall = now;

        if let Some(mv) = self.moving {
            let new_mv = mv.fall();
            if !self.can_locate(new_mv) {
                // fixed
                self.merge_boards();
                self.fixed_cells = self.merged_cells;
                self.moving = None;
                self.clear_fulfilled();
                self.merged_cells = self.fixed_cells;
                return
            }

            self.moving = Some(new_mv);
        } else {
            let x = self.width() / 2;
            let new_mv = Moving::new_random(self.rng, x, 0);
            if self.can_locate(new_mv) {
                self.moving = Some(new_mv);
            } else {
                self.finished = true;
            }
        }

        self.merge_boards();
    }

    pub fn width(&self) -> i32 { BOARD_WIDTH as i32 }
    pub fn height(&self) -> i32 { BOARD_HEIGHT as i32 }
    pub fn rotation(&self) -> i32 {
        match self.moving {
            Some(mv) => mv.rotation,
            None => 0
        }
    }
    pub fn x(&self) -> i32 {
        match self.moving {
            Some(mv) => mv.x + mv.base().0,
            None => self.width() / 2
        }
    }
    pub fn y(&self) -> i32 {
        match self.moving {
            Some(mv) => mv.y + mv.base().1,
            None => 0
        }
    }

    fn merge_boards(&mut self) {
        self.merged_cells = self.fixed_cells;

        if let Some(mv) = self.moving {
            for &(dx, dy) in &mv.points() {
                let x = mv.x + dx;
                let y = mv.y + dy;
                if y < 0 { continue }
                debug_assert_eq!(self.merged_cells[x as usize][y as usize], Cell::Empty);
                self.merged_cells[x as usize][y as usize] = Cell::Block(mv.shape());
            }
        }
    }
    fn can_locate(&self, mv: Moving) -> bool {
        if mv.y < 0 { return false }

        mv.points()
            .iter()
            .all(|&(dx, dy)| {
                let x = mv.x + dx;
                let y = mv.y + dy;
                if x < 0 || x >= self.width() { return false }
                if y < 0 { return true }
                if y >= self.height() { return true }
                self.fixed_cells[x as usize][y as usize] == Cell::Empty
            })
    }
    fn clear_fulfilled(&mut self) {
        let mut y_dst = self.height() - 2;
        for y_src in (0..self.height()-1).rev() {
            let filled = (1..self.width() - 1).all(|x| {
                let cell = self.fixed_cells[x as usize][y_src as usize];
                debug_assert!(cell != Cell::Wall);
                cell != Cell::Empty
            });

            if !filled {
                for x in 1..self.width() - 1 {
                    self.fixed_cells[x as usize][y_dst as usize] =
                        self.fixed_cells[x as usize][y_src as usize];
                }
                y_dst -= 1;
            }
        }
        for y in 0..y_dst+1 {
            for x in 1..self.width()-1 {
                self.fixed_cells[x as usize][y as usize] = Cell::Empty;
            }
        }
    }
}

impl<'a, R: Rng> Index<Point> for Board<'a, R> {
    type Output = Cell;
    fn index<'b>(&'b self, (x, y): Point) -> &'b Cell {
        return &self.merged_cells[x as usize][y as usize]
    }
}
