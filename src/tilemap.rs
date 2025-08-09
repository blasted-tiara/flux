use std::collections::HashMap;

use crate::*;

use sys::time::tick;

#[turbo::serialize]
pub struct TileMap {
    pub tiles: Vec<Tile>,
    pub flux_cores: Vec<FluxCore>,
    pub doors: Vec<Door>,
    height: f32,
    width: f32,
}

impl TileMap {
    pub fn new(
        terrain_tilemap: &[u8],
        flux_cores_tilemap: &[u8],
        flux_cores_properties: &HashMap<u8, FluxCoreData>,
        doors_tilemap: &[u8],
        width: usize,
        height: usize,
        tile_size: u32) -> Self
    {
        let mut tiles: Vec<Tile> = Vec::new();
        for j in 0..height {
            for i in 0..width {
                let tile_id = terrain_tilemap[j * width + i] as usize;
                if tile_id != 0 {
                    tiles.push(
                        Tile::new(
                            Vector2 {
                                x: (i as f32 + 0.5) * tile_size as f32,
                                y: (j as f32 + 0.5) * tile_size as f32
                            },
                            tile_size as f32,
                            tile_size as f32,
                            tile_id
                        )
                    );
                }
            }
        }
        
        log!("Tiles on level: {}", tiles.len());

        let mut flux_cores: Vec<FluxCore> = Vec::new();
        for j in 0..height {
            for i in 0..width {
                let flux_core_id = flux_cores_tilemap[j * width + i];
                match flux_cores_properties.get(&flux_core_id) {
                    Some(core_data) => {
                        flux_cores.push(FluxCore {
                            amplitude: core_data.amplitude,
                            time_offset: core_data.time_offset,
                            period_s: core_data.time_period,
                            core_type: core_data.core_type.clone(),
                            solid: Solid {
                                position: Vector2 {
                                    x: (i as f32 + 0.5) * tile_size as f32,
                                    y: (j as f32 + 0.5) * tile_size as f32
                                },
                                width: tile_size as f32,
                                height: tile_size as f32,
                            }
                        });
                    },
                    None => {}
                }
            }
        }

        let mut doors: Vec<Door> = Vec::new();
        for j in 0..height {
            for i in 0..width {
                if doors_tilemap[j * width + i] == 1 {
                    doors.push(Door::new(0, (i as f32 + 0.5) * tile_size as f32, (j as f32 + 1.5) * tile_size as f32, tile_size as f32, tile_size as f32 * 3., false));
                }
            }
        }

        TileMap {
            tiles,
            flux_cores,
            doors,
            width: width as f32 * tile_size as f32,
            height: height as f32 * tile_size as f32,
        }
    }
    
    pub fn lock_viewport_to_tilemap(self: &Self, position: &Vector2, viewport_dimensions: &Vector2) -> Vector2 {
        let min_x = viewport_dimensions.x / 2.;
        let max_x = self.width - viewport_dimensions.x / 2.;
        let min_y = viewport_dimensions.y / 2.;
        let max_y = self.height - viewport_dimensions.y / 2.;
        
        Vector2::new(position.x.clamp(min_x, max_x), position.y.clamp(min_y, max_y))
    } 

    pub fn draw_flux_field(&self) {
        for i in (0..self.width as i32).step_by(16) {
            for j in (-self.height as i32..self.height as i32).step_by(16) {
                let point = Vector2::new(i as f32, j as f32);
                let net_flux = net_flux_field_at_point(&point, &self.flux_cores);
                if net_flux.length() > 6.0 {
                    net_flux.draw_at_point(&point, f32::sqrt(0.4 * (tick() % 80) as f32 / 80.));
                }
            }
        }
    }
}