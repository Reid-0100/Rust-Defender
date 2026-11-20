use macroquad::prelude::*;

use crate::collision::Rect;

const PLAYER_HALF_SIZE: f32 = 30.0;

pub struct Player {
    x: f32,
    y: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn draw(&self) {
    draw_triangle(
        vec2(self.x, self.y - 30.0),
        vec2(self.x - 30.0, self.y + 30.0),
        vec2(self.x + 30.0, self.y + 30.0),
        RED,
    );
    }
    
    pub fn update(&mut self) {
        let speed = 300.0 * get_frame_time();

        // Right
        if is_key_down(KeyCode::Right) {
            self.x += speed;
        }

        // Left
        if is_key_down(KeyCode::Left) {
            self.x -= speed;
        }

        // Up
        if is_key_down(KeyCode::Up) {
            self.y -= speed;
        }

        // Down
        if is_key_down(KeyCode::Down) {
            self.y += speed;
        }

        // Keep the player inside the window
        self.x = self.x.clamp(30.0, screen_width() - 30.0);
        self.y = self.y.clamp(30.0, screen_height() - 30.0);
    }

    
    pub fn fire_position(&self) -> (f32, f32) {
        (self.x, self.y - 30.0)
    }

    
    pub fn bounding_box(&self) -> Rect {
        Rect {
            x: self.x - PLAYER_HALF_SIZE,
            y: self.y - PLAYER_HALF_SIZE,
            width: PLAYER_HALF_SIZE * 2.0,
            height: PLAYER_HALF_SIZE * 2.0,
        }
    }


}