use std::io;

use crate::libs::state::state::StateController;

use super::renderer::renderer;

pub fn game(_tate: &mut StateController) {
    println!("Playing Tetris");

    let mut stdout = io::stdout();

    renderer(&mut stdout);
}
