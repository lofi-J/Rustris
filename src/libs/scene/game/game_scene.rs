use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::libs::{state::state::StateController, utils::terminal::enable_raw_mode};

use super::controller::GameController;
use super::renderer::renderer;

pub fn game(state: &mut StateController) {
    let mut stdout = io::stdout();
    enable_raw_mode(state);

    // Arc<Mutex>로 controller를 래핑하여 thread-safe하게 공유
    let controller = Arc::new(Mutex::new(GameController::new()));
    let controller_clone = Arc::clone(&controller);

    // Input processing thread
    thread::spawn(move || {
        loop {
            {
                let mut ctrl = controller_clone.lock().unwrap();
                if ctrl.is_game_over() {
                    break;
                }
                ctrl.handle_input();
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Main rendering loop(game)
    loop {
        {
            let mut ctrl = controller.lock().unwrap(); // Get GameController Status

            // if game over, break loop
            if ctrl.is_game_over() {
                break;
            }

            // Update status every 16ms
            ctrl.update();

            // rendering
            renderer(&mut stdout, &ctrl);
        }

        // Frame Limit (60 FPS)
        thread::sleep(Duration::from_millis(16));
    }
}
