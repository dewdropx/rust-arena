use macroquad::prelude::*;

pub(crate) struct Projectile {
    pub(crate) position: Vec2,
    velocity: Vec2,
    size: f32,
    pub(crate) damage: i32,
    pub(crate) lifetime: f32,
}

impl Projectile {
    pub(crate) fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            velocity: direction * 500.0,
            size: 10.0,
            damage: 10,
            lifetime: 2.0,
        }
    }

    pub(crate) fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.lifetime -= dt;
    }

    pub(crate) fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size,
            self.size,
            YELLOW,
        );
    }

    pub(crate) fn size(&self) -> f32 {
        self.size
    }
}
