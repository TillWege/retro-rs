use crate::State;

#[derive(PartialEq, Copy, Clone)]
pub(super) enum MainMenuState {
    PongSelected,
    ExitSelected,
}

impl MainMenuState {
    pub(super) fn next(&self) -> Self {
        use MainMenuState::*;
        match *self {
            PongSelected => ExitSelected,
            ExitSelected => PongSelected,
        }
    }

    pub(super) fn prev(&self) -> Self {
        use MainMenuState::*;
        match *self {
            PongSelected => ExitSelected,
            ExitSelected => PongSelected,
        }
    }

    pub(super) fn to_state(&self) -> State {
        use MainMenuState::*;
        match *self {
            PongSelected => State::Pong,
            ExitSelected => State::Exit,
        }
    }
}
