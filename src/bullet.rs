pub const BULLET_WIDTH: f32 = 6.0;
pub const BULLET_HEIGHT: f32 = 16.0;

use crate::collision::Rect;

pub struct Bullet {
    pub x: f32,
    pub y: f32,
    speed: f32,
}


impl Bullet {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            speed: 500.0,
        }
    }

    
    pub fn update(&mut self) {
        self.y -= self.speed * macroquad::prelude::get_frame_time();
    }
    
    
    pub fn draw(&self) {
    macroquad::prelude::draw_rectangle(
        self.x - BULLET_WIDTH / 2.0,
        self.y - BULLET_HEIGHT / 2.0,
        BULLET_WIDTH,
        BULLET_HEIGHT,
        macroquad::prelude::YELLOW,
    );
    }

    
    pub fn bounding_box(&self) -> Rect {
        Rect {
            x: self.x - BULLET_WIDTH / 2.0,
            y: self.y - BULLET_HEIGHT / 2.0,
            width: BULLET_WIDTH,
            height: BULLET_HEIGHT,
        }
    }


}
