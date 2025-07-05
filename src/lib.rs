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

use core::fmt;
use std::ops;
use std::f32::consts::PI;

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 576;

turbo::init!(
    struct GameState {
        player: Player,
        harvesters: Vec<Harvester>,
        tile_map: TileMap,
        last_time: u64,
    } = {
        let tile_map = TileMap::new(
            &[
                &[1, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 1],
                &[1, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 1],
                &[1, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 1],
                &[1, 1, 1, 1,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 1],
                &[1, 1, 1, 1,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 1, 1],
                &[1, 1, 1, 1,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 1, 1],
                &[1, 1, 1, 1,  1, 1, 0, 0,  1, 1, 0, 0,  1, 1, 1, 1,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 1, 1, 1],
                &[1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 1, 1, 1],
                &[1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1],
            ],
            64,
            64,
        );
        
        let mut harvesters = vec![];
        harvesters.push(Harvester::new(200., 60., 0.0));
        harvesters.push(Harvester::new(300., 40., PI / 2.));
        harvesters.push(Harvester::new(450., 20., PI));
        
        GameState {
            player: Player::new(390., 80.),
            tile_map,
            harvesters,
            last_time: 0,
        }
    }
);

turbo::go!({
    let mut state = GameState::load();
    
    clear(0xadd8e6ff);
    for t in &mut state.tile_map.tiles {
        t.draw();
    }
    if !audio::is_playing("bg-music-nothing") {
        audio::play("bg-music-nothing");
    }
    state.player.handle_input();
    state.harvesters.iter_mut().for_each(|h| h.apply_gravity());

    let mut solids: Vec<&Solid> = vec![];
    for tile in &state.tile_map.tiles {
        solids.push(&tile.solid);        
    }

    state.player.actor_move(&solids);
    state.harvesters.iter_mut().for_each(|h| h.actor_move(&solids));

    center_camera(&state.player.get_position(), &state.tile_map);
    state.player.draw();
    state.player.draw_bounding_box();
    state.harvesters.iter().for_each(|h| { h.draw(); h.draw_bounding_box(); } );
    state.save();
});