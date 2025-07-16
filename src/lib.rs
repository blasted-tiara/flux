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

mod camera;
use camera::*;

mod bound;
use bound::*;

mod solid;
use solid::*;

mod actor_manager;
use actor_manager::*;

mod flux;
use flux::*;

use core::fmt;
use std::ops;
use std::f32::consts::PI;

const SCREEN_WIDTH: i32 = 512;
const SCREEN_HEIGHT: i32 = 288;
pub const SPRITE_SCALE: f32 = 0.25;
 
turbo::init!(
    struct GameState {
        player: Player,
        harvesters: Vec<Harvester>,
        tile_map: TileMap,
        last_time: u64,
        actor_manager: ActorManager,
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
                &[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1,],
                &[1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1,],
                &[1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1,],
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
        
        GameState {
            player: Player::new(390., -50.),
            tile_map,
            harvesters,
            last_time: 0,
            actor_manager,
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
    for flux in &state.tile_map.flux_cores {
        solids.push(&flux.solid);
    }
    
    state.player.pick_item(&mut state.actor_manager);
    // Move player
    state.player.actor_move(&solids, &mut state.actor_manager);

    //for harvester in &state.harvesters {
    //    harvester.actor_move(&solids, &actors);
    //}
    state.harvesters.iter_mut().for_each(|h| h.actor_move(&solids, &mut state.actor_manager));
    
    clear(0xadd8e6ff);
    state.tile_map.draw_flux_field();
    center_camera(&state.player.get_position(), &state.tile_map);
    for t in &state.tile_map.tiles {
        t.draw();
    }
    
    for f in &state.tile_map.flux_cores {
        f.draw();
    }
    
    state.player.draw();
    state.harvesters.iter().for_each(|h| { h.draw(&mut state.actor_manager); /* h.draw_bounding_box(); */ } );
    let mut total_flux = 0.;
    for harvester in &mut state.harvesters {
        total_flux += harvester.calculate_flux(&mut state.actor_manager, &state.tile_map.flux_cores);
    }
    
    show_total_flux(total_flux, state.tile_map.lock_viewport_to_tilemap(&Vector2::new( state.player.get_position().x + 8., state.player.get_position().y + 8.), &Vector2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)));
    
    if !audio::is_playing("bg-music-nothing") {
        audio::play("bg-music-nothing");
    }

    //state.player.draw_bounding_box();
    state.save();
});