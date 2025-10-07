use std::io;

use crossterm::{event, terminal};

use crate::libs::{
    state::state::{State, StateController},
    utils::terminal::{enable_raw_mode, get_terminal_size},
};

use super::renderer::renderer;

// 게임에 필요한 최소 터미널 크기
const MIN_WIDTH: u16 = 70;
const MIN_HEIGHT: u16 = 25;

pub fn fix_resolution(state: &mut StateController) {
    let mut stdout = io::stdout();
    enable_raw_mode(state);

    // Alternate screen 활성화
    crossterm::execute!(stdout, terminal::EnterAlternateScreen).unwrap();

    // 1단계: 크기 체크 루프
    loop {
        let (current_width, current_height) = get_terminal_size();

        // 터미널 크기 체크
        let width_ok = current_width >= MIN_WIDTH;
        let height_ok = current_height >= MIN_HEIGHT;

        if width_ok && height_ok {
            break;
        }

        // 크기가 부족하면 안내 화면 표시
        let width_diff = if width_ok {
            0
        } else {
            MIN_WIDTH - current_width
        };
        let height_diff = if height_ok {
            0
        } else {
            MIN_HEIGHT - current_height
        };

        renderer(
            &mut stdout,
            current_width,
            current_height,
            width_diff,
            height_diff,
            false,
        );

        // 키 입력 대기 (리사이즈 감지를 위해)
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                if key_event.code == event::KeyCode::Esc {
                    state.set_state(State::Exit);
                    crossterm::execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
                    return;
                }
            }
        }
    }

    // 2단계: 크기가 충족되면 시작 대기 화면
    let (current_width, current_height) = get_terminal_size();
    renderer(&mut stdout, current_width, current_height, 0, 0, true);

    // 사용자 입력 대기
    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    event::KeyCode::Enter | event::KeyCode::Char(' ') => {
                        // Enter나 Space로 게임 시작
                        state.set_state(State::Play);
                        break;
                    }
                    event::KeyCode::Esc => {
                        // ESC로 종료
                        state.set_state(State::Exit);
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    // Alternate screen 비활성화
    crossterm::execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
}
