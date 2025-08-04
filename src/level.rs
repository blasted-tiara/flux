use crate::*;

#[turbo::serialize]
pub struct Level {
    pub tilemap: TileMap,
    pub harvesters: Vec<Harvester>,
    pub actor_manager: ActorManager,
    pub player1_start_position: Vector2,
    pub player2_start_position: Vector2,
    pub background: Background,
}

impl Level {
    pub fn new() -> Self {
        Self {
            tilemap: TileMap::new(&[], 0),
            harvesters: vec!(),
            actor_manager: ActorManager::new(),
            player1_start_position: Vector2::zero(),
            player2_start_position: Vector2::zero(),
            background: Background::new(0xffffffff),
        }
    } 
}