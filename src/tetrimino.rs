use rand::{Rand, Rng};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Shape {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

pub type Point = (i32, i32);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rotate {
    pub base: Point,
    pub points: [Point; 4],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tetrimino {
    pub shape: Shape,
    pub rotates: [Rotate; 4],
}

impl Rand for Tetrimino {
    fn rand<R: Rng>(rng: &mut R) -> Tetrimino {
        let idx = rng.gen_range(0, TETRIMINO.len());
        TETRIMINO[idx]
    }
}

// rotates[0] が初期配置。 (5, 0) + (x, y) の位置にブロックが現れる
// rotates[0] => rotates[1] は反時計回り方向の回転
// http://www13.plala.or.jp/TETRiS_TGM/kouza/1.htm ここを参考に決定
const TETRIMINO: &'static [Tetrimino] = &[
    Tetrimino {
        shape: Shape::I,
        rotates: [
            Rotate {
                base: (0, 0),
                points: [(-1, 0), (0, 0), (1, 0), (2, 0)],
            },
            Rotate {
                base: (1, 1),
                points: [(1, -1), (1, 0), (1, 1), (1, 2)],
            },
            Rotate {
                base: (1, 0),
                points: [(-1, 0), (0, 0), (1, 0), (2, 0)],
            },
            Rotate {
                base: (1, 0),
                points: [(1, -1), (1, 0), (1, 1), (1, 2)],
            },
        ],
    },
    Tetrimino {
        shape: Shape::O,
        rotates: [
            Rotate {
                base: (0, 0),
                points: [(0, 0), (1, 0), (0, 1), (1, 1)],
            },
            Rotate {
                base: (0, 1),
                points: [(0, 0), (1, 0), (0, 1), (1, 1)],
            },
            Rotate {
                base: (1, 1),
                points: [(0, 0), (1, 0), (0, 1), (1, 1)],
            },
            Rotate {
                base: (1, 0),
                points: [(0, 0), (1, 0), (0, 1), (1, 1)],
            },
        ],
    },
    Tetrimino {
        shape: Shape::S,
        rotates: [
            Rotate {
                base: (0, 0),
                points: [(-1, 1), (0, 1), (0, 0), (1, 0)],
            },
            Rotate {
                base: (-1, 0),
                points: [(0, 1), (0, 0), (-1, 0), (-1, -1)],
            },
            Rotate {
                base: (0, 1),
                points: [(-1, 1), (0, 1), (0, 0), (1, 0)],
            },
            Rotate {
                base: (0, 0),
                points: [(0, 1), (0, 0), (-1, 0), (-1, -1)],
            },
        ],
    },
    Tetrimino {
        shape: Shape::Z,
        rotates: [
            Rotate {
                base: (0, 0),
                points: [(-1, 0), (0, 0), (0, 1), (1, 1)],
            },
            Rotate {
                base: (0, 0),
                points: [(0, 1), (0, 0), (1, 0), (1, -1)],
            },
            Rotate {
                base: (0, 1),
                points: [(-1, 0), (0, 0), (0, 1), (1, 1)],
            },
            Rotate {
                base: (1, 0),
                points: [(0, 1), (0, 0), (1, 0), (1, -1)],
            },
        ],
    },
    Tetrimino {
        shape: Shape::J,
        rotates: [
            Rotate {
                base: (0, 0),
                points: [(-1, 0), (0, 0), (1, 0), (1, 1)],
            },
            Rotate {
                base: (0, 0),
                points: [(0, 1), (0, 0), (0, -1), (1, -1)],
            },
            Rotate {
                base: (0, 1),
                points: [(-1, 0), (-1, 1), (0, 1), (1, 1)],
            },
            Rotate {
                base: (0, 0),
                points: [(-1, 1), (0, 1), (0, 0), (0, -1)],
            },
        ],
    },
    Tetrimino {
        shape: Shape::L,
        rotates: [
            Rotate {
                base: (0, 0),
                points: [(-1, 1), (-1, 0), (0, 0), (1, 0)],
            },
            Rotate {
                base: (0, 0),
                points: [(1, 1), (0, 1), (0, 0), (0, -1)],
            },
            Rotate {
                base: (0, 1),
                points: [(-1, 1), (0, 1), (1, 1), (1, 0)],
            },
            Rotate {
                base: (0, 0),
                points: [(0, 1), (0, 0), (0, -1), (-1, -1)],
            },
        ],
    },
    Tetrimino {
        shape: Shape::T,
        rotates: [
            Rotate {
                base: (0, 0),
                points: [(-1, 0), (0, 0), (1, 0), (0, 1)],
            },
            Rotate {
                base: (0, 0),
                points: [(0, 1), (0, 0), (0, -1), (1, 0)],
            },
            Rotate {
                base: (0, 1),
                points: [(-1, 1), (0, 1), (1, 1), (0, 0)],
            },
            Rotate {
                base: (0, 0),
                points: [(0, 1), (0, 0), (0, -1), (-1, 0)],
            },
        ],
    },
];
