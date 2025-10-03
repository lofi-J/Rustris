use std::io;

use crate::libs::state::state::StateController;

use super::renderer::renderer;

pub fn game_over(_state: &mut StateController) {
    println!("Game Over");

    let mut stdout = io::stdout();

    renderer(&mut stdout);
}
