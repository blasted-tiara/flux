use std::collections::HashMap;

use crate::*;

pub struct FluxCoreData {
    pub amplitude: f32,
    pub core_type: FluxCoreType,
    pub time_offset: f32,
    pub time_period: f32,
}

#[turbo::serialize]
pub struct LevelManager {
    pub loaded_level: Level,
    pub current_level: Option<LevelName>
}

impl LevelManager {
    pub fn new() -> Self {
        Self {
            current_level: Some(LevelName::Level1),
            loaded_level: construct_level_1(),
        }
    }

    pub fn load_next_level(&mut self) {
        match &self.current_level {
            Some(level_name) => {
                self.current_level = Self::get_next_level(level_name.clone());
                match &self.current_level {
                    Some(new_level_name) => {
                        self.loaded_level = Self::construct_level(new_level_name.clone());
                    },
                    None => { }
                }
            },
            None => { }
        }
    }

    fn get_next_level(level_name: LevelName) -> Option<LevelName> {
        match level_name {
            LevelName::Level1 => Some(LevelName::Level2),
            LevelName::Level2 => None,
        }        
    }

    fn construct_level(level_name: LevelName) -> Level {
        match level_name {
            LevelName::Level1 => construct_level_1(),
            LevelName::Level2 => construct_level_2(),
        }
    }
}

#[turbo::serialize]
pub enum LevelName {
    Level1,
    Level2,
}