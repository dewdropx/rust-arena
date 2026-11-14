use macroquad::prelude::*;

struct Player {
    position: Vec2,
    size: f32, speed: f32,
}

struct Game {
    player: Player,
    arena: Arena,
}

struct Arena {
    width: f32,
    height: f32,
}

impl Player {
    fn update(&mut self, dt: f32) {
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
        self.position.x = self.position.x.clamp(
            0.0, screen_width() - self.size
        );

        self.position.y = self.position.y.clamp(
            0.0, screen_height() - self.size
        );
    }

    fn draw(&self) {
        draw_rectangle(
            self.position.x, self.position.y,
            self.size, self.size,
            GREEN,
        );
    }
}

impl Game {
    fn new() -> Self {
        let arena = Arena::new();
        let player_size = 40.0;

        let player = Player {
            position: Vec2::new(
                (arena.width - player_size) / 2.0,
                (arena.height - player_size) / 2.0,
            ),
            size: player_size,
            speed: 250.0,
        };

        Self {
            player,
            arena,
        }
    }

    fn update(&mut self, dt: f32) {
        self.arena.update();
        self.player.update(dt);
    }

    fn draw(&self) {
        self.arena.draw();
        self.player.draw();
    }
}

impl Arena {
    fn new() -> Self {
        Self {
            width: screen_width(),
            height: screen_height(),
        }
    }

    fn update(&mut self) {
        self.width = screen_width();
        self.height = screen_height();
    }

    fn draw(&self) {
        clear_background(Color::from_rgba(18, 22, 30, 255));

        let grid_size = 40.0;

        // vertical grid lines
        let mut x = 0.0;

        while x <= self.width {
            draw_line(
                x, 0.0,
                x, self.height,
                1.0,
                Color::from_rgba(45, 52, 65, 255),
            );

            x += grid_size;
        }

        // horizontal grid lines
        let mut y = 0.0;

        while y <= self.height {
            draw_line(
                0.0, y,
                self.width, y,
                1.0,
                Color::from_rgba(45, 52, 65, 255),
            );

            y += grid_size;
        }

        // arena border
        draw_rectangle_lines(
            0.0, 0.0,
            self.width, self.height,
            3.0,
            WHITE,
        );
    }
}

#[macroquad::main("rust arena")]
async fn main() {
    // create new instance of game
    let mut game = Game::new();

    loop {
        // frame rate independent movement
        let dt = get_frame_time(); // time elapsed since prev frame

        // update game state
        game.update(dt);
        game.draw();

        next_frame().await; // move to next frame
    }
}