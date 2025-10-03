use crossterm::{cursor, execute};
use std::io::{Stdout, Write};

use crate::libs::utils::terminal::{center_x, clear_terminal, get_terminal_size};

pub fn renderer(stdout: &mut Stdout, options: &[&str], current_option: usize) {
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
