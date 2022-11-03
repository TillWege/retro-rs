use crate::main_menu::consts::*;
use macroquad::prelude::*;

use super::state::MainMenuState;

pub struct Button {
    x: f32,
    y: f32,
    height: f32,
    width: f32,
    font_size: u16,
    caption: String,
    color: Color,
    selected_color: Color,
    font_color: Color,
    pub selected: bool,
}

impl Button {
    pub fn draw(&self) {
        draw_rectangle(
            self.x,
            self.y,
            self.width,
            self.height,
            if self.selected {
                self.selected_color
            } else {
                self.color
            },
        );

        let text_size = measure_text(&self.caption, Some(Font::default()), self.font_size, 1.0);

        draw_text(
            &self.caption,
            self.x + ((self.width - text_size.width) / 2.0),
            self.y + text_size.height + ((self.height - text_size.height) * 0.5),
            self.font_size as f32,
            self.font_color,
        );
    }
}

pub(super) fn init_buttons(
    width: f32,
    _height: f32,
    state: MainMenuState,
) -> (Button, Button, Button) {
    let title_rect_width = width - (2.0 * MENU_MARGIN_HORZ);

    let title = Button {
        x: MENU_MARGIN_HORZ,
        y: TITLE_BAR_POS_Y,
        height: TITLE_BAR_HEIGHT,
        width: title_rect_width,
        font_size: 30,
        caption: TITLE_TEXT.to_string(),
        color: LIGHTGRAY,
        selected_color: GRAY,
        font_color: DARKGRAY,
        selected: false,
    };

    let button_width = width - (2.0 * BUTTON_MARGIN_HORZ);

    let pong_button = Button {
        x: BUTTON_MARGIN_HORZ,
        y: PONG_BUTTON_Y,
        height: BUTTON_HEIGHT,
        width: button_width,
        font_size: 16,
        caption: PONG_BUTTON_TEXT.to_string(),
        color: LIGHTGRAY,
        selected_color: GRAY,
        font_color: DARKGRAY,
        selected: state == MainMenuState::PongSelected,
    };

    let exit_button = Button {
        x: BUTTON_MARGIN_HORZ,
        y: EXIT_BUTTON_Y,
        height: BUTTON_HEIGHT,
        width: button_width,
        font_size: 16,
        caption: EXIT_BUTTON_TEXT.to_string(),
        color: LIGHTGRAY,
        selected_color: GRAY,
        font_color: DARKGRAY,
        selected: state == MainMenuState::ExitSelected,
    };

    return (title, pong_button, exit_button);
}
