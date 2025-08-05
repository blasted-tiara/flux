use turbo::audio::play;
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
    local_player: Player,
    server_player_position: Vector2,
    unprocessed_local_inputs: VecDeque<UserInput>,
    remote_player_snapshots: VecDeque<Player>,
    max_player_snapshots: usize,
    frames_per_server_update: u32,
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
        let p1_start = level.player1_start_position.clone();
        Self {
            level,
            local_player: Player::new(p1_start.x, p1_start.y),
            server_player_position: Vector2::zero(),
            unprocessed_local_inputs: VecDeque::new(),
            remote_player_snapshots: VecDeque::new(),
            max_player_snapshots: 5,
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
            GameFlowState::WaitingForPlayer2 => {
                self.handle_waiting_for_player_2_flow();
            },
            GameFlowState::Credits => {}
        }
    }
    
    fn handle_waiting_for_player_2_flow(&mut self) {
        if let Some(conn) = FluxGameStateChannel::subscribe("default") { 
            while let Ok(msg) = conn.recv() { 
                match msg {
                    ServerMsg::ConnectionSuccessful { player_id } => {
                        log!("Connection successful, your id: {}", player_id);
                        if self.local_player.id.is_empty() {
                            self.local_player.id = player_id;
                        }
                    },
                    ServerMsg::StartGame  => {
                        self.game_flow_state = GameFlowState::InGame;
                    },
                    _ => {},
                }
            }
            let _ = conn.send(&ClientMsg::Ready );
        }
        clear(0x00000000);
        text!(
            "Waiting for player 2...",
            x = SCREEN_WIDTH / 2 - 30,
            y = SCREEN_HEIGHT / 2,
            color = 0x00ffffff,
            font = "large",
        );
    }
    
    fn handle_main_menu_flow(&mut self) {
        clear(0x00000000);
        let selected_option = handle_input(&mut self.main_menu_options);
        match selected_option {
            Some(text) => {
                if text == "Start" {
                    self.game_flow_state = GameFlowState::WaitingForPlayer2;
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
                    ServerMsg::GameState { harvesters, player1, last_processed_tick_p1, player2, last_processed_tick_p2 } => {
                        self.level.harvesters = harvesters;
                        if self.local_player.id == player1.id  {
                            self.server_player_position = player1.actor.position.clone();
                            self.local_player = player1;

                            match last_processed_tick_p1 {
                                Some(last_tick) => {
                                    while let Some(user_input) = self.unprocessed_local_inputs.pop_front() {
                                        self.last_processed_tick = last_tick;
                                        if user_input.tick > last_tick {
                                            self.unprocessed_local_inputs.push_front(user_input);
                                            break;
                                        }
                                    }
                                },
                                None => {},
                            }

                            self.remote_player_snapshots.push_front(player2);
                        } else if self.local_player.id == player2.id {
                            self.server_player_position = player2.actor.position.clone();
                            self.local_player = player2;

                            match last_processed_tick_p2 {
                                Some(last_tick) => {
                                    while let Some(user_input) = self.unprocessed_local_inputs.pop_front() {
                                        self.last_processed_tick = last_tick;
                                        if user_input.tick > last_tick {
                                            self.unprocessed_local_inputs.push_front(user_input);
                                            break;
                                        }
                                    }
                                },
                                None => {},
                        }

                            self.remote_player_snapshots.push_front(player1);
                        }
                        
                        if self.remote_player_snapshots.len() > self.max_player_snapshots {
                            self.remote_player_snapshots.pop_back();
                        }
                        
                        // Replay unprocessed commands
                        for input in &self.unprocessed_local_inputs {
                            simulate_frame(&mut self.local_player, &mut self.level, input);
                        }
                        
                        self.last_fpsu = self.frames_per_server_update;
                        self.frames_per_server_update = 0;
                    },
                    _ => {}
                }
            }

            // Send gamepad state to the server
            let _ = conn.send(&ClientMsg::UserInput { user_input: user_input.clone() });
        } 
        
        self.unprocessed_local_inputs.push_back(user_input.clone());
        simulate_frame(&mut self.local_player, &mut self.level, &user_input);

        let camera_position = self.level.tilemap.lock_viewport_to_tilemap(&Vector2::new(self.local_player.actor.position.x, self.local_player.actor.position.y), &Vector2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32));
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
        
        self.local_player.draw();
        let interpolated_remote_player = self.remote_player_snapshots.back().cloned();
        let mut snapshot_positions: Vec<Vector2> = vec![];
        for remote_player in &self.remote_player_snapshots {
            snapshot_positions.push(remote_player.actor.position.clone());
        }
        match interpolated_remote_player {
            Some(mut player) => {
                player.actor.position = smooth_position(snapshot_positions);
                player.draw();
            },
            None => {}
        }
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

