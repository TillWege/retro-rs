use macroquad::prelude::*;

use crate::State;

const TITLE_TEXT: &str = "RETRO_RUST";
const TITLE_TEXT_SIZE: f32 = 30.0;
const TITLE_BAR_HEIGHT: f32 = 40.0;

#[derive()]
enum MainMenuState {
    PongSelected,
    ExitSelected
}

pub async fn main_menu() -> State {
    let mut menu_state = MainMenuState::PongSelected;
    loop {
        clear_background(WHITE);
        render_title();
        render_pong_button();


        if is_key_down(KeyCode::Escape) {
            return State::Exit;
        }
    
        if is_key_down(KeyCode::Enter) {
            return State::Pong;
        }
        next_frame().await
    }
}

fn render_title(){
    let title_rect_width = screen_width() - 40.0;

    draw_rectangle(20.0, 50.0, title_rect_width, TITLE_BAR_HEIGHT, LIGHTGRAY);

    let text_width = measure_text(
        TITLE_TEXT,
        Some(Font::default()),
        TITLE_TEXT_SIZE as u16,
        1.0,
    );
    draw_text(
        TITLE_TEXT,
        20.0 + ((title_rect_width - text_width.width) / 2.0),
        50.0 + text_width.height + ((TITLE_BAR_HEIGHT - text_width.height) * 0.5),
        TITLE_TEXT_SIZE,
        DARKGRAY,
    );
} 

fn render_pong_button() {

}

fn render_exit_button() {

}