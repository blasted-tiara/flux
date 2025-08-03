use turbo::*;
use std::collections::VecDeque;
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

mod mainmenu;
use mainmenu::*;

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
    unprocessed_inputs: VecDeque<UserInput>,
    frames_per_server_update: u32,
    server_player_position: Vector2,
    last_fpsu:u32,
    last_processed_tick:usize,
    camera_center_x: f32,
    camera_center_y: f32, 
    main_menu_options: Vec<MenuOption>,
    game_flow_state: GameFlowState,
}

impl GameState {
    pub fn new() -> Self {
        let level = construct_level1();
        let p1_start = level.player_start_position.clone();
        Self {
            level,
            player1: Player::new(p1_start.x, p1_start.y),
            unprocessed_inputs: VecDeque::new(),
            server_player_position: Vector2::zero(),
            frames_per_server_update: 0,
            last_fpsu: 0,
            last_processed_tick: 0,
            camera_center_x: SCREEN_WIDTH as f32 / 2.,
            camera_center_y: SCREEN_HEIGHT as f32 / 2.,
            main_menu_options: get_main_menu_options(),
            game_flow_state: GameFlowState::MainMenu,
        }
    }
    
    pub fn update(&mut self) {
        self.frames_per_server_update += 1;

        match self.game_flow_state {
            GameFlowState::MainMenu => {
                self.handle_main_menu_flow();
            },
            GameFlowState::InGame => {
                self.handle_in_game_flow();
            },
            GameFlowState::Credits => {}
        }
    }
    
    fn handle_main_menu_flow(&mut self) {
        let selected_option = handle_input(&mut self.main_menu_options);
        match selected_option {
            Some(text) => {
                if text == "Start" {
                    self.game_flow_state = GameFlowState::InGame;
                    return;
                } else if text == "Credits" {
                    self.game_flow_state = GameFlowState::Credits;
                    return;
                }
            },
            None => {},
        }
        draw_main_menu(&mut self.main_menu_options);
    }
    
    fn handle_in_game_flow(&mut self) {
        let gamepad = gamepad::get(0);
        let user_input = UserInput {
            tick: time::tick(),
            jump_just_pressed: gamepad.up.just_pressed() || gamepad.start.just_pressed(),
            jump_pressed: gamepad.up.pressed() || gamepad.start.pressed(),
            left_pressed: gamepad.left.pressed(),
            right_pressed: gamepad.right.pressed(),
            a_just_pressed: gamepad.a.just_pressed(),
        };
        
        if let Some(conn) = FluxGameStateChannel::subscribe("default") { 
            while let Ok(msg) = conn.recv() { 
                match msg {
                    ServerMsg::GameState { harvesters, player1, last_processed_tick } => {
                        self.level.harvesters = harvesters;
                        self.server_player_position = player1.actor.position.clone();
                        self.player1 = player1;
                        
                        while let Some(user_input) = self.unprocessed_inputs.pop_front() {
                            match last_processed_tick {
                                Some(last_tick) => {
                                    self.last_processed_tick = last_tick;
                                    if user_input.tick > last_tick {
                                        self.unprocessed_inputs.push_front(user_input);
                                        break;
                                    }
                                },
                                None => {
                                    self.unprocessed_inputs.push_front(user_input);
                                    break;
                                },
                            }
                        }
                        
                        // Replay unprocessed commands
                        for input in &self.unprocessed_inputs {
                            simulate_frame(&mut self.player1, &mut self.level, input);
                        }
                        
                        self.last_fpsu = self.frames_per_server_update;
                        self.frames_per_server_update = 0;
                    },
                    ServerMsg::StartGame => {},
                    ServerMsg::WaitingForPlayer2 => {},
                }
            }

            // Send gamepad state to the server
            let _ = conn.send( &user_input);
        } 
        
        self.unprocessed_inputs.push_back(user_input.clone());
        simulate_frame(&mut self.player1, &mut self.level, &user_input);

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
        self.server_player_position.draw(5);
        self.level.harvesters.iter().for_each(|h| { h.draw(&mut self.level.actor_manager); /* h.draw_bounding_box(); */ } );
        
        let mut total_flux = 0.;
        for harvester in &mut self.level.harvesters {
            total_flux += harvester.calculate_flux(&mut self.level.actor_manager, &self.level.tilemap.flux_cores);
        }

        let screen_center = Vector2::new(self.camera_center_x as f32, self.camera_center_y as f32);
        show_total_flux(total_flux, &screen_center);
        show_debug_info(self.last_fpsu, &screen_center);
        
        if !audio::is_playing("bg-music-nothing") {
            audio::play("bg-music-nothing");
        }
    }
}

