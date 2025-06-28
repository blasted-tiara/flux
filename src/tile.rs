use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Tile {
    grid_x: usize,
    grid_y: usize,
    tile_size_x: u32,
    tile_size_y: u32,
}

impl Tile {
    pub fn new(grid_x: usize, grid_y: usize, tile_size_x: u32, tile_size_y: u32) -> Self {
        Self { grid_x, grid_y, tile_size_x, tile_size_y }
    }
    
    pub fn top(self: &Self) -> f32 {
        (self.grid_y - 1) as f32 * self.tile_size_y as f32
    }
    
    pub fn contains(&self, point_x: f32, point_y: f32) -> bool {
        let tile_x = self.grid_x as f32 * self.tile_size_x as f32;
        let tile_y = self.grid_y as f32 * self.tile_size_y as f32;
        point_x >= tile_x
            && point_x < tile_x + self.tile_size_x as f32
            && point_y >= tile_y
            && point_y < tile_y + self.tile_size_y as f32
    }

    pub fn draw(&self) {
        let x = self.grid_x as i32 * self.tile_size_x as i32;
        let y = self.grid_y as i32 * self.tile_size_y as i32;

        sprite!("dirt", x = x, y = y);
        rect!(w = 1, h = 1, x = x, y = y, color = 0xff00ffff);
    }
}