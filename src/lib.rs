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

use core::fmt;
use std::ops;
use std::f32::consts::PI;

use crate::prelude::camera::set_xy;

const SCREEN_WIDTH: i32 = 512;
const SCREEN_HEIGHT: i32 = 288;
const FLUX_THRESHOLD: f32 = 400.;
pub const SPRITE_SCALE: f32 = 0.25;
 
turbo::init!(
    struct GameState {
        player: Player,
        level: Level,
        last_time: u64,
        camera_center_x: f32,
        camera_center_y: f32, 
    } = {
        let level = construct_level1();
        let player_start_position = level.player_start_position.clone();
        GameState {
            player: Player::new(player_start_position.x, player_start_position.y),
            level,
            last_time: 0,
            camera_center_x: player_start_position.x,
            camera_center_y: player_start_position.y,
        }
    }
);

turbo::go!({
    let mut state = GameState::load();
    
    // Add velocity and forces to player
    state.player.handle_input(&mut state.level.actor_manager);
    // Add gravity to 
    state.level.harvesters.iter_mut().for_each(|h| h.apply_gravity(&mut state.level.actor_manager));

    // List of all solids in the level
    let mut solids: Vec<&Solid> = vec![];
    for tile in &state.level.tilemap.tiles {
        solids.push(&tile.solid);        
    }
    for flux_core in &state.level.tilemap.flux_cores {
        solids.push(&flux_core.solid);
    }
    for door in &state.level.tilemap.doors {
        if !door.open {
            solids.push(&door.solid);
        }
    }
    
    state.player.pick_item(&mut state.level.actor_manager);
    // Move player
    state.player.actor_move(&solids, &mut state.level.actor_manager);

    // Move harvesters
    state.level.harvesters.iter_mut().for_each(|h| h.actor_move(&solids, &mut state.level.actor_manager));

    let mut total_flux = 0.;
    for harvester in &mut state.level.harvesters {
        total_flux += harvester.calculate_flux(&mut state.level.actor_manager, &state.level.tilemap.flux_cores);
    }
    
    for door in &mut state.level.tilemap.doors {
        if door.id == 0 {
            door.open = total_flux >= FLUX_THRESHOLD;
        }
    }

    let camera_position = state.level.tilemap.lock_viewport_to_tilemap(&Vector2::new(state.player.actor.position.x, state.player.actor.position.y), &Vector2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32));
    let new_camera_position_x = lerp(state.camera_center_x as f32, camera_position.x, 0.1);
    let new_camera_position_y = lerp(state.camera_center_y as f32, camera_position.y, 0.1);
    
    state.camera_center_x = new_camera_position_x;
    state.camera_center_y = new_camera_position_y;
    set_xy(state.camera_center_x, state.camera_center_y);

    state.level.tilemap.draw_flux_field();
    for t in &state.level.tilemap.tiles {
        t.draw();
    }
    
    for f in &state.level.tilemap.flux_cores {
        f.draw();
    }
    
    for d in &state.level.tilemap.doors {
        d.draw();
    }
    
    state.player.draw();
    state.level.harvesters.iter().for_each(|h| { h.draw(&mut state.level.actor_manager); /* h.draw_bounding_box(); */ } );
    
    show_total_flux(total_flux, Vector2::new(state.camera_center_x as f32, state.camera_center_y as f32));
    
    if !audio::is_playing("bg-music-nothing") {
        audio::play("bg-music-nothing");
    }

    //state.player.draw_bounding_box();
    state.save();
});