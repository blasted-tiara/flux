use core::fmt;
use std::ops;

const GRAVITY: f32 = 0.6;

const PLAYER_MOVE_SPEED_MAX: f32 = 2.0;
const PLAYER_ACCELERATION: f32 = 1.0;
const PLAYER_DECELERATION: f32 = 0.5;
const PLAYER_MIN_JUMP_FORCE: f32 = 3.0;
const PLAYER_MAX_JUMP_FORCE: f32 = 5.5;
//add these two
const PLAYER_JUMP_POWER_DUR: i32 = 6;
const PLAYER_COYOTE_TIMER_DUR: i32 = 3;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }
    
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl ops::Add for &Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::AddAssign<&Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: &Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct RigidBody {
    position: Vector2,
    velocity: Vector2,
    max_gravity: f32,
    is_falling: bool,
}

impl RigidBody {
    fn update_position(&mut self) {
        prelude::println!("Position: {}", self.position);
        prelude::println!("Velocity: {}", self.velocity);
        self.position += &self.velocity;
    }
    
    fn add_velocity(&mut self, velocity: Vector2) {
        self.velocity += &velocity;
    }
    
    fn clamp_velocity_x(&mut self, max_velocity: Vector2) {
        self.velocity.x = self.velocity.x.clamp(max_velocity.x, max_velocity.y);
    }
    
    fn clamp_velocity_y(&mut self, max_velocity: Vector2) {
        self.velocity.y = self.velocity.y.clamp(max_velocity.x, max_velocity.y);
    }
    
