mod consts;
mod state;
mod ui;

use self::{
    state::MainMenuState,
    ui::{init_buttons, Button},
};
use crate::{screen::View, State};
use macroquad::prelude::*;

pub(crate) struct MainMenuView {
    menu_state: MainMenuState,
    title: Button,
    pong_button: Button,
    exit_button: Button,
}

impl Default for MainMenuView {
    fn default() -> Self {
        let (title, pong_button, exit_button) =
            init_buttons(screen_width(), screen_height(), MainMenuState::PongSelected);
        Self {
            menu_state: MainMenuState::PongSelected,
            title,
            pong_button,
            exit_button,
        }
    }
}

impl View for MainMenuView {
    fn draw(&self) {
        clear_background(WHITE);

        self.title.draw();
        self.pong_button.draw();
        self.exit_button.draw();
    }

    fn handle_input(&mut self) -> Option<State> {
        let mut result = None;

        if is_key_pressed(KeyCode::Escape) {
            result = Some(State::Exit);
        }

        if is_key_pressed(KeyCode::Enter) {
            result = Some(self.menu_state.to_state());
        }

        if is_key_pressed(KeyCode::Up) {
            self.menu_state = self.menu_state.next();
            self.update_active_button();
        }

        if is_key_pressed(KeyCode::Down) {
            self.menu_state = self.menu_state.prev();
            self.update_active_button();
        }

        return result;
    }

    fn on_resize(&mut self, new_width: f32, new_height: f32) {
        (self.title, self.pong_button, self.exit_button) =
            init_buttons(new_width, new_height, self.menu_state);
    }
}

impl MainMenuView {
    fn update_active_button(&mut self) {
        self.title.selected = false;
        self.pong_button.selected = self.menu_state == MainMenuState::PongSelected;
        self.exit_button.selected = self.menu_state == MainMenuState::ExitSelected;
    }
}
