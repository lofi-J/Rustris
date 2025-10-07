use std::io::{Stdout, Write};

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
};

use crate::libs::utils::terminal::clear_terminal;

use super::controller::GameController;

const CELL: &str = "  "; // 공백 2개로 정사각형에 가까운 형태

// 렌더링 위치 상수
const FRAME_TOP: u16 = 2;
const FRAME_LEFT: u16 = 10;
const BOARD_START_X: u16 = FRAME_LEFT + 1; // ╔는 1칸 차지
const BOARD_START_Y: u16 = FRAME_TOP + 1;

pub fn renderer(stdout: &mut Stdout, controller: &GameController) {
    clear_terminal(stdout);

    // 게임 보드 프레임 그리기
    draw_board_frame(stdout);

    // 보드에 쌓인 블록들 그리기
    draw_board(stdout, controller);

    // 현재 떨어지는 테트로미노 그리기
    draw_current_tetromino(stdout, controller);

    // 다음 테트로미노 미리보기 그리기
    draw_preview(stdout, controller);

    stdout.flush().unwrap();
}

/// 게임 보드 프레임 그리기
fn draw_board_frame(stdout: &mut Stdout) {
    // 보드 너비: 10칸 × 2문자 = 20문자
    let board_width = 10 * 2;
    let inner_space = " ".repeat(board_width);
    let border_line = "═".repeat(board_width);

    // 상단 테두리
    execute!(
        stdout,
        cursor::MoveTo(FRAME_LEFT, FRAME_TOP),
        Print(format!("╔{}╗", border_line))
    )
    .unwrap();

    // 중간 부분 (20줄)
    for i in 1..=20 {
        execute!(
            stdout,
            cursor::MoveTo(FRAME_LEFT, FRAME_TOP + i),
            Print(format!("║{}║", inner_space))
        )
        .unwrap();
    }

    // 하단 테두리
    execute!(
        stdout,
        cursor::MoveTo(FRAME_LEFT, FRAME_TOP + 21),
        Print(format!("╚{}╝", border_line))
    )
    .unwrap();
}

/// 보드에 쌓인 블록들 그리기
fn draw_board(stdout: &mut Stdout, controller: &GameController) {
    for (row_idx, row) in controller.board.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell {
                // 보드 범위 체크 (0-9, 0-19)
                if col_idx < 10 && row_idx < 20 {
                    let x = BOARD_START_X + (col_idx as u16 * 2); // 각 셀은 2칸
                    let y = BOARD_START_Y + row_idx as u16;

                    execute!(
                        stdout,
                        cursor::MoveTo(x, y),
                        SetBackgroundColor(Color::Cyan),
                        Print(CELL),
                        ResetColor
                    )
                    .unwrap();
                }
            }
        }
    }
}

/// 현재 떨어지는 테트로미노 그리기
fn draw_current_tetromino(stdout: &mut Stdout, controller: &GameController) {
    let shape = controller.current_tetromino.get_shape();
    let (tetromino_x, tetromino_y) = controller.tetromino_pos;

    for (row_idx, row) in shape.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell {
                // 보드 좌표 계산 (이미 i32 타입)
                let board_x = tetromino_x + col_idx as i32;
                let board_y = tetromino_y + row_idx as i32;

                // 보드 범위 내에서만 그리기 (0-9, 0-19)
                if board_x >= 0 && board_x < 10 && board_y >= 0 && board_y < 20 {
                    // 화면 좌표로 변환
                    let x = BOARD_START_X + (board_x as u16 * 2);
                    let y = BOARD_START_Y + board_y as u16;

                    execute!(
                        stdout,
                        cursor::MoveTo(x, y),
                        SetBackgroundColor(Color::Yellow),
                        Print(CELL),
                        ResetColor
                    )
                    .unwrap();
                }
            }
        }
    }
}

/// 다음 테트로미노 미리보기 그리기
fn draw_preview(stdout: &mut Stdout, controller: &GameController) {
    let preview_x = 35;
    let preview_y = 4;

    execute!(
        stdout,
        cursor::MoveTo(preview_x, preview_y),
        Print("[ NEXT ]")
    )
    .unwrap();

    for (idx, tetromino) in controller.preview_tetrominos.iter().enumerate() {
        let shape = tetromino.get_shape();
        let offset_y = preview_y + 2 + (idx as u16 * 5);

        for (row_idx, row) in shape.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell {
                    let x = preview_x + (col_idx as u16 * 2); // CELL 크기에 맞춰 2칸으로 변경
                    let y = offset_y + row_idx as u16;

                    execute!(
                        stdout,
                        cursor::MoveTo(x, y),
                        SetBackgroundColor(Color::Green),
                        Print(CELL),
                        ResetColor
                    )
                    .unwrap();
                }
            }
        }
    }

    // 디버그 정보: 현재 테트로미노 위치 표시
    let (x, y) = controller.tetromino_pos;
    execute!(
        stdout,
        cursor::MoveTo(preview_x, preview_y + 20),
        Print(format!("Pos: ({}, {})", x, y))
    )
    .unwrap();

    // 보드 경계 표시
    execute!(
        stdout,
        cursor::MoveTo(preview_x, preview_y + 21),
        Print("Board: 0-9 x 0-19")
    )
    .unwrap();
}
