use crate::*;

use crate::prelude::camera::set_xy;

pub fn center_camera(center: &Vector2, tilemap: &TileMap) {
    let camera_center = tilemap.lock_viewport_to_tilemap(&Vector2::new( center.x + 8., center.y + 8.), &Vector2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32));

    camera::set_xy(camera_center.x, camera_center.y);
}