use std::{io, thread, time::Duration};

use crossterm::{execute, terminal};

use crate::libs::{
    state::state::{State, StateController},
    utils::terminal::enable_raw_mode,
};

use super::{controller::GameController, renderer};

pub fn game(state: &mut StateController) {
    let mut stdout = io::stdout();
    enable_raw_mode(state);

    // Alternate screen 활성화 (깜빡임 방지)
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();

    let mut controller = GameController::new();

    // 프레임 속도 제한 (60 FPS)
    let frame_duration = Duration::from_millis(16); // 약 60 FPS

    loop {
        if controller.is_game_over {
            state.set_state(State::GameOver);
            break;
        }

        // 사용자 입력 처리
        controller.handle_input();

        // 게임 상태 업데이트 (자동 낙하)
        controller.update();

        // 화면 렌더링
        renderer::renderer(&mut stdout, &controller);

        // 프레임 속도 제한
        thread::sleep(frame_duration);
    }

    // Alternate screen 비활성화
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
}
