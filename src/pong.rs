mod ball;
mod player;

use macroquad::{prelude::*, window::clear_background};

use crate::{screen::View, State};

use self::{player::Player, ball::Ball};

const ASPECT_RATIO: f32 = 2.0;
const GAME_WIDTH: u16 = 512;
const GAME_HEIGHT: u16 = 256;

pub struct Pong {
    player_left: Player,
    player_right: Player,
    ball: Ball,
    scaling: Vec2,
    use_integer_scaling: bool,
}

impl View for Pong {
    fn draw(&self) {
        clear_background(BLACK);

        // ---------------
        // prepare texture
        // ---------------

        let mut img = Image::gen_image_color(GAME_WIDTH, GAME_HEIGHT, GRAY);

        // ---------------------------
        // draw vertical line
        // ---------------------------

        for i in 0..u32::from(GAME_HEIGHT) {
            if i % 20 <= 10 {
                img.set_pixel((u32::from(GAME_WIDTH) / 2) - 1, i, WHITE);
                img.set_pixel(u32::from(GAME_WIDTH) / 2, i, WHITE);    
            }
        }

        // ---------------------------
        // draw gameobjects to texture
        // ---------------------------

        self.player_left.draw(&mut img);
        self.player_right.draw(&mut img);

        // ----------------------
        // draw texture to screen
        // ----------------------

        let border_height = screen_height() - self.scaling.y;
        let border_width = screen_width() - self.scaling.x;

        draw_texture_ex(
            Texture2D::from_image(&img),
            border_width / 2.0,
            border_height / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.scaling),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }

    fn update(&mut self) -> Option<crate::State> {

        // -------------
        // handle inputs
        // -------------

        if is_key_pressed(KeyCode::Escape) {
            return Some(State::MainMenu);
        }

        if is_key_down(KeyCode::Up) {
            self.player_right.move_up();
        }

        if is_key_down(KeyCode::Down) {
            self.player_right.move_down();
        }

        if is_key_down(KeyCode::W) {
            self.player_left.move_up();
        }

        if is_key_down(KeyCode::S) {
            self.player_left.move_down();
        }

        if is_key_pressed(KeyCode::P) {
            self.use_integer_scaling = !self.use_integer_scaling;

            println!("setting integer scaling: {}", self.use_integer_scaling);

            self.on_resize(screen_width(), screen_height());
        }

        // -------------
        // run game loop
        // -------------



        return None;
    }

    fn on_resize(&mut self, new_width: f32, new_height: f32) {
        self.scaling = calculate_texture_scaling(new_width, new_height, self.use_integer_scaling);

        println!(
            "resizing to x: {}, y: {}, integer scaling: {}",
            new_width, new_height, self.use_integer_scaling
        );
        println!("new Scaling set to: {:?}", self.scaling);
    }
}

fn calculate_texture_scaling(
    mut new_width: f32,
    mut new_height: f32,
    use_integer_scaling: bool,
) -> Vec2 {
    if use_integer_scaling {
        let rem_x = new_width % f32::from(GAME_WIDTH);
        let rem_y = new_height % f32::from(GAME_HEIGHT);

        new_width -= rem_x;
        new_height -= rem_y;
    }

    if new_width > (new_height * ASPECT_RATIO) {
        /* Limited by Width */

        return Vec2 {
            x: new_height * ASPECT_RATIO,
            y: new_height,
        };
    } else {
        /* Limited by Height */

        return Vec2 {
            x: new_width,
            y: new_width / ASPECT_RATIO,
        };
    }
}

impl Default for Pong {
    fn default() -> Self {
        Self {
            player_left: Player::default(),
            player_right: Player {
                side: player::Side::RightSide,
                ..Default::default()
            },
            ball: Ball::default(),
            scaling: calculate_texture_scaling(screen_width(), screen_height(), false),
            use_integer_scaling: false,
        }
    }
}
