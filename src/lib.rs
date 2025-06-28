mod vector2;
use vector2::*;

mod rigid_body;
use rigid_body::*;

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

use core::fmt;
use std::ops;
use std::f32::consts::PI;

const GRAVITY: f32 = 1.6;

turbo::init!(
    struct GameState {
        player: Player,
        harvesters: Vec<Harvester>,
        tiles: Vec<Tile>,
        last_time: u64,
    } = {
        let tile_map = TileMap::new(
            &[
                &[1, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 1],
                &[1, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 1],
                &[1, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 1],
                &[1, 1, 1, 1,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 1],
                &[1, 1, 1, 1,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 1, 1],
                &[1, 1, 1, 1,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 1, 1],
                &[1, 1, 1, 1,  1, 1, 0, 0,  1, 1, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 1, 1, 1],
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
            tiles: tile_map.tiles,
            harvesters,
            last_time: 0,
        }
    }
);

turbo::go!({
    let mut state = GameState::load();
    clear(0xadd8e6ff);
    for t in &mut state.tiles {
        t.draw();
    }
    if !audio::is_playing("bg-music-nothing") {
        audio::play("bg-music-nothing");
    }
    state.player.handle_input();
    state.harvesters.iter_mut().for_each(|h| h.rigid_body.apply_gravity());

    state.player.check_collision_tilemap(&state.tiles);
    state.harvesters.iter_mut().for_each(|h| h.rigid_body.check_collision_tilemap(&state.tiles));

    state.player.rigid_body.update_position();
    state.harvesters.iter_mut().for_each(|h| h.rigid_body.update_position());

    center_camera(&state.player.rigid_body.position);
    state.player.draw();
    state.harvesters.iter().for_each(|h| h.draw());
    state.save();
});