fn smooth_position(position_snapshots: Vec<Vector2>) -> Vector2 {
    let n = position_snapshots.len();
    let mut sum = Vector2::zero();
    for snapshot in position_snapshots {
        sum = sum + snapshot; 
    }
    
    sum * (1. / n as f32)
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
    tick: usize,
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
pub enum ClientMsg {
    UserInput {
        user_input: UserInput,
    },
    Ready
}

#[turbo::serialize]
pub enum ServerMsg {
    // Used to signal to connected clients that the game is starting
    StartGame, 
    // Signal player that connection went well
    ConnectionSuccessful {
        player_id: String,
    }, 
    GameState {
        player1: Player,
        last_processed_tick_p1: Option<usize>,
        player2: Player,
        last_processed_tick_p2: Option<usize>,
        harvesters: Vec<Harvester>,
    }
}

#[turbo::os::channel(program = "testchannel5", name = "main")] 
pub struct FluxGameStateChannel {
    level: Level,
    player1: Player,
    player1_inputs: VecDeque<UserInput>,
    player1_ready: bool,
    player2: Player,
    player2_inputs: VecDeque<UserInput>,
    player2_ready: bool,
    game_started: bool,
}

impl ChannelHandler for FluxGameStateChannel { 
    type Recv = ClientMsg; // incoming from client
    type Send = ServerMsg; // outgoing to client
                             //
    fn new() -> Self { 
        let level = construct_level1();
        let player1_start_position = level.player1_start_position.clone();
        let player2_start_position = level.player2_start_position.clone();
        Self {
            level,
            player1: Player::new(player1_start_position.x, player1_start_position.y),
            player1_inputs: VecDeque::new(),
            player1_ready: false,
            player2: Player::new(player2_start_position.x, player2_start_position.y),
            player2_inputs: VecDeque::new(),
            player2_ready: false,
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
        if self.player1.id.is_empty() && self.player2.id != user_id {
            self.player1.id = user_id.to_string();
            connect_successful = true;
        } else if self.player2.id.is_empty() {
            self.player2.id = user_id.to_string();
            connect_successful = true;
        }

        if connect_successful {
            return os::server::channel::broadcast(ServerMsg::ConnectionSuccessful  { player_id: user_id.to_string() });
        } else {
            Result::Err(Error::new(ErrorKind::Other ,"Lobby full :("))
        }
    }
    
    fn on_interval(&mut self) -> Result<(), std::io::Error> {
        if !self.game_started {
            return Result::Ok(());
        }
        let mut last_processed_tick_p1: Option<usize> = None;
        
        while !self.player1_inputs.is_empty() {
            let player1_input = self.player1_inputs.pop_front();
            
            match player1_input {
                Some(input) => {
                    last_processed_tick_p1 = Some(input.tick);
                    simulate_frame(&mut self.player1, &mut self.level, &input);
                },
                None => {}
            }
        }

        let mut last_processed_tick_p2: Option<usize> = None;

        // NOTE: Extract this into a function
        while !self.player2_inputs.is_empty() {
            let player2_input = self.player2_inputs.pop_front();
            
            match player2_input {
                Some(input) => {
                    last_processed_tick_p2 = Some(input.tick);
                    simulate_frame(&mut self.player2, &mut self.level, &input);
                },
                None => {}
            }
        }

        return os::server::channel::broadcast(ServerMsg::GameState { harvesters: self.level.harvesters.clone(), player1: self.player1.clone(), last_processed_tick_p1, player2: self.player2.clone(), last_processed_tick_p2 });
    }

    fn on_data(&mut self, user_id: &str, data: Self::Recv) -> Result<(), std::io::Error> { 
        log!("Data rec!");
        match data {
            ClientMsg::UserInput { user_input } => {
                if user_id == self.player1.id {
                    self.player1_inputs.push_back(user_input);
                } else if user_id == self.player2.id {
                    self.player2_inputs.push_back(user_input);
                }
            },
            ClientMsg::Ready => {
                if user_id == self.player1.id {
                    self.player1_ready = true;                    
                } else if user_id == self.player2.id {
                    self.player2_ready = true;                    
                }
                
                if self.player1_ready && self.player2_ready {
                    self.game_started = true;
                    return os::server::channel::broadcast(ServerMsg::StartGame );
                }
            }
        }
        Result::Ok(())
    } 
} 

fn simulate_frame(player: &mut Player, level: &mut Level, input: &UserInput) {
    let Level {
        tilemap,
        harvesters,
        actor_manager,
        player1_start_position: _,
        player2_start_position: _,
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