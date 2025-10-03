mod libs;

use libs::{scene, state};
use state::state::{State, StateController};

use scene::{fix_resolution, game, game_over, wellcome};

use fix_resolution::fix_resolution_scene;
use game::game_scene;
use game_over::game_over_scene;
use wellcome::wellcome_scene;

fn main() {
    let mut state = StateController::new();

    loop {
        let current_state = state.get_state();

        match current_state {
            State::Wellcome => wellcome_scene::wellcome(&mut state),
            State::FixResolution => fix_resolution_scene::fix_resolution(&mut state),
            State::Play => game_scene::play_tetris(&mut state),
            State::GameOver => game_over_scene::game_over(&mut state),
            State::Exit => break,
        }
    }

    println!("\nGoodbye!");
}
