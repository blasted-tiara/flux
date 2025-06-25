use crate::*;

use crate::prelude::camera::set_xy;

pub fn center_camera(center: &Vector2) {
    // Subtract half the width of the canvas, then add half the size of the player to center the camera
    camera::set_xy(center.x + 8., center.y + 8.);
}