use macroquad::prelude::*;

// structs
struct Player {
    position: Vec2,
    size: f32, speed: f32,
    health: i32, max_health: i32,
}

struct Enemy {
    position: Vec2,
    size: f32, speed: f32,
}

struct Game {
    player: Player,
    arena: Arena,
    enemies: Vec<Enemy>,

    spawn_timer: f32,
    spawn_interval: f32,

    damage_timer: f32,
    damage_interval: f32,
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
            health: 100,
            max_health: 100,
        };

        Self {
            player,
            arena,
            enemies: Vec::new(),

            spawn_timer: 0.0,
            spawn_interval: 2.0,

            damage_timer: 0.0,
            damage_interval: 0.5,
        }
    }

    fn update(&mut self, dt: f32) {
        if self.player.health <= 0 {
            return;
        }

        self.arena.update();
        self.player.update(dt);

        // update spawn timer
        self.spawn_timer += dt;

        // spawn enemy when interval has elapsed
        if self.spawn_timer >= self.spawn_interval {
            self.spawn_timer -= self.spawn_interval;

            let position = random_spawn_positions(
                self.arena.width,
                self.arena.height,
            );

            self.enemies.push(Enemy::new(position));
        }

        // update existing enemies
        let player_position = self.player.position;

        for enemy in &mut self.enemies {
            enemy.update(player_position, dt);
        }

        self.separate_enemies();

        // update damage cooldown
        if self.damage_timer > 0.0 {
            self.damage_timer = (self.damage_timer - dt).max(0.0);
        }

        // check collisions with enemies
        for enemy in &self.enemies {
            let collision = check_collision(
                self.player.position, self.player.size,
                enemy.position, enemy.size,
            );

            if collision && self.damage_timer <= 0.0 {
                self.player.health -= 10;
                self.player.health = self.player.health.max(0);

                // calculate direction from enemy to player
                let player_center = self.player.position + Vec2::splat(self.player.size / 2.0);
                let enemy_center = enemy.position + Vec2::splat(enemy.size / 2.0);

                let mut knockback_dir = player_center - enemy_center;

                // handle the rare case where both the centers are identical
                if knockback_dir.length_squared() > 0.0 {
                    knockback_dir = knockback_dir.normalize();
                }
                else {
                    knockback_dir = Vec2::new(1.0, 0.0);
                }

                // push the player away from enemy
                self.player.position += knockback_dir * 25.0;

                // keep player inside the arena
                self.player.position.x = self.player.position.x.clamp(
                    0.0,
                    self.arena.width - self.player.size,
                );

                self.player.position.y = self.player.position.y.clamp(
                    0.0,
                    self.arena.height - self.player.size,
                );

                self.damage_timer = self.damage_interval;
                break;
            }
        }
    }

    fn draw(&self) {
        self.arena.draw();
        self.player.draw();

        for enemy in &self.enemies {
            enemy.draw();
        }

        draw_text(
            &format!("HP: {}/{}", self.player.health, self.player.max_health),
            20.0, 30.0,
            25.0,
            WHITE,
        );

        if self.player.health <= 0 {
            draw_text(
                "GAME OVER",
                screen_width() / 2.0 - 80.0,
                screen_height() / 2.0,
                40.0,
                RED,
            );
        }
    }

    fn separate_enemies(&mut self) {
        let count = self.enemies.len();

        for i in 0..count {
            for j in (i + 1)..count {
                // split the vector so that we can mutably access 2 different enemies at the same time
                let (left, right) = self.enemies.split_at_mut(j);

                let enemy_a = &mut left[i];
                let enemy_b = &mut right[0];

                let overlap_x = (enemy_a.position.x + enemy_a.size).min(enemy_b.position.x + enemy_b.size)
                                - (enemy_a.position.x).max(enemy_b.position.x);

                let overlap_y = (enemy_a.position.y + enemy_a.size).min(enemy_b.position.y + enemy_b.size)
                                - (enemy_a.position.y).max(enemy_b.position.y);

                // only resolve actual overlaps
                if overlap_x > 0.0 && overlap_y > 0.0 {
                    let center_a = enemy_a.position + Vec2::splat(enemy_a.size / 2.0);
                    let center_b = enemy_b.position + Vec2::splat(enemy_b.size / 2.0);

                    // resolve along the axis with less overlap
                    if overlap_x < overlap_y {
                        let push = overlap_x / 2.0;

                        if center_a.x < center_b.x {
                            enemy_a.position.x -= push;
                            enemy_b.position.x += push;
                        }
                        else {
                            enemy_a.position.x += push;
                            enemy_b.position.x -= push;
                        }
                    }
                    else {
                        let push = overlap_y / 2.0;

                        if center_a.y < center_b.y {
                            enemy_a.position.y -= push;
                            enemy_b.position.y += push;
                        }
                        else {
                            enemy_a.position.y += push;
                            enemy_b.position.y -= push;
                        }
                    }
                }
            }
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

fn random_spawn_positions(width: f32, height: f32) -> Vec2 {
    let edge = macroquad::rand::gen_range(0, 4);

    match edge {
        0 => Vec2::new(
            macroquad::rand::gen_range(0.0, width),
            0.0,
        ),

        1 => Vec2::new(
            width - 30.0,
            macroquad::rand::gen_range(0.0, height),
        ),

        2 => Vec2::new(
            macroquad::rand::gen_range(0.0, width),
            height - 30.0,
        ),

        _ => Vec2::new(
            0.0,
            macroquad::rand::gen_range(0.0, height),
        ),
    }
}

fn check_collision(
    pos_a: Vec2, size_a: f32,
    pos_b: Vec2, size_b: f32,
) -> bool {
    pos_a.x < pos_b.x + size_b &&
    pos_a.x + size_a > pos_b.x &&
    pos_a.y < pos_b.y + size_b &&
    pos_a.y + size_a > pos_b.y
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