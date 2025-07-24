use crate::*;

#[turbo::serialize]
pub struct Door {
    pub id: u32,
    pub open: bool,
    pub solid: Solid
}

impl Door {
    pub fn new(door_id: u32, position_x: f32, position_y: f32, width: f32, height: f32, open: bool) -> Self {
        Self {
            id: door_id,
            open,
            solid: Solid {
                position: Vector2 { x: position_x, y: position_y },
                width,
                height,
            }
        } 
    }
    
    pub fn draw(&self) {
        let x = (self.solid.position.x - self.solid.width / 2.) as i32;
        let y = (self.solid.position.y - self.solid.height / 2.) as i32;

        if self.open {
            sprite!("door_open", x = x, y = y);
        } else {
            sprite!("door_closed", x = x, y = y);
        }
    }

    pub fn draw_bounding_box(&self) {
        self.solid.get_bound().draw_bounding_box();
    }
    
}