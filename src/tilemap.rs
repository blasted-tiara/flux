use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct TileMap {
    pub tiles: Vec<Tile>,
    height: f32,
    width: f32,
}

impl TileMap {
    pub fn new(data: &[&[u8]], tile_size_x: u32, tile_size_y: u32) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();
        let width = data[0].len() as f32 * tile_size_x as f32;
        let height = data.len() as f32 * tile_size_y as f32;

        for j in 0..data.len() {
            for i in 0..data[j].len() {
                if data[j][i] == 1 {
                    tiles.push(Tile::new(i, j, tile_size_x, tile_size_y));
                }
            }
        }
        TileMap {
            tiles,
            width,
            height,
        }
    }
    
    pub fn lock_viewport_to_tilemap(self: &Self, position: &Vector2, viewport_dimensions: &Vector2) -> Vector2 {
        let min_x = viewport_dimensions.x / 2.;
        let max_x = self.width - viewport_dimensions.x / 2.;
        let min_y = -100000000.;
        let max_y = self.height - viewport_dimensions.y / 2.;
        
        Vector2::new(position.x.clamp(min_x, max_x), position.y.clamp(min_y, max_y))
    } 
}

struct Box {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}