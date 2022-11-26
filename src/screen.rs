use crate::State;

pub(crate) trait View {
    fn draw(&self);

    // interactivity
    fn update(&mut self) -> Option<State>;

    // events
    fn on_resize(&mut self, new_width: f32, new_height: f32);
}
