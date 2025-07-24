use std::collections::HashMap;

use crate::*;

pub type ActorId = u32;

#[turbo::serialize]
pub struct ActorManager {
    next_id: ActorId,
    pub actors: HashMap<ActorId, Actor>,
}

impl ActorManager {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            actors: HashMap::new(),
        }
    } 
    
    pub fn spawn_actor(&mut self, actor: Actor) -> ActorId {
        let id = self.next_id;
        self.next_id += 1;
        
        self.actors.insert(id, actor);

        id
    }
    
    pub fn get_actor_mut(&mut self, id: ActorId) -> Option<&mut Actor> {
        self.actors.get_mut(&id)
    }
    
    pub fn get_actor(&self, id: ActorId) -> Option<&Actor> {
        self.actors.get(&id)
    }
}
