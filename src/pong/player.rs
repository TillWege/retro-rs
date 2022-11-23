const PLAYER_HEIGHT: f32 = 80.0;
const PLAYER_WIDTH: f32 = 20.0;

pub(super) struct Player {
    x_pos: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { x_pos: 0.0 }
    }
}
