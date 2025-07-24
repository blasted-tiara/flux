use crate::*;

#[turbo::serialize]
pub struct Level {
    pub tilemap: TileMap,
    pub harvesters: Vec<Harvester>,
    pub actor_manager: ActorManager,
    pub player_start_position: Vector2,
    pub background: Background,
}