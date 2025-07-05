use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Tile {
    pub solid: Solid,
}

impl Tile {
    pub fn new(position: Vector2, width: f32, height: f32) -> Self {
        Self {
            solid: Solid {
                position,
                width,
                height,
            }
        }
    }
    
    pub fn draw(&self) {
        let x = (self.solid.position.x - self.solid.width / 2.) as i32;
        let y = (self.solid.position.y - self.solid.height / 2.) as i32;

        sprite!("dirt", x = x, y = y);
        rect!(w = 1, h = 1, x = x, y = y, color = 0xff00ffff);
    }
}