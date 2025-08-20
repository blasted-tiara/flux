use crate::*;

#[turbo::serialize]
pub struct Level {
    pub tilemap: TileMap,
    pub harvesters: Vec<Harvester>,
    pub actor_manager: ActorManager,
    pub player1_start_position: Vector2,
    pub player2_start_position: Vector2,
    pub background: Background,
    pub required_flux: f32,
    pub juice_particle_manager: juice_particles::ParticleManager,
}