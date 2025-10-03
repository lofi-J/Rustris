#[derive(Clone, Debug)]
pub enum State {
    Wellcome,
    FixResolution,
    Play,
    GameOver,
    Exit,
}

pub struct StateController {
    state: State,
}

impl StateController {
    pub fn new() -> Self {
        Self {
            state: State::Wellcome,
        }
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state
    }
}
