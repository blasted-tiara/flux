use turbo::*;
use std::io::{Error, ErrorKind};

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
    level: Level,
    player1: Player,
    last_time: u64,
    camera_center_x: f32,
    camera_center_y: f32, 
}

impl GameState {
    pub fn new() -> Self {
        Self {
            last_time: 0,
            camera_center_x: SCREEN_WIDTH as f32 / 2.,
            camera_center_y: SCREEN_HEIGHT as f32 / 2.,
            level: construct_level1(),
            player1: Player::new(0., 0.),
        }
    }
    
    pub fn update(&mut self) {
        let gamepad = gamepad::get(0);
        let user_input = UserInput {
            jump_just_pressed: gamepad.up.just_pressed() || gamepad.start.just_pressed(),
            jump_pressed: gamepad.up.pressed() || gamepad.start.pressed(),
            left_pressed: gamepad.left.pressed(),
            right_pressed: gamepad.right.pressed(),
            a_just_pressed: gamepad.a.just_pressed(),
        };
        
        if let Some(conn) = FluxGameStateChannel::subscribe("default") { 
            while let Ok(msg) = conn.recv() { 
                match msg {
                    ServerMsg::GameState { level, player1 } => {
                        self.level = level;
                        self.player1 = player1;
                    },
                    ServerMsg::StartGame => {},
                    ServerMsg::WaitingForPlayer2 => {},
                }
            }

            // Send gamepad state to the server
            let _ = conn.send( &user_input);
        } 

        let camera_position = self.level.tilemap.lock_viewport_to_tilemap(&Vector2::new(self.player1.actor.position.x, self.player1.actor.position.y), &Vector2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32));
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
        
        self.player1.draw();
        self.level.harvesters.iter().for_each(|h| { h.draw(&mut self.level.actor_manager); /* h.draw_bounding_box(); */ } );
        
        let mut total_flux = 0.;
        for harvester in &mut self.level.harvesters {
            total_flux += harvester.calculate_flux(&mut self.level.actor_manager, &self.level.tilemap.flux_cores);
        }

        show_total_flux(total_flux, Vector2::new(self.camera_center_x as f32, self.camera_center_y as f32));
        
        if !audio::is_playing("bg-music-nothing") {
            audio::play("bg-music-nothing");
        }
    }
}

#[turbo::serialize]
pub struct UserInput {
    jump_pressed: bool,
    jump_just_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    a_just_pressed: bool,
}

impl UserInput {
    pub fn new() -> Self {
        Self {
            jump_just_pressed: false,
            jump_pressed: false,
            left_pressed: false,
            right_pressed: false,
            a_just_pressed: false,
        }
    }
}
 
#[turbo::serialize]
pub enum ServerMsg {
    StartGame, // Used to signal to connected clients that the game is starting
    WaitingForPlayer2, 
    GameState {
        level: Level,
        player1: Player,
    }
}

#[turbo::os::channel(program = "testchannel2", name = "main")] 
pub struct FluxGameStateChannel {
    level: Level,
    player1_id: String,
    player1_pending_input: bool,
    player1: Player,
    player1_input: UserInput,
    player2_id: String,
    game_started: bool,
}

impl ChannelHandler for FluxGameStateChannel { 
    type Recv = UserInput; // incoming from client
    type Send = ServerMsg; // outgoing to client
                             //
    fn new() -> Self { 
        let level = construct_level1();
        let player_start_position = level.player_start_position.clone();
        Self {
            player1_id: String::new(),
            player1: Player::new(player_start_position.x, player_start_position.y),
            player1_pending_input: false,
            player1_input: UserInput::new(),
            player2_id: String::new(),            
            game_started: false,
            level,
        }
    } 
    
    fn on_open(&mut self, settings: &mut ChannelSettings) -> Result<(), std::io::Error> {
        settings.set_interval(32);
        Result::Ok(())
    }
    
    fn on_connect(&mut self, user_id: &str) -> Result<(), std::io::Error> {
        let mut connect_successful = false;
        // If the user is not already registered in the channel, register them
        if self.player1_id.is_empty() && self.player2_id != user_id {
            self.player1_id = user_id.to_string();
            connect_successful = true;
        } else if self.player2_id.is_empty() {
            self.player2_id = user_id.to_string();
            connect_successful = true;
        }

        if connect_successful {
            // Start the game if the max players is reached
            if !self.player1_id.is_empty() && !self.player2_id.is_empty() {
                self.game_started = true;
                return os::server::channel::broadcast(ServerMsg::StartGame);
            }
        
            return os::server::channel::broadcast(ServerMsg::WaitingForPlayer2);
        } else {
            Result::Err(Error::new(ErrorKind::Other ,"Lobby full :("))
        }
    }
    
    fn on_interval(&mut self) -> Result<(), std::io::Error> {
        if self.player1_pending_input {
            self.player1.handle_input(&mut self.level.actor_manager, &self.player1_input);
            self.player1_pending_input = false;
        }

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
        
        self.player1.pick_item(&mut self.level.actor_manager);
        // Move player
        self.player1.actor_move(&solids, &mut self.level.actor_manager);

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

        
        return os::server::channel::broadcast(ServerMsg::GameState { level: self.level.clone(), player1: self.player1.clone() });
    }

    fn on_data(&mut self, user_id: &str, data: Self::Recv) -> Result<(), std::io::Error> { 
        log!("Got {:?} from {:?}", data, user_id); 
        if !self.player1_pending_input {
            self.player1_input = data;
            self.player1_pending_input = true;
        }
        Result::Ok(())
    } 
} 