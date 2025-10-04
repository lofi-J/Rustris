use std::{
    io::{self},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, terminal,
};

use super::tetromino::Tetromino;

type Board = [[bool; 10]; 20]; // 10 * 20 size board

/// controller for tetris game
#[derive(Debug)]
pub struct GameController {
    pub is_game_over: bool,
    pub is_game_pause: bool,
    pub board: Board,
    pub current_tetromino: Tetromino,
    pub preview_tetrominos: Vec<Tetromino>,
    pub tetromino_pos: (u16, u16),
}

impl GameController {
    pub fn new() -> Self {
        println!("GameController Initializing...");
        Self {
            is_game_over: false,
            is_game_pause: false,
            board: [[false; 10]; 20],
            current_tetromino: Tetromino::generate_random_tetromino(),
            preview_tetrominos: vec![
                Tetromino::generate_random_tetromino(),
                Tetromino::generate_random_tetromino(),
            ],
            // board coordinate system start at center top (center of 10x20 board = 3~4 position)
            tetromino_pos: (3, 0),
        }
    }

    /// check game over
    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }

    pub fn handle_input(&mut self) {
        // key input polling (non-blocking)
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Up => self.rotate(),
                    KeyCode::Down => self.move_down(),
                    KeyCode::Left => self.move_left(),
                    KeyCode::Right => self.move_right(),
                    KeyCode::Esc => self.esc_key_input_handler(),
                    _ => {}
                }
            }
        }
    }

    /// reference: https://www.geeksforgeeks.org/inplace-rotate-square-matrix-by-90-degrees/
    /// Tetromino rotate 90 degrees clockwise
    fn rotate(&mut self) {
        let mut block = self.current_tetromino.get_shape();
        let block_len = block.len();

        if block_len == 0 {
            return;
        }

        // first step: transpose (flip along diagonal)
        for i in 0..block_len {
            for j in i + 1..block_len {
                let temp = block[i][j];
                block[i][j] = block[j][i];
                block[j][i] = temp;
            }
        }

        // second step: reverse each row
        for i in 0..block_len {
            block[i].reverse();
        }

        // Save rotated shape back to tetromino
        self.current_tetromino.set_shape(block);
    }

    /// Tetromino move down
    fn move_down(&mut self) {
        if self.tetromino_pos.1 < 19 {
            self.tetromino_pos.1 += 1;
        }
    }

    /// Tetromino move left
    fn move_left(&mut self) {
        if self.tetromino_pos.0 > 0 {
            self.tetromino_pos.0 -= 1;
        }
    }

    /// Tetromino move right
    fn move_right(&mut self) {
        if self.tetromino_pos.0 < 9 {
            self.tetromino_pos.0 += 1;
        }
    }

    /// ESC key input handler
    fn esc_key_input_handler(&mut self) {
        let mut stdout = io::stdout();
        let current_term_status = terminal::is_raw_mode_enabled().unwrap();
        if current_term_status && !self.is_game_over {
            terminal::disable_raw_mode().unwrap();
            execute!(stdout, cursor::Show).unwrap();
            self.is_game_pause = true;
        } else {
            terminal::enable_raw_mode().unwrap();
            execute!(stdout, cursor::Hide).unwrap();
            self.is_game_pause = false;
        }
    }

    /// TODO game logic (falling block, collision detection, game over, update score, current tetromino queue syst)
    pub fn update(&mut self) {
        // tetromino auto falling

        // collision

        // game over

        // update score
    }
}
