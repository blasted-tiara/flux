use crate::*;

#[turbo::serialize]
pub struct Tile {
    pub sprite_id: usize,
    pub solid: Solid,
}

impl Tile {
    pub fn new(position: Vector2, width: f32, height: f32, sprite_id: usize) -> Self {
        Self {
            sprite_id,
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

        sprite!(TERRAIN[self.sprite_id], x = x, y = y);
    }
}

const TERRAIN: [&str; 34] = [
    "",
    "tile_1",
    "tile_2",
    "tile_3",
    "tile_4",
    "tile_5",
    "tile_6",
    "tile_7",
    "tile_8",
    "tile_9",
    "tile_10",
    "tile_11",
    "tile_12",
    "tile_13",
    "tile_14",
    "tile_15",
    "tile_16",
    "tile_17",
    "tile_18",
    "tile_19",
    "tile_20",
    "tile_21",
    "tile_22",
    "tile_23",
    "tile_24",
    "tile_25",
    "tile_26",
    "tile_27",
    "tile_28",
    "tile_29",
    "tile_30",
    "tile_31",
    "tile_32",
    "tile_33",
];