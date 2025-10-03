use std::io::{self};
use std::time::Duration;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};

use crate::libs::state::state::{State, StateController};
use crate::libs::utils::terminal::enable_raw_mode;

use super::renderer::renderer;

pub fn wellcome(state: &mut StateController) {
    // 터미널 raw 모드 활성화 (키 입력을 직접 처리)
    enable_raw_mode(state);

    let mut stdout = io::stdout(); // get stdout handle

    // 화면 초기화
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
    )
    .unwrap();

    let wellcome_menu_options = vec!["Let's play!", "Check your resolution and fix", "Exit"];
    let mut current_option = 0;
    let mut should_exit = false;

    // 초기 화면 렌더링
    renderer(&mut stdout, &wellcome_menu_options, current_option);

    // 메뉴 선택 이벤트 루프
    while !should_exit {
        // 키 입력 대기
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Up => {
                        // 위로 이동
                        if current_option > 0 {
                            current_option -= 1;
                            renderer(&mut stdout, &wellcome_menu_options, current_option);
                        }
                    }
                    KeyCode::Down => {
                        // 아래로 이동
                        if current_option < wellcome_menu_options.len() - 1 {
                            current_option += 1;
                            renderer(&mut stdout, &wellcome_menu_options, current_option);
                        }
                    }
                    KeyCode::Enter => {
                        // 선택한 메뉴에 따라 상태 변경
                        match current_option {
                            0 => state.set_state(State::Play),          // Let's play!
                            1 => state.set_state(State::FixResolution), // Check your resolution and fix
                            2 => state.set_state(State::Exit),          // Exit
                            _ => {}
                        }
                        should_exit = true;
                    }
                    KeyCode::Esc => {
                        // ESC로 종료
                        state.set_state(State::Exit);
                        should_exit = true;
                    }
                    _ => {}
                }
            }
        }
    }

    // 터미널 정리
    execute!(stdout, cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
}
