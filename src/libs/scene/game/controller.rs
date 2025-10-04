use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use super::tetromino::Tetromino;

type Board = [[bool; 10]; 20]; // 10 * 20 size board

/// controller for tetris game
#[derive(Debug)]
pub struct GameController {
    pub is_game_over: bool,
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
            board: [[false; 10]; 20],
            current_tetromino: Tetromino::get_random_tetromino(),
            preview_tetrominos: vec![
                Tetromino::get_random_tetromino(),
                Tetromino::get_random_tetromino(),
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
                    KeyCode::Esc => self.is_game_over = true,
                    _ => {}
                }
            }
        }
    }

    /// Tetromino rotate
    fn rotate(&mut self) {
        println!("Rotate");
        // TODO: rotate logic
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

    /// TODO game logic (falling block, collision detection, game over, update score)
    pub fn update(&mut self) {
        // tetromino auto falling

        // collision

        // game over

        // update score
    }
}
