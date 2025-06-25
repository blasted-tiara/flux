use crate::*;

pub struct TileMap {
    pub tiles: Vec<Tile>,
}

impl TileMap {
    pub fn new(data: &[&[u8]], tile_size_x: u32, tile_size_y: u32) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();
        for j in 0..data.len() {
            for i in 0..data[j].len() {
                if data[j][i] == 1 {
                    tiles.push(Tile::new(i, j, tile_size_x, tile_size_y));
                }
            }
        }
        TileMap {
            tiles
        }
    }
}
