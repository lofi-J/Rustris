use crate::libs::{print_management, state::state::StateController};

pub fn fix_resolution(state: &mut StateController) {
    print_management::resolution::minimum_resolution();
}
