use crate::*;

const GRAVITY: f32 = 1.6;
const MAX_FLUX: f32 = 200.;
const FLUX_LEVELS: u32 = 3;

#[turbo::serialize]
pub struct Harvester {
    pub actor: ActorId,
    velocity: Vector2,
    rotation: f32,
    flux_field: Vector2,
    flux: f32,
}

impl Harvester {
    pub fn new(x: f32, y: f32, rotation: f32, actor_manager: &mut ActorManager) -> Self {
        Self {
            actor: actor_manager.spawn_actor(Actor::new(Vector2::new(x, y), 18., 18.,)),
            velocity: Vector2::zero(),
            rotation: rotation,
            flux_field: Vector2::zero(),
            flux: 0.,
        }        
    }

    pub fn actor_move(&mut self, tiles: &Vec<&Solid>, actor_manager: &mut ActorManager) {
        let actor_option = actor_manager.get_actor_mut(self.actor);
        match actor_option {
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
            },
            None => return,
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
    
    pub fn calculate_flux(&mut self, actor_manager: &mut ActorManager, flux_cores: &Vec<FluxCore>) -> f32 {
        let actor = actor_manager.get_actor(self.actor);
        match actor {
            Some(actor) => {
                let bounding_box = actor.get_bound();
                let (start, end) = get_flux_line(self.rotation, &bounding_box);
                
                self.flux_field = calculate_line_flux(&start, &end, 6, flux_cores);
                self.flux = (end - start).get_normal_vector().normalize().dot(&self.flux_field);
                //(end - start).get_normal_vector().normalize().draw_at_point(&actor.position, self.flux / 10.);
                self.flux
            },
            None => { 0. }
        }
    }
    
    pub fn draw(&self, actor_manager: &ActorManager) {
        let actor_option = actor_manager.get_actor(self.actor);
        match actor_option {
            Some(actor) => {
                let x_offset = 9.;
                let y_offset = 9.;

                let mut sprite = "energybox00";
                match self.flux {
                    50.0..=100. => {
                        sprite = "energybox01"
                    },
                    100.0..=150. => {
                        sprite = "energybox02"
                    },
                    150.0..=200. => {
                        sprite = "energybox03"
                    },
                    200.0..=250. => {
                        sprite = "energybox04"
                    },
                    250.0..=300. => {
                        sprite = "energybox05"
                    },
                    300.0.. => {
                        sprite = "energybox06"
                    },
                    _ => {}
                }

                sprite!(
                    sprite,
                    x = actor.position.x - x_offset,
                    y = actor.position.y - y_offset,
                    rotation = self.rotation.to_degrees(),
                );
            },
            None => return,
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
