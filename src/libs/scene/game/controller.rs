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

/// controller for tetris game
pub struct GameController {
    pub is_game_over: bool,
    pub is_game_pause: bool,
    pub board: Board,
    pub current_tetromino: Tetromino,
    pub preview_tetrominos: Vec<Tetromino>,
    pub tetromino_pos: (i32, i32), // i32로 변경하여 음수 좌표 지원
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

        // 회전 후 충돌 체크
        let (x, y) = self.tetromino_pos;
        if !self.check_collision(&block, x, y) {
            // 충돌이 없으면 회전 적용
            self.current_tetromino.set_shape(block);
        }
        // 충돌이 있으면 회전하지 않음 (원래 상태 유지)
    }

    /// Tetromino move down
    fn move_down(&mut self) {
        let shape = self.current_tetromino.get_shape();
        let (x, y) = self.tetromino_pos;
        let new_y = y + 1;

        // 아래로 이동 가능한지 체크
        if !self.check_collision(&shape, x, new_y) {
            self.tetromino_pos.1 = new_y;
        } else {
            // 더 이상 아래로 이동할 수 없으면 고정
            self.lock_tetromino();
            self.clear_lines();
            self.spawn_new_tetromino();
        }
    }

    /// Tetromino move left
    fn move_left(&mut self) {
        let shape = self.current_tetromino.get_shape();
        let (x, y) = self.tetromino_pos;
        let new_x = x - 1;

        // 왼쪽으로 이동 가능한지 체크
        if !self.check_collision(&shape, new_x, y) {
            self.tetromino_pos.0 = new_x;
        }
    }

    /// Tetromino move right
    fn move_right(&mut self) {
        let shape = self.current_tetromino.get_shape();
        let (x, y) = self.tetromino_pos;
        let new_x = x + 1;

        // 오른쪽으로 이동 가능한지 체크
        if !self.check_collision(&shape, new_x, y) {
            self.tetromino_pos.0 = new_x;
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

    /// Hard drop - 테트로미노를 즉시 바닥까지 떨어뜨림
    pub fn hard_drop(&mut self) {
        let shape = self.current_tetromino.get_shape();
        let (x, mut y) = self.tetromino_pos;

        // 충돌할 때까지 계속 아래로 이동
        while !self.check_collision(&shape, x, y + 1) {
            y += 1;
        }

        self.tetromino_pos.1 = y;

        // 즉시 고정
        self.lock_tetromino();
        self.clear_lines();
        self.spawn_new_tetromino();
    }

    /// 충돌 감지 - 특정 위치에 테트로미노를 놓을 수 있는지 확인
    fn check_collision(&self, shape: &[Vec<bool>], x: i32, y: i32) -> bool {
        for (row_idx, row) in shape.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell {
                    let board_x = x + col_idx as i32;
                    let board_y = y + row_idx as i32;

                    // 보드 경계를 벗어나는지 체크
                    if board_x < 0 || board_x >= 10 || board_y < 0 || board_y >= 20 {
                        return true; // 충돌
                    }

                    // 보드에 이미 블록이 있는지 체크
                    if self.board[board_y as usize][board_x as usize] {
                        return true; // 충돌
                    }
                }
            }
        }
        false // 충돌 없음
    }

    /// 테트로미노를 보드에 고정
    fn lock_tetromino(&mut self) {
        let shape = self.current_tetromino.get_shape();
        let (x, y) = self.tetromino_pos;

        for (row_idx, row) in shape.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell {
                    let board_x = x + col_idx as i32;
                    let board_y = y + row_idx as i32;

                    if board_x >= 0 && board_x < 10 && board_y >= 0 && board_y < 20 {
                        self.board[board_y as usize][board_x as usize] = true;
                    }
                }
            }
        }
    }

    /// 라인 클리어 체크 및 처리
    fn clear_lines(&mut self) {
        let mut lines_to_clear = Vec::new();

        // 완성된 라인 찾기
        for (idx, row) in self.board.iter().enumerate() {
            if row.iter().all(|&cell| cell) {
                lines_to_clear.push(idx);
            }
        }

        // 라인 제거 및 위에서 아래로 블록 내리기
        for &line_idx in lines_to_clear.iter().rev() {
            self.board.remove(line_idx);
            self.board.insert(0, vec![false; 10]);
        }
    }

    /// 새로운 테트로미노 생성
    fn spawn_new_tetromino(&mut self) {
        // preview의 첫 번째 테트로미노를 현재 테트로미노로
        self.current_tetromino = self.preview_tetrominos.remove(0);
        // 새로운 preview 추가
        self.preview_tetrominos
            .push(Tetromino::generate_random_tetromino());
        // 위치 초기화
        self.tetromino_pos = (3, 0);

        // 게임 오버 체크: 새로 생성된 위치에서 이미 충돌하면 게임 오버
        let shape = self.current_tetromino.get_shape();
        if self.check_collision(&shape, self.tetromino_pos.0, self.tetromino_pos.1) {
            self.is_game_over = true;
        }
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
