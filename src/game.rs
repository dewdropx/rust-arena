use macroquad::prelude::*;

use crate::arena::Arena;
use crate::collision::check_collision;
use crate::enemy::{Enemy, random_spawn_positions};
use crate::player::Player;
use crate::projectile::Projectile;

pub(crate) struct Game {
    player: Player,
    arena: Arena,
    enemies: Vec<Enemy>,

    spawn_timer: f32,
    spawn_interval: f32,

    damage_timer: f32,
    damage_interval: f32,

    projectiles: Vec<Projectile>,

    attack_timer: f32,
    attack_interval: f32,
}

impl Game {
    pub(crate) fn new() -> Self {
        let arena = Arena::new();
        let player_size = 40.0;

        let player_position = Vec2::new(
            (arena.width - player_size) / 2.0,
            (arena.height - player_size) / 2.0,
        );

        let player = Player::new(player_position);

        Self {
            player,
            arena,
            enemies: Vec::new(),

            spawn_timer: 0.0,
            spawn_interval: 2.0,

            damage_timer: 0.0,
            damage_interval: 0.5,

            projectiles: Vec::new(),

            attack_timer: 0.0,
            attack_interval: 0.8,
        }
    }

    pub(crate) fn update(&mut self, dt: f32) {
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

            let position = random_spawn_positions(self.arena.width, self.arena.height);

            self.enemies.push(Enemy::new(position));
        }

        // update existing enemies
        let player_position = self.player.position;

        for enemy in &mut self.enemies {
            enemy.update(player_position, dt);
        }

        self.separate_enemies();

        self.attack_timer += dt;

        if self.attack_timer >= self.attack_interval {
            self.attack_timer -= self.attack_interval;
            self.auto_attack();
        }

        // update projectile positions
        for projectile in &mut self.projectiles {
            projectile.update(dt);
        }

        // check projectile enemy collisions
        for projectile in &mut self.projectiles {
            for enemy in &mut self.enemies {
                let collision = check_collision(
                    projectile.position,
                    projectile.size(),
                    enemy.position,
                    enemy.size,
                );

                if collision {
                    enemy.health -= projectile.damage;

                    // remove projectile after successful hit
                    projectile.lifetime = 0.0;
                    break;
                }
            }
        }

        // remove expired projectiles
        self.projectiles.retain(|p| p.lifetime > 0.0);

        // remove defeated enemies
        self.enemies.retain(|e| e.health > 0);

        // update damage cooldown
        if self.damage_timer > 0.0 {
            self.damage_timer = (self.damage_timer - dt).max(0.0);
        }

        // check collisions with enemies
        for enemy in &self.enemies {
            let collision = check_collision(
                self.player.position,
                self.player.size,
                enemy.position,
                enemy.size,
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
                } else {
                    knockback_dir = Vec2::new(1.0, 0.0);
                }

                // push the player away from enemy
                self.player.position += knockback_dir * 25.0;

                // keep player inside the arena
                self.player.position.x = self
                    .player
                    .position
                    .x
                    .clamp(0.0, self.arena.width - self.player.size);

                self.player.position.y = self
                    .player
                    .position
                    .y
                    .clamp(0.0, self.arena.height - self.player.size);

                self.damage_timer = self.damage_interval;
                break;
            }
        }
    }

    pub(crate) fn draw(&self) {
        self.arena.draw();
        self.player.draw();

        for enemy in &self.enemies {
            enemy.draw();
        }

        for projectile in &self.projectiles {
            projectile.draw();
        }

        // HUD
        draw_text(
            &format!("HP: {}/{}", self.player.health, self.player.max_health),
            20.0,
            30.0,
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

                let overlap_x = (enemy_a.position.x + enemy_a.size)
                    .min(enemy_b.position.x + enemy_b.size)
                    - (enemy_a.position.x).max(enemy_b.position.x);

                let overlap_y = (enemy_a.position.y + enemy_a.size)
                    .min(enemy_b.position.y + enemy_b.size)
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
                        } else {
                            enemy_a.position.x += push;
                            enemy_b.position.x -= push;
                        }
                    } else {
                        let push = overlap_y / 2.0;

                        if center_a.y < center_b.y {
                            enemy_a.position.y -= push;
                            enemy_b.position.y += push;
                        } else {
                            enemy_a.position.y += push;
                            enemy_b.position.y -= push;
                        }
                    }
                }
            }
        }
    }

    fn auto_attack(&mut self) {
        if self.enemies.is_empty() {
            return;
        }

        // find the center of the player
        let player_center = self.player.position + Vec2::splat(self.player.size / 2.0);

        // find nearest enemy
        let mut target_position =
            self.enemies[0].position + Vec2::splat(self.enemies[0].size / 2.0);
        let mut closest_distance = player_center.distance_squared(target_position);

        for enemy in &self.enemies {
            let enemy_center = enemy.position + Vec2::splat(enemy.size / 2.0);
            let distance = player_center.distance_squared(enemy_center);

            if distance < closest_distance {
                closest_distance = distance;
                target_position = enemy_center;
            }
        }

        // calculate the direction towards the target
        let mut direction = target_position - player_center;

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();

            self.projectiles
                .push(Projectile::new(player_center, direction));
        }
    }
}
