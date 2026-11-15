use macroquad::prelude::*;

pub(crate) struct Player {
    pub(crate) position: Vec2,
    pub(crate) size: f32,
    pub(crate) speed: f32,
    pub(crate) health: i32,
    pub(crate) max_health: i32,
}

impl Player {
    pub(crate) fn new(position: Vec2) -> Self {
        Self {
            position,
            size: 40.0,
            speed: 250.0,
            health: 100,
            max_health: 100,
        }
    }

    pub(crate) fn update(&mut self, dt: f32) {
        let mut direction = Vec2::ZERO;

        // handle keyboard input
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            direction.x += 1.0;
        }

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            direction.x -= 1.0;
        }

        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            direction.y += 1.0;
        }

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            direction.y -= 1.0;
        }

        // normalize direction if the player is moving
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        // apply movement
        self.position += direction * self.speed * dt;

        // keep player inside the window
        self.position.x = self.position.x.clamp(0.0, screen_width() - self.size);

        self.position.y = self.position.y.clamp(0.0, screen_height() - self.size);
    }

    pub(crate) fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size,
            self.size,
            GREEN,
        );
    }
}
