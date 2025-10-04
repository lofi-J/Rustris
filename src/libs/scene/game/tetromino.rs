// CONST TETROMINO SHAPE

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

pub type TetrominoType = Vec<Vec<bool>>; // mutable vector

#[derive(Debug, Clone)]
pub struct Tetromino {
    tetromino: TetrominoType,
}

impl Tetromino {
    /// Generate random tetromino
    pub fn generate_random_tetromino() -> Self {
        let tetrominos: Vec<TetrominoType> = vec![
            convert_to_vec(&SHAPE_I),
            convert_to_vec(&SHAPE_J),
            convert_to_vec(&SHAPE_L),
            convert_to_vec(&SHAPE_O),
            convert_to_vec(&SHAPE_S),
            convert_to_vec(&SHAPE_T),
            convert_to_vec(&SHAPE_Z),
        ];

        let rand_idx = rand::random_range(0..tetrominos.len());

        Self {
            tetromino: tetrominos[rand_idx].clone(),
        }
    }

    /// Get current tetromino shape
    pub fn get_shape(&self) -> TetrominoType {
        self.tetromino.clone()
    }

    /// Set tetromino shape (used for rotation)
    pub fn set_shape(&mut self, shape: TetrominoType) {
        self.tetromino = shape;
    }
}

/// Helper function: Convert 2D array to Vec<Vec<bool>>
fn convert_to_vec<const N: usize>(arr: &[[bool; N]; N]) -> TetrominoType {
    arr.iter().map(|row| row.to_vec()).collect()
}
