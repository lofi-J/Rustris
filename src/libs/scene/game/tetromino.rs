use crossterm::style::Color;

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

#[derive(Debug, Clone, Copy)]
pub enum TetrominoKind {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    tetromino: TetrominoType,
    kind: TetrominoKind,
}

impl Tetromino {
    /// Generate random tetromino
    pub fn generate_random_tetromino() -> Self {
        let tetrominos: Vec<(TetrominoType, TetrominoKind)> = vec![
            (convert_to_vec(&SHAPE_I), TetrominoKind::I),
            (convert_to_vec(&SHAPE_J), TetrominoKind::J),
            (convert_to_vec(&SHAPE_L), TetrominoKind::L),
            (convert_to_vec(&SHAPE_O), TetrominoKind::O),
            (convert_to_vec(&SHAPE_S), TetrominoKind::S),
            (convert_to_vec(&SHAPE_T), TetrominoKind::T),
            (convert_to_vec(&SHAPE_Z), TetrominoKind::Z),
        ];

        let rand_idx = rand::random_range(0..tetrominos.len());
        let (shape, kind) = tetrominos[rand_idx].clone();

        Self {
            tetromino: shape,
            kind,
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

    /// Get tetromino color based on kind
    pub fn get_color(&self) -> Color {
        match self.kind {
            TetrominoKind::I => Color::Cyan,       // 하늘색
            TetrominoKind::J => Color::Blue,       // 파란색
            TetrominoKind::L => Color::DarkYellow, // 주황색
            TetrominoKind::O => Color::Yellow,     // 노란색
            TetrominoKind::S => Color::Green,      // 초록색
            TetrominoKind::T => Color::Magenta,    // 보라색
            TetrominoKind::Z => Color::Red,        // 빨간색
        }
    }
}

/// Helper function: Convert 2D array to Vec<Vec<bool>>
fn convert_to_vec<const N: usize>(arr: &[[bool; N]; N]) -> TetrominoType {
    arr.iter().map(|row| row.to_vec()).collect()
}
