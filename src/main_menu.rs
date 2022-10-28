mod ui;
mod state;
mod consts;

use crate::{screen::View, State};
use macroquad::prelude::*;
use self::{ui::{init_buttons, Button}, state::MainMenuState};

pub (crate) struct MainMenuView {
    menu_state: MainMenuState,
    title: Button,
    pong_button: Button,
}

impl Default for MainMenuView {
    fn default() -> Self {
        let (title, pong_button) = init_buttons(screen_width(), screen_height());
        Self {
            menu_state: MainMenuState::PongSelected,
            title,
            pong_button
        }
    }
}

impl View for MainMenuView {
    fn draw(&self) {
        clear_background(WHITE);

        self.title.draw();
        self.pong_button.draw();
    }

    fn handle_input(&mut self) ->  Option<State> {
        let mut result = None;
        
        if is_key_down(KeyCode::Escape) {
            result = Some(State::Exit);
        }

        if is_key_down(KeyCode::Enter) {
            result = Some(self.menu_state.to_state());
        }

        if is_key_down(KeyCode::Up) {
            self.menu_state = self.menu_state.next();
        }

        if is_key_down(KeyCode::Down) {
            self.menu_state = self.menu_state.prev();
        }

        return result;
    }

    fn on_resize(&mut self, new_width: f32, new_height: f32) {
        (self.title, self.pong_button) = init_buttons(new_width, new_height);
    }
}