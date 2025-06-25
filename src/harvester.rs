use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct Harvester {
    pub rigid_body: RigidBody,
}

impl Harvester {
    pub fn new(x: f32, y: f32, rotation: f32) -> Self {
        Self {
            rigid_body: RigidBody {
                position: Vector2::new(x, y),
                velocity: Vector2::zero(),
                rotation: rotation,
                max_gravity: 15.0,
                is_falling: false,
                is_landed: false,
                coyote_timer: 0,
            }
        }        
    }
    
    pub fn draw(&self) {
        sprite!(
            "harvester",
            x = self.rigid_body.position.x,
            y = self.rigid_body.position.y,
            rotation = self.rigid_body.rotation_degrees(),
        );
    }
}
