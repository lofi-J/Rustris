use std::io::{self, Write};
use std::time::Duration;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};

use crate::libs::state::state::{State, StateController};
use crate::libs::utils::terminal::{center_x, clear_terminal, enable_raw_mode, get_terminal_size};

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
    render_wellcome_screen(&mut stdout, &wellcome_menu_options, current_option);

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
                            render_wellcome_screen(
                                &mut stdout,
                                &wellcome_menu_options,
                                current_option,
                            );
                        }
                    }
                    KeyCode::Down => {
                        // 아래로 이동
                        if current_option < wellcome_menu_options.len() - 1 {
                            current_option += 1;
                            render_wellcome_screen(
                                &mut stdout,
                                &wellcome_menu_options,
                                current_option,
                            );
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

fn render_wellcome_screen(stdout: &mut io::Stdout, options: &[&str], current_option: usize) {
    // 커서를 화면 맨 위로 이동하고 화면 지우기
    clear_terminal(stdout);

    // 터미널 크기 가져오기
    let (terminal_width, terminal_height) = get_terminal_size();

    // RUSTRIS ASCII 아트
    let logo = vec![
        "██████╗ ██╗   ██╗███████╗████████╗██████╗ ██╗███████╗",
        "██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔══██╗██║██╔════╝",
        "██████╔╝██║   ██║███████╗   ██║   ██████╔╝██║███████╗",
        "██╔══██╗██║   ██║╚════██║   ██║   ██╔══██╗██║╚════██║",
        "██║  ██║╚██████╔╝███████║   ██║   ██║  ██║██║███████║",
        "╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝╚══════╝",
    ];

    // 화면 세로 중앙 계산 (로고 + 메뉴 + 여백)
    let total_content_height = logo.len() + 3 + options.len(); // 로고 + 공백 + 메뉴
    let start_y = if terminal_height as usize > total_content_height {
        (terminal_height as usize - total_content_height) / 2
    } else {
        1
    };

    // 로고 출력 (가로 중앙 정렬)
    for (i, line) in logo.iter().enumerate() {
        let x = center_x(line, terminal_width);
        execute!(stdout, cursor::MoveTo(x, (start_y + i) as u16)).unwrap();
        write!(stdout, "{}", line).unwrap();
    }

    // 메뉴 제목
    let menu_start_y = start_y + logo.len() + 2;

    // 메뉴 옵션 출력 (가로 중앙 정렬)
    for (index, option) in options.iter().enumerate() {
        let menu_text = if index == current_option {
            format!("▶ {}", option)
        } else {
            format!("  {}", option)
        };

        let x = center_x(&menu_text, terminal_width);
        let y = (menu_start_y + index) as u16;

        execute!(stdout, cursor::MoveTo(x, y)).unwrap();
        write!(stdout, "{}", menu_text).unwrap();
    }

    // 팁 메시지 출력 (가로 중앙 정렬)
    let tip_text = "Tip: Use ↑ ↓ to navigate, Enter to select, ESC to exit";
    let tip_x = center_x(tip_text, terminal_width);
    let tip_y = (menu_start_y + options.len() + 2) as u16;

    execute!(stdout, cursor::MoveTo(tip_x, tip_y)).unwrap();
    write!(stdout, "{}", tip_text).unwrap();

    stdout.flush().unwrap(); // 직접 flush해 출력
}
