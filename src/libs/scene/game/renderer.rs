use std::{
    collections::HashSet,
    io::{Stdout, Write},
};

use crossterm::{
    cursor, execute,
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
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

    // 테트로미노 낙하 가이드라인 그리기
    draw_guide_lines(stdout, controller);

    // 보드에 쌓인 블록들 그리기
    draw_board(stdout, controller);

    // 현재 떨어지는 테트로미노 그리기
    draw_current_tetromino(stdout, controller);

    // 다음 테트로미노 미리보기 그리기
    draw_preview(stdout, controller);

    stdout.flush().unwrap();
}

/// 테트로미노 낙하 가이드라인 그리기
fn draw_guide_lines(stdout: &mut Stdout, controller: &GameController) {
    let shape = controller.current_tetromino.get_shape();
    let (tetromino_x, _) = controller.tetromino_pos;
    let color = controller.current_tetromino.get_color();

    // 테트로미노가 차지하는 x 좌표들을 수집
    let mut occupied_x_positions = HashSet::new();

    for row in shape.iter() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell {
                let board_x = tetromino_x + col_idx as i32;
                if board_x >= 0 && board_x < 10 {
                    occupied_x_positions.insert(board_x);
                }
            }
        }
    }

    // 각 x 좌표의 세로줄에 색상 있는 dot 그리기
    for &x in occupied_x_positions.iter() {
        for y in 0..20 {
            // 화면 좌표로 변환
            let screen_x = BOARD_START_X + (x as u16 * 2);
            let screen_y = BOARD_START_Y + y;

            execute!(
                stdout,
                cursor::MoveTo(screen_x, screen_y),
                SetForegroundColor(color),
                Print("·"),
                ResetColor
            )
            .unwrap();
        }
    }
}

/// 게임 보드 프레임 그리기
fn draw_board_frame(stdout: &mut Stdout) {
    // 보드 너비: 10칸 × 2문자 = 20문자
    let board_width = 10 * 2;
    let border_line = "═".repeat(board_width);

    // 세로 구분선을 위한 점 패턴 생성 (각 셀의 중앙에 점)
    // 10칸이므로 "· · · · · · · · · ·" 형태
    let grid_line = (0..10).map(|_| "· ").collect::<String>();

    // 상단 테두리
    execute!(
        stdout,
        cursor::MoveTo(FRAME_LEFT, FRAME_TOP),
        Print(format!("╔{}╗", border_line))
    )
    .unwrap();

    // 중간 부분 (20줄) - 점 패턴으로 세로선 표시
    for i in 1..=20 {
        execute!(
            stdout,
            cursor::MoveTo(FRAME_LEFT, FRAME_TOP + i),
            Print(format!("║{}║", grid_line))
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
        for (col_idx, cell) in row.iter().enumerate() {
            // cell이 Some(Color)인 경우에만 그리기
            if let Some(color) = cell {
                // 보드 범위 체크 (0-9, 0-19)
                if col_idx < 10 && row_idx < 20 {
                    let x = BOARD_START_X + (col_idx as u16 * 2); // 각 셀은 2칸
                    let y = BOARD_START_Y + row_idx as u16;

                    execute!(
                        stdout,
                        cursor::MoveTo(x, y),
                        SetBackgroundColor(*color), // 저장된 색상 사용
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
    let color = controller.current_tetromino.get_color(); // 테트로미노 고유 색상

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
                        SetBackgroundColor(color),
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
        let color = tetromino.get_color(); // 각 테트로미노의 고유 색상
        let offset_y = preview_y + 2 + (idx as u16 * 5);

        for (row_idx, row) in shape.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell {
                    let x = preview_x + (col_idx as u16 * 2); // CELL 크기에 맞춰 2칸으로 변경
                    let y = offset_y + row_idx as u16;

                    execute!(
                        stdout,
                        cursor::MoveTo(x, y),
                        SetBackgroundColor(color),
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
