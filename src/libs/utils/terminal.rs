use std::io::Stdout;

use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};

use crate::libs::state::state::{State, StateController};

/// enable to raw terminal
pub fn enable_raw_mode(state: &mut StateController) {
    if let Err(e) = terminal::enable_raw_mode() {
        eprintln!(
            "Failed to enable raw mode: {}. Make sure you're running in a terminal.",
            e
        );
        state.set_state(State::Exit);
        return;
    }
}

/// terminal clear
pub fn clear_terminal(stdout: &mut Stdout) {
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Hide
    )
    .unwrap();
}

/// get terminal size as (width: u16, height:u16)
pub fn get_terminal_size() -> (u16, u16) {
    let (width, height) = terminal::size().unwrap();

    (width, height)
}

/// 텍스트를 화면 가로 중앙에 배치하기 위한 X 좌표 계산
pub fn center_x(text: &str, terminal_width: u16) -> u16 {
    let text_width = text.chars().count() as u16;
    if terminal_width > text_width {
        (terminal_width - text_width) / 2
    } else {
        0
    }
}
