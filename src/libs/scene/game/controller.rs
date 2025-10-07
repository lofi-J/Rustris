use std::{
    fs::OpenOptions,
    io::{self, Write},
    time::{Duration, Instant},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, terminal,
};

use super::tetromino::Tetromino;

type Board = Vec<Vec<bool>>;

enum TetrominoAction {
    MoveDown,
    MoveLeft,
    MoveRight,
    Rotate,
    Drop,
}

/// controller for tetris game
pub struct GameController {
    pub is_game_over: bool,
    pub is_game_pause: bool,
    pub board: Board,
    pub current_tetromino: Tetromino,
    pub preview_tetrominos: Vec<Tetromino>,
    pub tetromino_pos: (u16, u16),
    last_drop_time: Instant,
    drop_interval: Duration,
}

impl GameController {
    pub fn new() -> Self {
        Self {
            is_game_over: false,
            is_game_pause: false,
            board: vec![vec![false; 10]; 20],
            current_tetromino: Tetromino::generate_random_tetromino(),
            preview_tetrominos: vec![
                Tetromino::generate_random_tetromino(),
                Tetromino::generate_random_tetromino(),
            ],
            // board coordinate system start at center top (center of 10x20 board = 3~4 position)
            tetromino_pos: (3, 0),
            last_drop_time: Instant::now(),
            drop_interval: Duration::from_millis(500), // 0.5초마다 자동 낙하
        }
    }

    /// debug log TODO remove
    fn debug_log(&self, message: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("game_debug.log")
        {
            let _ = writeln!(file, "{}", message);
        }
    }

    /// check game over
    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }

    pub fn handle_input(&mut self) {
        // key input polling (non-blocking, 10ms timeout)
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Up => {
                        self.rotate();
                    }
                    KeyCode::Down => {
                        self.move_down();
                    }
                    KeyCode::Left => {
                        self.move_left();
                    }
                    KeyCode::Right => {
                        self.move_right();
                    }
                    KeyCode::Esc => {
                        self.esc_key_input_handler();
                    }
                    KeyCode::Char(' ') => {
                        self.hard_drop();
                    }
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
        let current_term_raw_mode = terminal::is_raw_mode_enabled().unwrap();

        if current_term_raw_mode && !self.is_game_over {
            terminal::disable_raw_mode().unwrap();
            execute!(stdout, cursor::Show).unwrap();
            self.is_game_pause = true;
        } else {
            terminal::enable_raw_mode().unwrap();
            execute!(stdout, cursor::Hide).unwrap();
            self.is_game_pause = false;
        }
    }

    /// Hard drop
    pub fn hard_drop(&mut self) {
        self.is_collision(TetrominoAction::Drop);
    }

    /// collision detection
    fn is_collision(&self, action: TetrominoAction) -> bool {
        let tetromino = self.current_tetromino.get_shape();
        let board = self.board.clone();

        self.debug_log(&format!("board: {:?}", board));

        // first move tetromino and check collision
        // let (tetromino_x, tetromino_y) = self.tetromino_pos;

        false
    }

    /// 게임 상태 업데이트 (자동 낙하 처리)
    pub fn update(&mut self) {
        // 게임이 일시정지 상태면 업데이트하지 않음
        if self.is_game_pause {
            return;
        }

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_drop_time);

        // 설정된 시간 간격이 지났으면 테트로미노를 아래로 이동
        if elapsed >= self.drop_interval {
            self.move_down();
            self.last_drop_time = now;
        }
    }

    /// 낙하 속도 설정 (밀리초 단위)
    pub fn set_drop_interval(&mut self, millis: u64) {
        self.drop_interval = Duration::from_millis(millis);
    }
}
