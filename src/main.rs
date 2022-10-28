use macroquad::window::{next_frame, screen_height, screen_width};
use main_menu::MainMenuView;
use screen::View;

mod main_menu;
mod screen;

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
    fn to_view(&self) -> Box<dyn View>{
        match *self{
            State::MainMenu => Box::new(MainMenuView::default()),
            State::Pong => todo!(),
            State::Exit => todo!(),
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

        let control_res = view.handle_input();
        if control_res.is_some() {
            state = control_res.unwrap();
            view = state.to_view();
        }
        view.draw();
        next_frame().await;
    }
}
