use std::collections::HashMap;

use turbo::sprite;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn draw_hud(camera_center_x: f32, camera_center_y: f32) {
    sprite!(
        "energybar01",
        x = (camera_center_x - SCREEN_WIDTH as f32 / 2.) as i32, 
        y = (camera_center_y - SCREEN_HEIGHT as f32 / 2.) as i32,
    );
}