    fn stop_y(&mut self) {
        self.velocity.y = 0.0;
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
struct Harvester {
    
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
struct Player {
    rigid_body: RigidBody,
    is_facing_left: bool,
    coyote_timer: i32,
    is_landed: bool,
    is_powering_jump: bool,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Self {
            rigid_body: RigidBody {
                position: Vector2::new(x, y),
                velocity: Vector2::zero(),
                max_gravity: 15.0,
                is_falling: false,
            },
            is_facing_left: true,
            is_landed: false,
            coyote_timer: 0,
            is_powering_jump: false,
        }
    }
    fn handle_input(&mut self) {
        let gp = gamepad(0);
        if (gp.up.just_pressed() || gp.start.just_pressed())
            && (self.is_landed || self.coyote_timer > 0)
            && self.rigid_body.velocity.y >= 0.
        {
            if !self.is_powering_jump {
                self.rigid_body.velocity.y = -PLAYER_MIN_JUMP_FORCE;
                self.is_powering_jump = true;
                audio::play("jump-sfx-nothing");
            }
        }

        if self.is_powering_jump && (gp.up.pressed() || gp.start.pressed()) && self.rigid_body.velocity.y < 0. {
            self.rigid_body.add_velocity(Vector2::new(0.0, -(PLAYER_MAX_JUMP_FORCE - PLAYER_MIN_JUMP_FORCE) / (PLAYER_JUMP_POWER_DUR as f32)));
            if self.rigid_body.velocity.y <= -PLAYER_MAX_JUMP_FORCE {
                self.is_powering_jump = false;
            }
        } else {
            self.is_powering_jump = false;
        }

        if gp.left.pressed() {
            self.rigid_body.add_velocity(Vector2::new(-PLAYER_ACCELERATION, 0.0));
            self.is_facing_left = true;
        } else if gp.right.pressed() {
            self.rigid_body.add_velocity(Vector2::new(PLAYER_ACCELERATION, 0.0));
            self.is_facing_left = false;
        } else {
            if self.rigid_body.velocity.x > 0. {
                self.rigid_body.add_velocity(Vector2::new(-PLAYER_DECELERATION, 0.0));
            } else if self.rigid_body.velocity.x < 0. {
                self.rigid_body.add_velocity(Vector2::new(PLAYER_DECELERATION, 0.0));
            }
        }

        self.rigid_body.clamp_velocity_x(Vector2::new(-PLAYER_MOVE_SPEED_MAX, PLAYER_MOVE_SPEED_MAX));
        if !self.is_powering_jump {
            self.rigid_body.add_velocity(Vector2::new(0.0, GRAVITY));
        }
        self.rigid_body.clamp_velocity_y(Vector2::new(-PLAYER_MAX_JUMP_FORCE, self.rigid_body.max_gravity));

        if self.coyote_timer > 0 {
            self.coyote_timer -= 1;
        }
    }

    fn check_collision_tilemap(&mut self, tiles: &[Tile]) {
        // Check collision down
        if self.rigid_body.velocity.y > 0.0 {
            if check_collision(&self.rigid_body.position + &Vector2::new(0.0, self.rigid_body.velocity.y), Direction::Down, tiles) {
                self.rigid_body.stop_y();
                self.is_landed = true;
            } else {
                if self.is_landed {
                    self.is_landed = false;
                    self.coyote_timer = PLAYER_COYOTE_TIMER_DUR;
                }
            }
        }

        // Check collision up
        if self.rigid_body.velocity.y < 0.0 {
            while self.rigid_body.velocity.y < 0.0 {
                if check_collision(&self.rigid_body.position + &Vector2::new(0.0, self.rigid_body.velocity.y), Direction::Up, tiles) {
                    self.rigid_body.add_velocity(Vector2::new(0.0, 1.0));
                } else {
                    break;
                }
            }
        }

        // Check collision right
        if self.rigid_body.velocity.x > 0.0 {
            while self.rigid_body.velocity.x > 0.0 {
                if check_collision(&self.rigid_body.position + &Vector2::new(self.rigid_body.velocity.x, 0.0), Direction::Right, tiles) {
                    self.rigid_body.add_velocity(Vector2::new(-1.0, 0.0));
                } else {
                    break;
                }
            }
        }

        // Check collision left
        if self.rigid_body.velocity.x < 0.0 {
            while self.rigid_body.velocity.x < 0.0 {
                if check_collision( &self.rigid_body.position + &Vector2::new(self.rigid_body.velocity.x, 0.0), Direction::Left, tiles) {
                    self.rigid_body.add_velocity(Vector2::new(1.0, 0.0));
                } else {
                    break;
                }
            }
        }
    }

    fn draw(&self) {
        if self.is_landed && self.rigid_body.velocity.x != 0. {
            sprite!(
                "kiwi_walking",
                x = self.rigid_body.position.x as i32,
                y = self.rigid_body.position.y as i32,
                flip_x = self.is_facing_left,
            );
        } else {
            sprite!(
                "kiwi_idle",
                x = self.rigid_body.position.x as i32,
                y = self.rigid_body.position.y as i32,
                flip_x = self.is_facing_left,
            );
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
struct Tile {
    grid_x: usize,
    grid_y: usize,
    tile_size_x: u32,
    tile_size_y: u32,
}

impl Tile {
    fn new(grid_x: usize, grid_y: usize, tile_size_x: u32, tile_size_y: u32) -> Self {
        Self { grid_x, grid_y, tile_size_x, tile_size_y }
    }
    
    fn contains(&self, point_x: f32, point_y: f32) -> bool {
        let tile_x = self.grid_x as f32 * self.tile_size_x as f32;
        let tile_y = self.grid_y as f32 * self.tile_size_y as f32;
        point_x >= tile_x
            && point_x < tile_x + self.tile_size_x as f32
            && point_y >= tile_y
            && point_y < tile_y + self.tile_size_y as f32
    }

    fn draw(&self) {
        let x = self.grid_x as i32 * self.tile_size_x as i32;
        let y = self.grid_y as i32 * self.tile_size_y as i32;

        sprite!("dirt", x = x, y = y);
    }
}

struct TileMap {
    tiles: Vec<Tile>,
}

impl TileMap {
    fn new(data: &[&[u8]], tile_size_x: u32, tile_size_y: u32) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();
        for j in 0..data.len() {
            for i in 0..data[j].len() {
                if data[j][i] == 1 {
                    tiles.push(Tile::new(i, j, tile_size_x, tile_size_y));
                }
            }
        }
        TileMap {
            tiles
        }
    }
}


turbo::init!(
    struct GameState {
        player: Player,
        tiles: Vec<Tile>,
    } = {
        let tile_map = TileMap::new(
            &[
                &[1, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0],
                &[1, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0],
                &[1, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0],
                &[1, 1, 1, 1,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0],
                &[1, 1, 1, 1,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0],
                &[1, 1, 1, 1,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0],
                &[1, 1, 1, 1,  1, 1, 0, 0,  1, 1, 0, 0,  0, 0, 0, 0],
                &[1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 0, 0,  0, 0, 0, 0],
                &[1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1],
            ],
            16,
            16,
        );

        GameState {
            player: Player::new(110., 80.),
            tiles: tile_map.tiles,
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
    state.player.check_collision_tilemap(&state.tiles);
    state.player.rigid_body.update_position();
    center_camera(&state.player.rigid_body.position);
    state.player.draw();
    state.save();
});

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//check collision betwen the player and the tilemap
fn check_collision(
    position: Vector2,
    direction: Direction,
    tiles: &[Tile],
) -> bool {
    //Width and height of sprite art
    let w: f32 = 12.;
    let h: f32 = 12.;
    //Padding between top and left for where sprite art begins
    let pad_x: f32 = 2.;
    let pad_y: f32 = 3.;
    let (check_x1, check_y1, check_x2, check_y2) = match direction {
        Direction::Up => (
            position.x + pad_x,
            position.y + pad_y,
            position.x + pad_x + w,
            position.y + pad_y,
        ),
        Direction::Down => (
            position.x + pad_x,
            position.y + pad_y + h,
            position.x + pad_x + w,
            position.y + pad_y + h,
        ),
        Direction::Left => (
            position.x + pad_x - 1.,
            position.y + pad_y,
            position.x - 1.,
            position.y + pad_y + h,
        ),
        Direction::Right => (
            position.x + pad_x + w + 1.,
            position.y + pad_y,
            position.x + pad_x + w + 1.,
            position.y + pad_y + h,
        ),
    };

    for tile in tiles {
        if tile.contains(check_x1, check_y1) || tile.contains(check_x2, check_y2) {
            return true
        }
    }
    false
}

fn center_camera(center: &Vector2) {
    // Subtract half the width of the canvas, then add half the size of the player to center the camera
    camera::set_xy(center.x + 8., center.y + 8.);
}