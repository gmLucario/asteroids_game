use crate::constants::SHIP_HEIGHT;
use crate::utils::wrap_around;
use std::slice::{Iter, IterMut};

use macroquad::prelude::{draw_poly_lines, rand, screen_height, screen_width, Vec2, BLACK};

pub struct Asteroid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub size: f32,
    pub sides: u8,
    pub collided: bool,
}

impl Asteroid {
    pub fn new_random(screen_center: Vec2) -> Self {
        Self {
            position: screen_center
                + Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize()
                    * screen_width().min(screen_height())
                    / 2.0,
            velocity: Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)),
            rotation: 0.0,
            rotation_speed: rand::gen_range(-2.0, 2.0),
            size: screen_width().min(screen_height()) / 10.0,
            sides: rand::gen_range(3, 8),
            collided: false,
        }
    }

    pub fn move_asteroids(asteroids: IterMut<Self>) {
        for asteroid in asteroids {
            asteroid.position += asteroid.velocity;
            asteroid.position = wrap_around(&asteroid.position);
            asteroid.rotation += asteroid.rotation_speed;
        }
    }

    pub fn draw_asteroids(asteroids: Iter<Self>) {
        for asteroid in asteroids {
            draw_poly_lines(
                asteroid.position.x,
                asteroid.position.y,
                asteroid.sides,
                asteroid.size,
                asteroid.rotation,
                2.0,
                BLACK,
            )
        }
    }

    pub fn collided_ship(&self, ship_position: Vec2) -> bool {
        (self.position - ship_position).length() < self.size + SHIP_HEIGHT / 3.0
    }
}
