use macroquad::prelude::*;

struct Player {
    x: f32, y: f32,
    size: f32, speed: f32,
}

impl Player {
    fn update(&mut self, dt: f32) {
        // handle keyboard input
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.x += self.speed * dt;
        }

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.x -= self.speed * dt;
        }

        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.y += self.speed * dt;
        }

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.y -= self.speed * dt;
        }

        self.x = self.x.clamp(
            0.0, screen_width() - self.size
        );

        self.y = self.y.clamp(
            0.0, screen_height() - self.size
        );
    }

    fn draw(&self) {
        draw_rectangle(
            self.x, self.y,
            self.size, self.size,
            GREEN,
        );
    }
}

#[macroquad::main("rust arena")]
async fn main() {
    // create a player instance
    let mut player = Player {
        x: 200.0, y: 200.0,
        size: 40.0, speed: 250.0,
    };

    loop {
        // frame rate independent movement
        let dt = get_frame_time(); // time elapsed since prev frame

        // update player coordinates
        player.update(dt);

        // rendering
        clear_background(BLACK);

        // render the player
        player.draw();

        next_frame().await; // move to next frame
    }
}