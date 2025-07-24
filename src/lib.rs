use turbo::*;

mod vector2;
use vector2::*;

mod actor;
use actor::*;

mod harvester;
use harvester::*;

mod player;
use player::*;

mod tile;
use tile::*;

mod tilemap;
use tilemap::*;

mod bound;
use bound::*;

mod solid;
use solid::*;

mod door;
use door::*;

mod actor_manager;
use actor_manager::*;

mod flux;
use flux::*;

mod level;
use level::*;

mod levels;
use levels::*;

mod background;
use background::*;

use core::fmt;
use std::ops;
use std::f32::consts::PI;

use camera::set_xy;

const SCREEN_WIDTH: i32 = 512;
const SCREEN_HEIGHT: i32 = 288;
const FLUX_THRESHOLD: f32 = 400.;
pub const SPRITE_SCALE: f32 = 0.25;
 
#[turbo::game]
struct GameState {
    player: Player,
    level: Level,
    last_time: u64,
    camera_center_x: f32,
    camera_center_y: f32, 
}

impl GameState {
    pub fn new() -> Self {
        let level = construct_level1();
        let player_start_position = level.player_start_position.clone();
        Self {
            player: Player::new(player_start_position.x, player_start_position.y),
            level,
            last_time: 0,
            camera_center_x: player_start_position.x,
            camera_center_y: player_start_position.y,
        }
    }
    
    pub fn update(&mut self) {
        // Add velocity and forces to player
        self.player.handle_input(&mut self.level.actor_manager);
        // Add gravity to 
        self.level.harvesters.iter_mut().for_each(|h| h.apply_gravity(&mut self.level.actor_manager));

        // List of all solids in the level
        let mut solids: Vec<&Solid> = vec![];
        for tile in &self.level.tilemap.tiles {
            solids.push(&tile.solid);        
        }
        for flux_core in &self.level.tilemap.flux_cores {
            solids.push(&flux_core.solid);
        }
        for door in &self.level.tilemap.doors {
            if !door.open {
                solids.push(&door.solid);
            }
        }
        
        self.player.pick_item(&mut self.level.actor_manager);
        // Move player
        self.player.actor_move(&solids, &mut self.level.actor_manager);

        // Move harvesters
        self.level.harvesters.iter_mut().for_each(|h| h.actor_move(&solids, &mut self.level.actor_manager));

        let mut total_flux = 0.;
        for harvester in &mut self.level.harvesters {
            total_flux += harvester.calculate_flux(&mut self.level.actor_manager, &self.level.tilemap.flux_cores);
        }
        
        for door in &mut self.level.tilemap.doors {
            if door.id == 0 {
                door.open = total_flux >= FLUX_THRESHOLD;
            }
        }

        let camera_position = self.level.tilemap.lock_viewport_to_tilemap(&Vector2::new(self.player.actor.position.x, self.player.actor.position.y), &Vector2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32));
        let new_camera_position_x = lerp(self.camera_center_x as f32, camera_position.x, 0.1);
        let new_camera_position_y = lerp(self.camera_center_y as f32, camera_position.y, 0.1);
        self.level.background.draw(Vector2{ x: new_camera_position_x, y: new_camera_position_y });
        
        self.camera_center_x = new_camera_position_x;
        self.camera_center_y = new_camera_position_y;
        set_xy(self.camera_center_x, self.camera_center_y);

        self.level.tilemap.draw_flux_field();
        for t in &self.level.tilemap.tiles {
            t.draw();
        }
        
        for f in &self.level.tilemap.flux_cores {
            f.draw();
        }
        
        for d in &self.level.tilemap.doors {
            d.draw();
        }
        
        self.player.draw();
        self.level.harvesters.iter().for_each(|h| { h.draw(&mut self.level.actor_manager); /* h.draw_bounding_box(); */ } );
        
        show_total_flux(total_flux, Vector2::new(self.camera_center_x as f32, self.camera_center_y as f32));
        
        if !audio::is_playing("bg-music-nothing") {
            audio::play("bg-music-nothing");
        }
    }
}
