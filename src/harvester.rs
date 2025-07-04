use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct Harvester {
    pub actor: Actor,
}

impl Harvester {
    pub fn new(x: f32, y: f32, rotation: f32) -> Self {
        Self {
            actor: Actor {
                position: Vector2::new(x, y),
                velocity: Vector2::zero(),
                rotation: rotation,
            }
        }        
    }
    
    pub fn draw(&self) {
        sprite!(
            "harvester",
            x = self.actor.position.x,
            y = self.actor.position.y,
            rotation = self.actor.rotation_degrees(),
        );
    }
}
