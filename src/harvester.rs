use turbo::canvas::camera::y;

use crate::*;

const GRAVITY: f32 = 1.6;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct Harvester {
    pub actor: ActorId,
    velocity: Vector2,
    rotation: f32,
}

impl Harvester {
    pub fn new(x: f32, y: f32, rotation: f32, actor_manager: &mut ActorManager) -> Self {
        Self {
            actor: actor_manager.spawn_actor(Actor::new(Vector2::new(x, y), 36., 36.,)),
            velocity: Vector2::zero(),
            rotation: rotation,
        }        
    }

    pub fn actor_move(&mut self, tiles: &Vec<&Solid>, actor_manager: &mut ActorManager) {
        let actor_option = actor_manager.get_actor_mut(self.actor);
        match actor_option {
            None => return,
            Some(actor) => {
                if actor.is_child { return };
                let current_velocity_x = self.velocity.x;
                let current_velocity_y = self.velocity.y;

                let on_x_collision = || {
                    self.velocity.x = 0.;
                };

                actor.move_x(&tiles, current_velocity_x, on_x_collision);

                let on_y_collision = |collision_happened: bool| {
                    if collision_happened {
                        self.velocity.y = 0.;
                    }
                };
                actor.move_y(tiles, current_velocity_y, on_y_collision);
            }
        }
    }
    
    pub fn apply_gravity(self: &mut Self, actor_manager: &mut ActorManager) {
        let actor_option = actor_manager.get_actor(self.actor);
        match actor_option {
            None => {},
            Some(actor) => {
                 if !actor.is_child {
                    self.velocity += &Vector2::new(0.0, GRAVITY);
                 }
            }
        }
    }
    
    pub fn draw(&self, actor_manager: &ActorManager) {
        let actor_option = actor_manager.get_actor(self.actor);
        match actor_option {
            None => return,
            Some(actor) => {
                let x_ofsset = 18.;
                let y_ofsset = 18.;

                sprite!(
                    "harvester",
                    x = actor.position.x - x_ofsset,
                    y = actor.position.y - y_ofsset,
                    rotation = self.rotation.to_degrees(),
                );
            }
        }
    }
    
    pub fn draw_bounding_box(&self, actor_manager: &ActorManager) {
        let actor_option = actor_manager.get_actor(self.actor);
        match actor_option {
            None => return,
            Some(actor) => {
                actor.get_bound().draw_bounding_box();
            }
        }
    }
}
