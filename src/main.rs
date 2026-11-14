use macroquad::prelude::*;

// structs
struct Player {
    position: Vec2,
    size: f32, speed: f32,
}

struct Enemy {
    position: Vec2,
    size: f32, speed: f32,
}

struct Game {
    player: Player,
    arena: Arena,
    enemies: Vec<Enemy>,
}

struct Arena {
    width: f32,
    height: f32,
}

// methods
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

impl Enemy {
    fn new(position: Vec2) -> Self {
        Self {
            position,
            size: 30.0,
            speed: 100.0,
        }
    }

    fn update(&mut self, player_position: Vec2, dt: f32) {
        // calculate direction towards player
        let mut direction = player_position - self.position;

        // normalize direction
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        // move towards player
        self.position += direction * self.speed * dt;
    }

    fn draw(&self) {
        draw_rectangle(
            self.position.x, self.position.y,
            self.size, self.size,
            RED,
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

        let enemies = vec![
            Enemy::new(Vec2::new(100.0, 100.0)),
            Enemy::new(Vec2::new(400.0, 150.0)),
            Enemy::new(Vec2::new(200.0, 400.0)),
        ];

        Self {
            player,
            arena,
            enemies,
        }
    }

    fn update(&mut self, dt: f32) {
        self.arena.update();
        self.player.update(dt);

        let player_position = self.player.position;

        for enemy in &mut self.enemies {
            enemy.update(player_position, dt);
        }
    }

    fn draw(&self) {
        self.arena.draw();
        self.player.draw();

        for enemy in &self.enemies {
            enemy.draw();
        }
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