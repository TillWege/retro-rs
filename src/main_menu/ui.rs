use macroquad::prelude::*;
use crate::main_menu::consts::*;

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
    pub font_size: u16,
    pub caption: String,
}

impl Button {
    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, LIGHTGRAY);

        let text_size = measure_text(&self.caption, Some(Font::default()), self.font_size, 1.0);

        draw_text(
            &self.caption,
            self.x + ((self.width - text_size.width) / 2.0),
            self.y + text_size.height + ((self.height - text_size.height) * 0.5),
            self.font_size as f32,
            DARKGRAY,
        );
    }
}

pub(super) fn init_buttons(width: f32, _height: f32) -> (Button, Button) {
    let title_rect_width = width - (2.0 * MENU_MARGIN_HORZ);

    let title = Button {
        x: MENU_MARGIN_HORZ,
        y: TITLE_BAR_POS_Y,
        height: TITLE_BAR_HEIGHT,
        width: title_rect_width,
        font_size: 30,
        caption: TITLE_TEXT.to_string(),
    };

    const BASE_Y: f32 = TITLE_BAR_POS_Y + TITLE_BAR_HEIGHT + BUTTON_MARGIN_VERT;
    const BUTTON_TEXT: &str = "Pong";
    let button_width = width - (2.0 * BUTTON_MARGIN_HORZ);

    let btn = Button {
        x: BUTTON_MARGIN_HORZ,
        y: BASE_Y,
        height: BUTTON_HEIGHT,
        width: button_width,
        font_size: 16,
        caption: BUTTON_TEXT.to_string(),
    };

    return (title, btn);
}