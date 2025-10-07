use std::io::{Stdout, Write};

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
};

use crate::libs::utils::terminal::clear_terminal;

/// 터미널 크기가 충족되었을 때 시작 대기 화면
fn render_ready_screen(stdout: &mut Stdout, current_width: u16, center_y: u16) {
    // 제목
    let title = "✅ 터미널 준비 완료!";
    let title_x = if current_width > title.len() as u16 {
        (current_width - title.len() as u16) / 2
    } else {
        0
    };

    execute!(
        stdout,
        cursor::MoveTo(title_x, center_y),
        SetForegroundColor(Color::Green),
        Print(title.bold()),
        ResetColor
    )
    .unwrap();

    // 안내 메시지 1
    let msg1 = "게임을 시작할 준비가 되었습니다";
    let msg1_x = if current_width > msg1.len() as u16 {
        (current_width - msg1.len() as u16) / 2
    } else {
        0
    };

    execute!(stdout, cursor::MoveTo(msg1_x, center_y + 2), Print(msg1)).unwrap();

    // 구분선
    let separator = "─".repeat(40);
    let separator_x = if current_width > separator.len() as u16 {
        (current_width - separator.len() as u16) / 2
    } else {
        0
    };

    execute!(
        stdout,
        cursor::MoveTo(separator_x, center_y + 4),
        SetForegroundColor(Color::DarkGrey),
        Print(&separator),
        ResetColor
    )
    .unwrap();

    // 조작법 안내
    let controls_title = "[ 조작법 ]";
    let controls_title_x = if current_width > controls_title.len() as u16 {
        (current_width - controls_title.len() as u16) / 2
    } else {
        0
    };

    execute!(
        stdout,
        cursor::MoveTo(controls_title_x, center_y + 6),
        SetForegroundColor(Color::Cyan),
        Print(controls_title.bold()),
        ResetColor
    )
    .unwrap();

    let controls = vec![
        "← → : 좌우 이동",
        "↑ : 회전",
        "↓ : 빠른 낙하",
        "Space : 즉시 낙하",
        "ESC : 일시정지",
    ];

    for (idx, control) in controls.iter().enumerate() {
        let x = if current_width > control.len() as u16 {
            (current_width - control.len() as u16) / 2
        } else {
            0
        };

        execute!(
            stdout,
            cursor::MoveTo(x, center_y + 8 + idx as u16),
            Print(control)
        )
        .unwrap();
    }

    // 시작 안내
    let start_msg = "Enter 또는 Space - 게임 시작";
    let start_x = if current_width > start_msg.len() as u16 {
        (current_width - start_msg.len() as u16) / 2
    } else {
        0
    };

    execute!(
        stdout,
        cursor::MoveTo(start_x, center_y + 15),
        SetForegroundColor(Color::Yellow),
        Print(start_msg.bold()),
        ResetColor
    )
    .unwrap();

    // 종료 안내
    let exit_msg = "ESC - 종료";
    let exit_x = if current_width > exit_msg.len() as u16 {
        (current_width - exit_msg.len() as u16) / 2
    } else {
        0
    };

    execute!(
        stdout,
        cursor::MoveTo(exit_x, center_y + 16),
        SetForegroundColor(Color::DarkGrey),
        Print(exit_msg),
        ResetColor
    )
    .unwrap();

    stdout.flush().unwrap();
}

pub fn renderer(
    stdout: &mut Stdout,
    current_width: u16,
    current_height: u16,
    width_diff: u16,
    height_diff: u16,
    size_ok: bool,
) {
    clear_terminal(stdout);

    // 화면 중앙 계산
    let center_y = current_height / 2;
    let center_y = if center_y > 5 { center_y - 5 } else { 1 };

    // 크기가 충족되면 시작 대기 화면 표시
    if size_ok {
        render_ready_screen(stdout, current_width, center_y);
        return;
    }

    // 제목
    let title = " 터미널 크기 부족 ";
    let title_x = if current_width > title.len() as u16 {
        (current_width - title.len() as u16) / 2
    } else {
        0
    };

    execute!(
        stdout,
        cursor::MoveTo(title_x, center_y),
        SetForegroundColor(Color::Red),
        Print(title.bold()),
        ResetColor
    )
    .unwrap();

    // 현재 크기 표시
    let current_msg = format!("현재 크기: {}x{}", current_width, current_height);
    let current_x = if current_width > current_msg.len() as u16 {
        (current_width - current_msg.len() as u16) / 2
    } else {
        0
    };

    execute!(
        stdout,
        cursor::MoveTo(current_x, center_y + 2),
        Print(current_msg)
    )
    .unwrap();

    // 필요한 크기 표시
    let required_msg = format!("필요한 크기: 70x25");
    let required_x = if current_width > required_msg.len() as u16 {
        (current_width - required_msg.len() as u16) / 2
    } else {
        0
    };

    execute!(
        stdout,
        cursor::MoveTo(required_x, center_y + 3),
        SetForegroundColor(Color::Green),
        Print(required_msg.bold()),
        ResetColor
    )
    .unwrap();

    // 부족한 크기 표시
    if width_diff > 0 || height_diff > 0 {
        let mut diff_parts = Vec::new();

        if width_diff > 0 {
            diff_parts.push(format!("너비 {}칸", width_diff));
        }

        if height_diff > 0 {
            diff_parts.push(format!("높이 {}줄", height_diff));
        }

        let diff_msg = format!("부족: {}", diff_parts.join(", "));
        let diff_x = if current_width > diff_msg.len() as u16 {
            (current_width - diff_msg.len() as u16) / 2
        } else {
            0
        };

        execute!(
            stdout,
            cursor::MoveTo(diff_x, center_y + 5),
            SetForegroundColor(Color::Red),
            Print(diff_msg.bold()),
            ResetColor
        )
        .unwrap();
    }

    // 안내 메시지
    let instruction1 = "터미널 창을 더 키워주세요";
    let instruction1_x = if current_width > instruction1.len() as u16 {
        (current_width - instruction1.len() as u16) / 2
    } else {
        0
    };

    execute!(
        stdout,
        cursor::MoveTo(instruction1_x, center_y + 7),
        SetForegroundColor(Color::Yellow),
        Print(instruction1),
        ResetColor
    )
    .unwrap();

    let instruction2 = "(Cmd + / Cmd - 또는 View > Appearance)";
    let instruction2_x = if current_width > instruction2.len() as u16 {
        (current_width - instruction2.len() as u16) / 2
    } else {
        0
    };

    execute!(
        stdout,
        cursor::MoveTo(instruction2_x, center_y + 8),
        SetForegroundColor(Color::DarkGrey),
        Print(instruction2),
        ResetColor
    )
    .unwrap();

    // ESC 안내
    let esc_msg = "ESC - 종료";
    let esc_x = if current_width > esc_msg.len() as u16 {
        (current_width - esc_msg.len() as u16) / 2
    } else {
        0
    };

    execute!(
        stdout,
        cursor::MoveTo(esc_x, center_y + 10),
        SetForegroundColor(Color::DarkGrey),
        Print(esc_msg),
        ResetColor
    )
    .unwrap();

    stdout.flush().unwrap();
}
