use mainmenu::main_menu;
use pong::pong;

mod mainmenu;
mod pong;

#[derive(PartialEq)]
pub enum State {
    MainMenu,
    Pong,
    Exit,
}

#[macroquad::main("rust_retro")]
async fn main() {
    let mut state = State::MainMenu;
    while state != State::Exit {
        state = match state {
            State::MainMenu => main_menu().await,
            State::Pong => pong().await,
            State::Exit => State::Exit,
        }
    }
}
