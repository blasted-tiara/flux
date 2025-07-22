use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Background {
    pub layers: Vec<Layer>,
    pub default_color: u32,
}

impl Background {
    pub fn new(default_color: u32) -> Self {
        Self {
            layers: vec!(),
            default_color,
        }
    }
    
    pub fn draw(&self, camera_center: Vector2) {
        clear(self.default_color);
        for layer in &self.layers {
            sprite!(layer.sprite.as_str(), x = layer.offset_x + (camera_center.x * layer.speed_x) as i32, y = layer.offset_y);
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Layer {
    pub sprite: String,
    pub speed_x: f32,
    pub speed_y: f32,
    pub offset_x: i32,
    pub offset_y: i32,
}