use macroquad::prelude::*;

pub(crate) struct Arena {
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl Arena {
    pub(crate) fn new() -> Self {
        Self {
            width: screen_width(),
            height: screen_height(),
        }
    }

    pub(crate) fn update(&mut self) {
        self.width = screen_width();
        self.height = screen_height();
    }

    pub(crate) fn draw(&self) {
        clear_background(Color::from_rgba(18, 22, 30, 255));

        let grid_size = 40.0;

        // vertical grid lines
        let mut x = 0.0;

        while x <= self.width {
            draw_line(
                x,
                0.0,
                x,
                self.height,
                1.0,
                Color::from_rgba(45, 52, 65, 255),
            );

            x += grid_size;
        }

        // horizontal grid lines
        let mut y = 0.0;

        while y <= self.height {
            draw_line(
                0.0,
                y,
                self.width,
                y,
                1.0,
                Color::from_rgba(45, 52, 65, 255),
            );

            y += grid_size;
        }

        // arena border
        draw_rectangle_lines(0.0, 0.0, self.width, self.height, 3.0, WHITE);
    }
}
