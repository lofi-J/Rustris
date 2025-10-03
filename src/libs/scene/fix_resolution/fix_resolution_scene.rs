use std::io;

use crate::libs::state::state::StateController;

use super::renderer::renderer;

pub fn fix_resolution(_state: &mut StateController) {
    let mut stdout = io::stdout();

    renderer(&mut stdout);
}
