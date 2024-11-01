pub mod window {
    use macroquad::prelude::{screen_height, screen_width};

    pub fn width() -> f32 {
        screen_width()
    }

    pub fn height() -> f32 {
        screen_height()
    }
}

pub mod grid {
    use super::window;

    pub const NUM_TILES: usize = 65;
    pub const TILE_THICKNESS: f32 = 1.0;

    pub fn size() -> f32 {
        window::width() / 2.0
    }

    pub fn y_pos() -> f32 {
        (window::height() - size()) / 2.0
    }

    pub fn x_pos() -> f32 {
        window::width() / 4.0
    }
}

pub mod buttons {
    use super::grid;

    pub const NUM_BUTTONS: usize = 4;

    pub fn width() -> f32 {
        grid::size() / 3.0
    }

    pub fn height() -> f32 {
        grid::size() / 6.0
    }

    pub fn right_buttons_x() -> f32 {
        grid::x_pos() as f32 + grid::size() + 100.0
    }

    pub fn button_distance_y() -> f32 {
        (grid::size() - (NUM_BUTTONS as f32 * height())) / (NUM_BUTTONS as f32 - 1.0) + height()
    }
}
