use std::io::Stdout;

use crate::libs::utils::terminal::clear_terminal;

pub fn renderer(stdout: &mut Stdout) {
    clear_terminal(stdout);
}
