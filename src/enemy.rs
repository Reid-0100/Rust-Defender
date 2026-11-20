
pub const ENEMY_SIZE: f32 = 35.0;
const ENEMY_SPEED: f32 = 150.0;
use macroquad::prelude::*;
use crate::collision::Rect;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    speed: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            speed: ENEMY_SPEED,
        }
    }
   
    pub fn update(&mut self) {
        self.y += self.speed * get_frame_time();
    }
  
    pub fn draw(&self) {
        draw_rectangle(
            self.x - ENEMY_SIZE / 2.0,
            self.y - ENEMY_SIZE / 2.0,
            ENEMY_SIZE,
            ENEMY_SIZE,
            GREEN,
        );
    }
  
    pub fn bounding_box(&self) -> Rect {
        Rect {
            x: self.x - ENEMY_SIZE / 2.0,
            y: self.y - ENEMY_SIZE / 2.0,
            width: ENEMY_SIZE,
            height: ENEMY_SIZE,
        }
    }

}
