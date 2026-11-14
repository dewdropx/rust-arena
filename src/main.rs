use macroquad::prelude::*;

#[macroquad::main("rust arena")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_text(
            "RUST ARENA",
            30.0, 50.0, 40.0,
            WHITE,
        );

        next_frame().await;
    }
}