use macroquad::prelude::*;

mod arena;
mod collision;
mod enemy;
mod game;
mod player;
mod projectile;

use game::Game;

#[macroquad::main("rust arena")]
async fn main() {
    let mut game = Game::new();

    loop {
        let dt = get_frame_time();

        game.update(dt);
        game.draw();

        next_frame().await;
    }
}
