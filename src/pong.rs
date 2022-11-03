use crate::screen::View;

struct pong {

}

impl View for pong {
    fn draw(&self) {
        todo!()
    }

    fn handle_input(&mut self) -> Option<crate::State> {
        todo!()
    }

    fn on_resize(&mut self, new_width: f32, new_height: f32) {
        todo!()
    }
}

impl Default for pong {
    fn default() -> Self {
        Self {  }
    }
}