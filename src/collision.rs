use macroquad::prelude::*;

pub(crate) fn check_collision(pos_a: Vec2, size_a: f32, pos_b: Vec2, size_b: f32) -> bool {
    pos_a.x < pos_b.x + size_b
        && pos_a.x + size_a > pos_b.x
        && pos_a.y < pos_b.y + size_b
        && pos_a.y + size_a > pos_b.y
}
