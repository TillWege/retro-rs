use macroquad::prelude::*;

use crate::State;

pub async fn pong() -> State {
    loop {
        clear_background(WHITE);

        if is_key_down(KeyCode::Backspace) {
            return State::MainMenu;
        }

        next_frame().await;
    }
}
