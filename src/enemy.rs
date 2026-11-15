use macroquad::prelude::*;

pub(crate) struct Enemy {
    pub(crate) position: Vec2,
    pub(crate) size: f32,
    pub(crate) speed: f32,
    pub(crate) health: i32,
}

impl Enemy {
    pub(crate) fn new(position: Vec2) -> Self {
        Self {
            position,
            size: 30.0,
            speed: 100.0,
            health: 30,
        }
    }

    pub(crate) fn update(&mut self, player_position: Vec2, dt: f32) {
        // calculate direction towards player
        let mut direction = player_position - self.position;

        // normalize direction
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        // move towards player
        self.position += direction * self.speed * dt;
    }

    pub(crate) fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size, self.size, RED);
    }
}

pub(crate) fn random_spawn_positions(width: f32, height: f32) -> Vec2 {
    let edge = macroquad::rand::gen_range(0, 4);

    match edge {
        0 => Vec2::new(macroquad::rand::gen_range(0.0, width), 0.0),

        1 => Vec2::new(width - 30.0, macroquad::rand::gen_range(0.0, height)),

        2 => Vec2::new(macroquad::rand::gen_range(0.0, width), height - 30.0),

        _ => Vec2::new(0.0, macroquad::rand::gen_range(0.0, height)),
    }
}
