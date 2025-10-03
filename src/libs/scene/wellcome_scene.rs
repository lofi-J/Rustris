use crate::libs::{print_management, state::state::StateController};

pub fn wellcome(state: &mut StateController) {
    print_management::ascii::wellcome_message();

    println!("Let's play!");
    println!("Check your resolution and fix");
}
