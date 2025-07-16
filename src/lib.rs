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
        harvesters: Vec<Harvester>,
        tile_map: TileMap,
        last_time: u64,
        actor_manager: ActorManager,
        camera_center_x: u32,
        camera_center_y: u32, 
    } = {
        let tile_map = TileMap::new(
            &[
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 3, 3, 2, 2, 3, 3, 2, 2, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1,],
                &[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
                &[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
                &[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
                &[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
                &[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
                &[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 1, 3, 1, 2, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,],
                &[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,],
                &[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1,],
                &[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11,0,],
                &[1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 00,0,],
                &[1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 00,0,],
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,],
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,],
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,],
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,],
            ],
            16,
            16,
        );
        
        let mut actor_manager = ActorManager::new();
    
        let mut harvesters = vec![];
        harvesters.push(Harvester::new(150., 60., 0.0, &mut actor_manager));
        harvesters.push(Harvester::new(300., 60., PI / 2., &mut actor_manager));
        harvesters.push(Harvester::new(450., 60., PI, &mut actor_manager));
        
        let player_x = 390.;
        let player_y = -50.;
        GameState {
            player: Player::new(player_x, player_y),
            tile_map,
            harvesters,
            last_time: 0,
            actor_manager,
            camera_center_x: 0,
            camera_center_y: 0,
        }
    }
);

turbo::go!({
    let mut state = GameState::load();
    
    // Add velocity and forces to player
    state.player.handle_input(&mut state.actor_manager);
    // Add gravity to 
    state.harvesters.iter_mut().for_each(|h| h.apply_gravity(&mut state.actor_manager));

    // List of all solids in the level
    let mut solids: Vec<&Solid> = vec![];
    for tile in &state.tile_map.tiles {
        solids.push(&tile.solid);        
    }
    for flux_core in &state.tile_map.flux_cores {
        solids.push(&flux_core.solid);
    }
    for door in &state.tile_map.doors {
        if !door.open {
            solids.push(&door.solid);
        }
    }
    
    state.player.pick_item(&mut state.actor_manager);
    // Move player
    state.player.actor_move(&solids, &mut state.actor_manager);

    // Move harvesters
    state.harvesters.iter_mut().for_each(|h| h.actor_move(&solids, &mut state.actor_manager));

    let mut total_flux = 0.;
    for harvester in &mut state.harvesters {
        total_flux += harvester.calculate_flux(&mut state.actor_manager, &state.tile_map.flux_cores);
    }
    
    for door in &mut state.tile_map.doors {
        if door.id == 0 {
            door.open = total_flux >= FLUX_THRESHOLD;
        }
    }

    let camera_position = state.tile_map.lock_viewport_to_tilemap(&Vector2::new(state.player.actor.position.x, state.player.actor.position.y), &Vector2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32));
    let new_camera_position_x = lerp(state.camera_center_x as f32, camera_position.x, 0.2);
    let new_camera_position_y = lerp(state.camera_center_y as f32, camera_position.y, 0.2);
    
    state.camera_center_x = new_camera_position_x as u32;
    state.camera_center_y = new_camera_position_y as u32;
    set_xy(state.camera_center_x, state.camera_center_y);

    clear(0xadd8e6ff);
    state.tile_map.draw_flux_field();
    for t in &state.tile_map.tiles {
        t.draw();
    }
    
    for f in &state.tile_map.flux_cores {
        f.draw();
    }
    
    for d in &state.tile_map.doors {
        d.draw();
    }
    
    state.player.draw();
    state.harvesters.iter().for_each(|h| { h.draw(&mut state.actor_manager); /* h.draw_bounding_box(); */ } );
    
    show_total_flux(total_flux, Vector2::new(state.camera_center_x as f32, state.camera_center_y as f32));
    
    if !audio::is_playing("bg-music-nothing") {
        audio::play("bg-music-nothing");
    }

    //state.player.draw_bounding_box();
    state.save();
});