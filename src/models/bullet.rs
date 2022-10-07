use std::slice::{Iter, IterMut};

use crate::constants::{BULLET_RADIUS, SHIP_HEIGHT};
use crate::models::ship::Ship;
use macroquad::prelude::{draw_circle, Vec2, RED};

pub struct Bullet {
    pub position: Vec2,
    pub velocity: Vec2,
    pub shot_at: f64,
    pub collided: bool,
}

impl Bullet {
    pub fn new_default(ship: &Ship, current_time: f64) -> Self {
        let ship_pos: Vec2 = ship.position;
        let ship_rotation: f32 = ship.rotation.to_radians();
        let rotation = Vec2::new(ship_rotation.sin(), -ship_rotation.cos());

        Self {
            position: ship_pos + rotation * SHIP_HEIGHT / 2.0,
            velocity: rotation * 7.0,
            shot_at: current_time,
            collided: false,
        }
    }

    pub fn is_alive(&self, current_time: f64) -> bool {
        self.shot_at + 1.5 > current_time
    }

    pub fn move_bullets(bullets: IterMut<Self>) {
        for bullet in bullets {
            bullet.position += bullet.velocity;
        }
    }

    pub fn draw_bullets(bullets: Iter<Self>) {
        for bullet in bullets {
            draw_circle(bullet.position.x, bullet.position.y, BULLET_RADIUS, RED);
        }
    }
}
