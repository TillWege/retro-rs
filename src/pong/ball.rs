use macroquad::texture::Image;

const BALL_DIAMETER: u32 = 5;

pub(super) struct Ball {
    x_pos: u32,
    y_pos: u32,
    y_vel: u32,
    x_vel: u32,
}

impl Ball {
    fn update() {

    }

    pub fn draw(self: &Ball, img: &mut Image) {}

    pub fn reset(self: &mut Ball) {

    }
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            x_pos: 0,
            y_pos: 0,
            x_vel: 0,
            y_vel: 0,
        }
    }
}
