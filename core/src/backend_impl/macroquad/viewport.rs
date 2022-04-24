use macroquad::window::{screen_height, screen_width};

use crate::camera::camera_position;

use crate::viewport::Viewport;

pub fn viewport() -> Viewport {
    let position = camera_position();

    Viewport {
        position,
        width: screen_width(),
        height: screen_height(),
    }
}