fn show_debug_info(fpsu: u32, screen_center: &Vector2) {
    let mut a = "fpsu: ".to_owned();
    a.push_str(&(fpsu.to_string()));
    text!(
        &a,
        x = screen_center.x + 150.,
        y = screen_center.y - 125., 
        font = "large",
        color = 0x556677ff,
    );
}

#[turbo::serialize]
pub struct UserInput {
    pub tick: usize,
    jump_pressed: bool,
    jump_just_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    a_just_pressed: bool,
}

impl UserInput {
    pub fn new() -> Self {
        Self {
            tick: 0,
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
        last_processed_tick: Option<usize>,
        harvesters: Vec<Harvester>,
        player1: Player,
    }
}

#[turbo::os::channel(program = "testchannel4", name = "main")] 
pub struct FluxGameStateChannel {
    level: Level,
    player1_id: String,
    player1_inputs: VecDeque<UserInput>,
    player1: Player,
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
            level,
            player1_id: String::new(),
            player1: Player::new(player_start_position.x, player_start_position.y),
            player1_inputs: VecDeque::new(),
            player2_id: String::new(),            
            game_started: false,
        }
    } 
    
    fn on_open(&mut self, settings: &mut ChannelSettings) -> Result<(), std::io::Error> {
        settings.set_interval(20);
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
        let mut last_processed_tick: Option<usize> = None;
        // List of all solids in the level
                    
        while !self.player1_inputs.is_empty() {
            let player1_input = self.player1_inputs.pop_front();
            
            match player1_input {
                Some(input) => {
                    last_processed_tick = Some(input.tick);
                    simulate_frame(&mut self.player1, &mut self.level, &input);
                },
                None => {}
            }
        }
        
        return os::server::channel::broadcast(ServerMsg::GameState { harvesters: self.level.harvesters.clone(), player1: self.player1.clone(), last_processed_tick });
    }

    fn on_data(&mut self, user_id: &str, data: Self::Recv) -> Result<(), std::io::Error> { 
        self.player1_inputs.push_back(data);
        Result::Ok(())
    } 
} 

fn simulate_frame(player: &mut Player, level: &mut Level, input: &UserInput) {
    let Level {
        tilemap,
        harvesters,
        actor_manager,
        player_start_position: _,
        background: _,
    } = level;

    let mut solids: Vec<&Solid> = vec![];
    for tile in &tilemap.tiles {
        solids.push(&tile.solid);        
    }
    for flux_core in &tilemap.flux_cores {
        solids.push(&flux_core.solid);
    }
    // NOTE: Cloning because usually a level won't contain more than a couple of doors.
    for door in &tilemap.doors {
        if !door.open {
            solids.push(&door.solid);
        }
    }
        
    player.handle_input(actor_manager, input);

    // Add gravity to 
    for harvester in harvesters.iter_mut() {
        harvester.apply_gravity(&mut level.actor_manager);
    }

    player.pick_item(&mut level.actor_manager);
    // Move player
    player.actor_move(&solids, &mut level.actor_manager);

    // Move harvesters
    level.harvesters.iter_mut().for_each(|h| h.actor_move(&solids, &mut level.actor_manager));

    let mut total_flux = 0.;
    for harvester in &mut level.harvesters {
        total_flux += harvester.calculate_flux(&mut level.actor_manager, &level.tilemap.flux_cores);
    }
    
    for door in &mut level.tilemap.doors {
        if door.id == 0 {
            door.open = total_flux >= FLUX_THRESHOLD;
        }
    }
}