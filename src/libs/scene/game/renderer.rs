use std::io::{Stdout, Write};

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

use crate::libs::utils::terminal::clear_terminal;

use super::controller::GameController;

const CELL: &str = "██";

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
    let frame_top = 2;
    let frame_left = 10;

    // 상단 테두리
    execute!(
        stdout,
        cursor::MoveTo(frame_left, frame_top),
        Print("╔════════════════════════════════════════╗")
    )
    .unwrap();

    // 중간 부분 (20줄)
    for i in 1..=20 {
        execute!(
            stdout,
            cursor::MoveTo(frame_left, frame_top + i),
            Print("║                                        ║")
        )
        .unwrap();
    }

    // 하단 테두리
    execute!(
        stdout,
        cursor::MoveTo(frame_left, frame_top + 21),
        Print("╚════════════════════════════════════════╝")
    )
    .unwrap();
}

/// 보드에 쌓인 블록들 그리기
fn draw_board(stdout: &mut Stdout, controller: &GameController) {
    let frame_top = 2;
    let frame_left = 10;
    let board_start_x = frame_left + 2; // 테두리 내부 시작점
    let board_start_y = frame_top + 1;

    for (row_idx, row) in controller.board.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell {
                let x = board_start_x + (col_idx as u16 * 4); // 각 셀은 4칸 (██ + 공백)
                let y = board_start_y + row_idx as u16;

                execute!(
                    stdout,
                    cursor::MoveTo(x, y),
                    SetForegroundColor(Color::Cyan),
                    Print(CELL),
                    ResetColor
                )
                .unwrap();
            }
        }
    }
}

/// 현재 떨어지는 테트로미노 그리기
fn draw_current_tetromino(stdout: &mut Stdout, controller: &GameController) {
    let frame_top = 2;
    let frame_left = 10;
    let board_start_x = frame_left + 2;
    let board_start_y = frame_top + 1;

    let shape = controller.current_tetromino.get_shape();
    let (tetromino_x, tetromino_y) = controller.tetromino_pos;

    for (row_idx, row) in shape.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell {
                // 테트로미노 위치를 보드 좌표로 변환
                let x = board_start_x + ((tetromino_x + col_idx as u16) * 4);
                let y = board_start_y + tetromino_y + row_idx as u16;

                execute!(
                    stdout,
                    cursor::MoveTo(x, y),
                    SetForegroundColor(Color::Yellow),
                    Print(CELL),
                    ResetColor
                )
                .unwrap();
            }
        }
    }
}

/// 다음 테트로미노 미리보기 그리기
fn draw_preview(stdout: &mut Stdout, controller: &GameController) {
    let preview_x = 55;
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
                    let x = preview_x + (col_idx as u16 * 2);
                    let y = offset_y + row_idx as u16;

                    execute!(
                        stdout,
                        cursor::MoveTo(x, y),
                        SetForegroundColor(Color::Green),
                        Print(CELL),
                        ResetColor
                    )
                    .unwrap();
                }
            }
        }
    }
}
