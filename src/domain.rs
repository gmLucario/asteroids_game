use crate::models;
use crate::utils;

use macroquad::prelude::{clear_background, is_key_down, next_frame, KeyCode, Vec2, LIGHTGRAY};

pub struct GameProcessor {
    pub ship: models::ship::Ship,
    pub bullets: Vec<models::bullet::Bullet>,
    pub last_shot: f64,
    pub asteroids: Vec<models::asteroid::Asteroid>,
    pub gameover: bool,
    pub screen_center: Vec2,
}

impl GameProcessor {
    pub fn new() -> Self {
        Self {
            ship: models::ship::Ship::new(),
            bullets: Vec::new(),
            last_shot: utils::get_elapsed_time(),
            asteroids: Vec::new(),
            gameover: false,
            screen_center: utils::get_screen_center(),
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.gameover = self.asteroids.is_empty() || self.gameover;

            if self.gameover {
                self.show_end_game_view(!self.asteroids.is_empty());

                if is_key_down(KeyCode::Enter) {
                    self.reset_game_values();
                    self.fill_asteroids();
                }
                next_frame().await;
                continue;
            }

            self.draw_entities().await;

            let current_time: f64 = utils::get_elapsed_time();
            if is_key_down(KeyCode::Space) && current_time - self.last_shot > 0.5 {
                self.bullets.push(models::bullet::Bullet::new_default(
                    &self.ship,
                    current_time,
                ));

                self.last_shot = current_time;
            }

            self.ship.update_ship_rotation();

            self.ship.velocity += self.ship.get_acceleration();
            self.ship.update_ship_position();

            models::bullet::Bullet::move_bullets(self.bullets.iter_mut());

            models::asteroid::Asteroid::move_asteroids(self.asteroids.iter_mut());

            self.bullets.retain(|bullet| bullet.is_alive(current_time));

            let mut asteroid_bullet_collided: bool;
            for asteroid in self.asteroids.iter_mut() {
                if asteroid.collided_ship(self.ship.position) {
                    self.gameover = true;
                    break;
                }

                // Asteroid/bullet collision
                for bullet in self.bullets.iter_mut() {
                    asteroid_bullet_collided =
                        (asteroid.position - bullet.position).length() < asteroid.size;

                    asteroid.collided = asteroid_bullet_collided || asteroid.collided;
                    bullet.collided = asteroid_bullet_collided || bullet.collided;

                    if asteroid_bullet_collided {
                        break;
                    }
                }
            }

            self.remvove_collided(current_time);
        }
    }

    fn show_end_game_view(&self, are_asteroids: bool) {
        clear_background(LIGHTGRAY);
        let message: &str = {
            if are_asteroids {
                "Game Over. Press [enter] to play again."
            } else {
                "You Win!. Press [enter] to play again."
            }
        };
        let font_size: f64 = 30.0;

        utils::draw_message(message, font_size);
    }

    fn reset_game_values(&mut self) {
        *self = Self::new();
    }

    fn fill_asteroids(&mut self) {
        for _ in 0..10 {
            self.asteroids
                .push(models::asteroid::Asteroid::new_random(self.screen_center))
        }
    }

    fn remvove_collided(&mut self, current_time: f64) {
        self.bullets
            .retain(|bullet| bullet.shot_at + 1.5 > current_time && !bullet.collided);
        self.asteroids.retain(|asteroid| !asteroid.collided);
    }

    async fn draw_entities(&self) {
        clear_background(LIGHTGRAY);

        self.ship.draw();
        models::bullet::Bullet::draw_bullets(self.bullets.iter());
        models::asteroid::Asteroid::draw_asteroids(self.asteroids.iter());

        next_frame().await
    }
}
