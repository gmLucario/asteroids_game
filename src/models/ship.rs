use crate::constants::{HALF_SHIP_BASE, HALF_SHIP_HEIGHT};
use crate::utils::wrap_around;
use macroquad::prelude::{
    draw_triangle_lines, is_key_down, screen_height, screen_width, KeyCode, Vec2, BLACK,
};

pub struct Ship {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
            rotation: 0.0,
            velocity: Vec2::new(0.0, 0.0),
        }
    }

    pub fn draw(&self) {
        let rotation: f32 = self.rotation.to_radians();
        let v1 = Vec2::new(
            self.position.x + rotation.sin() * HALF_SHIP_HEIGHT,
            self.position.y - rotation.cos() * HALF_SHIP_HEIGHT,
        );

        let v2 = Vec2::new(
            self.position.x - rotation.cos() * HALF_SHIP_BASE - rotation.sin() * HALF_SHIP_HEIGHT,
            self.position.y - rotation.sin() * HALF_SHIP_BASE + rotation.cos() * HALF_SHIP_HEIGHT,
        );
        let v3 = Vec2::new(
            self.position.x + rotation.cos() * HALF_SHIP_BASE - rotation.sin() * HALF_SHIP_HEIGHT,
            self.position.y + rotation.sin() * HALF_SHIP_BASE + rotation.cos() * HALF_SHIP_HEIGHT,
        );
        draw_triangle_lines(v1, v2, v3, 2.0, BLACK);
    }

    pub fn get_acceleration(&self) -> Vec2 {
        let acceleration: Vec2 = {
            if is_key_down(KeyCode::Up) {
                // Forward
                let rotation: f32 = self.rotation.to_radians();
                Vec2::new(rotation.sin(), -rotation.cos()) / 3.0
            } else {
                // Friction
                -self.velocity / 100.0
            }
        };

        acceleration
    }

    pub fn update_ship_rotation(&mut self) {
        if is_key_down(KeyCode::Right) {
            self.rotation += 5.0;
        } else if is_key_down(KeyCode::Left) {
            self.rotation -= 5.0;
        }
    }

    pub fn update_ship_position(&mut self) {
        if self.velocity.length() > 5.0 {
            self.velocity = self.velocity.normalize() * 5.0;
        }

        self.position += self.velocity;
        self.position = wrap_around(&self.position);
    }
}
