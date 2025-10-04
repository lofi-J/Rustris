#[derive(Clone, Debug)]
pub enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

// 테트로미노 형태 상수 정의
const SHAPE_I: [[bool; 4]; 4] = [
    [false, false, false, false],
    [true, true, true, true],
    [false, false, false, false],
    [false, false, false, false],
];

const SHAPE_J: [[bool; 3]; 3] = [
    [true, false, false],
    [true, true, true],
    [false, false, false],
];

const SHAPE_L: [[bool; 3]; 3] = [
    [false, false, true],
    [true, true, true],
    [false, false, false],
];

const SHAPE_O: [[bool; 2]; 2] = [[true, true], [true, true]];

const SHAPE_S: [[bool; 3]; 3] = [
    [false, true, true],
    [true, true, false],
    [false, false, false],
];

const SHAPE_T: [[bool; 3]; 3] = [
    [false, true, false],
    [true, true, true],
    [false, false, false],
];

const SHAPE_Z: [[bool; 3]; 3] = [
    [true, true, false],
    [false, true, true],
    [false, false, false],
];

impl Tetromino {
    pub fn get_random_tetromino() -> Self {
        let tetrominos: Vec<Tetromino> = vec![
            Tetromino::I,
            Tetromino::J,
            Tetromino::L,
            Tetromino::O,
            Tetromino::S,
            Tetromino::T,
            Tetromino::Z,
        ];

        let rand_idx = rand::random_range(0..tetrominos.len());

        tetrominos[rand_idx].clone()
    }
}

impl Tetromino {
    pub fn get_shape(&self) -> Vec<Vec<bool>> {
        match self {
            Tetromino::I => SHAPE_I.iter().map(|row| row.to_vec()).collect(),
            Tetromino::J => SHAPE_J.iter().map(|row| row.to_vec()).collect(),
            Tetromino::L => SHAPE_L.iter().map(|row| row.to_vec()).collect(),
            Tetromino::O => SHAPE_O.iter().map(|row| row.to_vec()).collect(),
            Tetromino::S => SHAPE_S.iter().map(|row| row.to_vec()).collect(),
            Tetromino::T => SHAPE_T.iter().map(|row| row.to_vec()).collect(),
            Tetromino::Z => SHAPE_Z.iter().map(|row| row.to_vec()).collect(),
        }
    }
}
