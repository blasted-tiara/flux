use turbo::canvas::camera::y;

use crate::*;

const GRAVITY: f32 = 1.6;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct Harvester {
    pub actor: Actor,
    velocity: Vector2,
    rotation: f32,
}

impl Harvester {
    pub fn new(x: f32, y: f32, rotation: f32) -> Self {
        Self {
            actor: Actor::new(Vector2::new(x, y), 64., 64.,),
            velocity: Vector2::zero(),
            rotation: rotation,
        }        
    }

    pub fn actor_move(&mut self, tiles: &Vec<&Solid>) {
        let current_velocity_x = self.velocity.x;
        let current_velocity_y = self.velocity.y;

        let on_x_collision = || {
            self.velocity.x = 0.;
        };
        self.actor.move_x(&tiles, current_velocity_x, on_x_collision);

        let on_y_collision = |collision_happened: bool| {
            if collision_happened {
                self.velocity.y = 0.;
            }
        };
        self.actor.move_y(tiles, current_velocity_y, on_y_collision);
    }
    
    pub fn apply_gravity(self: &mut Self) {
        self.velocity += &Vector2::new(0.0, GRAVITY);
    }
    
    pub fn draw(&self) {
        let x_ofsset = 32.;
        let y_ofsset = 32.;

        sprite!(
            "harvester",
            x = self.actor.position.x - x_ofsset,
            y = self.actor.position.y - y_ofsset,
            rotation = self.rotation.to_degrees(),
        );
    }
    
    pub fn draw_bounding_box(&self) {
        self.actor.get_bound().draw_bounding_box();
    }
}
