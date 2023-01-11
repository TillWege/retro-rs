use macroquad::{texture::Image, prelude::WHITE};

const BALL_DIAMETER: u32 = 5;

pub(super) struct Ball {
    x_pos: u32,
    y_pos: u32,
    y_vel: u32,
    x_vel: u32,
}

impl Ball {
    pub(super) fn update(&self) {

    }

    // a draw function that draws the ball around the center 
    // to the given image and makes sure to stay in bounds
    pub(super) fn draw(&self, img: &mut Image) {
        
        fn check_bounds(x: u32, y: u32, img: &mut Image) -> bool {
            if (x + BALL_DIAMETER) as u16 >= img.width || x <= 0 {
                return false;
            }
            if (y + BALL_DIAMETER) as u16 >= img.height || y <= 0 {
                return false;
            }
            true
        }

        let x_pos = self.x_pos.checked_sub(BALL_DIAMETER / 2).unwrap_or(0);
        let y_pos = self.y_pos.checked_sub(BALL_DIAMETER / 2).unwrap_or(0);

        for x in x_pos..x_pos + BALL_DIAMETER {
            for y in y_pos..y_pos + BALL_DIAMETER {
                if check_bounds(x, y, img) {
                    img.set_pixel(x, y, WHITE);
                }
            }
        }
    }
 
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            x_pos: 100,
            y_pos: 100,
            x_vel: 0,
            y_vel: 0,
        }
    }
}
