mod player;

use macroquad::{prelude::*, window::clear_background};

use crate::{screen::View, State};

use self::player::Player;

const ASPECT_RATIO: f32 = 2.0;
const GAME_WIDTH: u16 = 512;
const GAME_HEIGHT: u16 = 256;
const PADDLE_WIDTH: f32 = 2.0;
const PADDLE_HEIGHT: f32 = 28.0;
const BALL_DIAMETER: f32 = 5.0;

pub struct Pong {
    player_left: Player,
    player_right: Player,
    scaling: Option<Vec2>,
}

impl View for Pong {
    fn draw(&self) {
        clear_background(BLACK);

        let mut img = Image::gen_image_color(GAME_WIDTH, GAME_HEIGHT, GRAY);

        self.player_left.draw(&mut img);
        self.player_right.draw(&mut img);

        draw_texture_ex(
            Texture2D::from_image(&img),
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: self.scaling,
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }

    fn handle_input(&mut self) -> Option<crate::State> {
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

        return None;
    }

    fn on_resize(&mut self, new_width: f32, new_height: f32) {
        if new_width > (new_height * ASPECT_RATIO) {
            /* Limited by Width */

            self.scaling = Some(Vec2 {
                x: new_height * ASPECT_RATIO,
                y: new_height,
            });
        } else {
            /* Limited by Height */

            self.scaling = Some(Vec2 {
                x: new_width,
                y: new_width / ASPECT_RATIO,
            });
        }
        println!("resizing to x: {}, y: {}", new_width, new_height);
        println!("new Scaling set to: {:?}", self.scaling);
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
            scaling: None,
        }
    }
}
