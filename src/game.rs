
use crate::player::Player;
use crate::bullet::Bullet;
use crate::enemy::Enemy;

use macroquad::prelude::*;

const ENEMY_SPAWN_INTERVAL: f32 = 2.0;

const SCORE_PER_ENEMY: u32 = 10;
use crate::ui;

enum GameScreen {
    MainMenu,
    Playing,
    GameOver,
}

pub struct Game {
    player: Player,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    score: u32,
    spawn_timer: f32,
    
    screen: GameScreen,
}


impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(
                macroquad::prelude::screen_width() / 2.0,
                macroquad::prelude::screen_height() / 2.0,
            ),
            bullets: Vec::new(),
            enemies: Vec::new(),
            score: 0,
            spawn_timer: 0.0,
            
            screen: GameScreen::MainMenu,
        }
    }
    
    
    pub fn update(&mut self) {

        if matches!(self.screen, GameScreen::MainMenu) {
            if ui::start_button_clicked() {
                self.screen = GameScreen::Playing;
            }
            return;
        }

        // Stop updating when the game is over
        
        
        if matches!(self.screen, GameScreen::GameOver) {
            if is_key_pressed(KeyCode::R) || ui::play_again_clicked() {
                self.restart();
            } else if ui::exit_button_clicked() {
                macroquad::miniquad::window::order_quit();
            }

            return;
        }


        // Update player
        self.player.update();

        // Fire a bullet when Space is pressed
        if is_key_pressed(KeyCode::Space) {
            let (x, y) = self.player.fire_position();
            self.bullets.push(Bullet::new(x, y));
        }

        // Update bullets
        for bullet in self.bullets.iter_mut() {
            bullet.update();
        }

        // Remove bullets outside the screen
        self.bullets.retain(|bullet| {
            bullet.y >= -crate::bullet::BULLET_HEIGHT / 2.0
        });

        // Update enemy spawn timer
        self.spawn_timer += get_frame_time();

        // Spawn enemies at random positions
        if self.spawn_timer >= ENEMY_SPAWN_INTERVAL {
            let half_size = crate::enemy::ENEMY_SIZE / 2.0;

            let random_x = rand::gen_range(
                half_size,
                screen_width() - half_size,
            );

            self.enemies.push(Enemy::new(random_x, -half_size));

            self.spawn_timer -= ENEMY_SPAWN_INTERVAL;
        }

        // Update enemies
        for enemy in self.enemies.iter_mut() {
            enemy.update();
        }

        // Remove enemies outside the screen
        self.enemies.retain(|enemy| {
            enemy.y - crate::enemy::ENEMY_SIZE / 2.0 <= screen_height()
        });

        
        for bullet_index in (0..self.bullets.len()).rev() {
            for enemy_index in (0..self.enemies.len()).rev() {
                if self.bullets[bullet_index]
                    .bounding_box()
                    .intersects(&self.enemies[enemy_index].bounding_box())
                {
                    self.bullets.remove(bullet_index);
                    self.enemies.remove(enemy_index);

                    self.score += SCORE_PER_ENEMY;

                    break;
                }
            }
        }

        // Check collision between player and enemies
        if matches!(self.screen, GameScreen::Playing) {
            for enemy in &self.enemies {
                if self.player
                    .bounding_box()
                    .intersects(&enemy.bounding_box())
                {
                    

                    self.screen = GameScreen::GameOver;

                    println!("Game Over! Final Score: {}", self.score);

                    break;
                }
            }
        }

    }
    
    pub fn restart(&mut self) {
        *self = Self::new();
        self.screen = GameScreen::Playing;
    }

    
    
    pub fn draw(&self) {
        // UI settings
        const SCORE_FONT_SIZE: f32 = 30.0;
        const SCORE_X: f32 = 20.0;
        const SCORE_Y: f32 = 40.0;
        

        // Show the main menu
        if matches!(self.screen, GameScreen::MainMenu) {
            ui::draw_main_menu();
            return;
        }

        clear_background(BLACK);

        // Game Over screen
        
        if matches!(self.screen, GameScreen::GameOver) {
            ui::draw_game_over(self.score);
            return;
        }


        // Draw player
        self.player.draw();

        // Draw bullets
        for bullet in &self.bullets {
            bullet.draw();
        }

        // Draw enemies
        for enemy in &self.enemies {
            enemy.draw();
        }

        // Draw score
        draw_text(
            &format!("Score: {}", self.score),
            SCORE_X,
            SCORE_Y,
            SCORE_FONT_SIZE,
            WHITE,
        );
    }




}
