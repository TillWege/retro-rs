mod main_menu;
mod pong;
mod screen;

use macroquad::window::{next_frame, screen_height, screen_width};
use main_menu::MainMenuView;
use pong::Pong;
use screen::View;
use std::process::exit;

#[derive(PartialEq)]
pub enum State {
    MainMenu,
    Pong,
    Exit,
}

impl Default for State {
    fn default() -> Self {
        State::MainMenu
    }
}

impl State {
    fn to_view(&self) -> Box<dyn View> {
        match *self {
            State::MainMenu => Box::new(MainMenuView::default()),
            State::Pong => Box::new(Pong::default()),
            State::Exit => exit(0),
        }
    }
}

#[macroquad::main("rust_retro")]
async fn main() {
    let mut height = screen_height();
    let mut width = screen_width();

    let mut state = State::default();
    let mut view: Box<dyn View> = state.to_view();

    while state != State::Exit {
        let new_height = screen_height();
        let new_width = screen_width();
        if (height != new_height) || (width != new_width) {
            height = new_height;
            width = new_width;

            view.on_resize(width, height);
        }

        let control_res = view.update();
        if control_res.is_some() {
            state = control_res.unwrap();
            view = state.to_view();
            //view.on_resize(new_width, new_height);
        }
        view.draw();
        next_frame().await;
    }
}
