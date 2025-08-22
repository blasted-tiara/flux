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
        let level_name = LevelName::Level1;
        Self {
            loaded_level: Self::construct_level(&level_name),
            current_level: Some(level_name),
        }
    }

    pub fn load_next_level(&mut self) {
        match &self.current_level {
            Some(level_name) => {
                self.current_level = Self::get_next_level(level_name.clone());
                match &self.current_level {
                    Some(new_level_name) => {
                        self.loaded_level = Self::construct_level(&new_level_name);
                    },
                    None => { }
                }
            },
            None => { }
        }
    }
    
    pub fn reload_current_level(&mut self) {
        match &self.current_level {
            Some(level_name) => {
                self.loaded_level = Self::construct_level(level_name);
            },
            None => {},
        }
    }

    fn get_next_level(level_name: LevelName) -> Option<LevelName> {
        match level_name {
            LevelName::Level1 => Some(LevelName::Level2),
            LevelName::Level2 => Some(LevelName::Level3),
            LevelName::Level3 => Some(LevelName::Level4),
            LevelName::Level4 => None,
        }        
    }

    fn construct_level(level_name: &LevelName) -> Level {
        match level_name {
            LevelName::Level1 => construct_level_1(),
            LevelName::Level2 => construct_level_2(),
            LevelName::Level3 => construct_level_3(),
            LevelName::Level4 => construct_level_4(),
        }
    }
}

#[turbo::serialize]
pub enum LevelName {
    Level1,
    Level2,
    Level3,
    Level4,
}