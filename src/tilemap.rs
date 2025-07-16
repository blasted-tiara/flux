use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct TileMap {
    pub tiles: Vec<Tile>,
    pub flux_cores: Vec<Flux>,
    pub doors: Vec<Door>,
    height: f32,
    width: f32,
}

impl TileMap {
    pub fn new(data: &[&[u8]], tile_size_x: u32, tile_size_y: u32) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();
        let mut flux_cores: Vec<Flux> = Vec::new();
        let mut doors: Vec<Door> = Vec::new();

        let width = data[0].len() as f32 * tile_size_x as f32;
        let height = data.len() as f32 * tile_size_y as f32;

        for j in 0..data.len() {
            for i in 0..data[j].len() {
                let data_value = data[j][i];
                if data_value == 1 {
                    tiles.push(Tile::new(Vector2 { x: (i as f32 + 0.5) * tile_size_x as f32, y: (j as f32 + 0.5) * tile_size_x as f32 }, tile_size_x as f32, tile_size_y as f32));
                } else if data_value == 11 {
                    doors.push(Door::new(0, (i as f32 + 0.5) * tile_size_x as f32, (j as f32 + 1.5) * tile_size_y as f32, tile_size_x as f32, tile_size_y as f32 * 3., false));
                } else if data_value == 2 {
                    flux_cores.push(Flux::new(2000., Vector2 { x: (i as f32 + 0.5) * tile_size_x as f32, y: (j as f32 + 0.5) * tile_size_x as f32 }, tile_size_x as f32, tile_size_y as f32));
                } else if data_value == 3 {
                    flux_cores.push(Flux::new(-2000. , Vector2 { x: (i as f32 + 0.5) * tile_size_x as f32, y: (j as f32 + 0.5) * tile_size_x as f32 }, tile_size_x as f32, tile_size_y as f32));
                }
            }
        }

        TileMap {
            tiles,
            flux_cores,
            doors,
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

    pub fn draw_flux_field(&self) {
        for i in (0..self.width as i32).step_by(16) {
            for j in (0..self.height as i32).step_by(16) {
                let point = Vector2::new(i as f32, j as f32);
                let net_flux = net_flux_field_at_point(&point, &self.flux_cores);
                net_flux.draw_at_point(&point, f32::sqrt(0.4 * (tick() % 80) as f32 / 80.));
            }
        }
    }
}