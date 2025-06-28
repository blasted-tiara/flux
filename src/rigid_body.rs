use crate::*;

const GRAVITY: f32 = 1.6;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct RigidBody {
    pub position: Vector2,
    pub rotation: f32,
    pub velocity: Vector2,
}

impl RigidBody {
    pub fn update_position(&mut self) {
        self.position += &self.velocity;
    }
    
    pub fn apply_gravity(&mut self) {
        self.add_velocity(Vector2::new(0.0, GRAVITY));
    }
    
    pub fn add_velocity(&mut self, velocity: Vector2) {
        self.velocity += &velocity;
    }
    
    pub fn clamp_velocity_x(&mut self, max_velocity: Vector2) {
        self.velocity.x = self.velocity.x.clamp(max_velocity.x, max_velocity.y);
    }
    
    pub fn clamp_velocity_y(&mut self, max_velocity: Vector2) {
        self.velocity.y = self.velocity.y.clamp(max_velocity.x, max_velocity.y);
    }
    
    pub fn stop_y(&mut self) {
        self.velocity.y = 0.0;
    }
    
    pub fn rotation_degrees(&self) -> i32 {
        (self.rotation * 360.0 / (2.0 * PI)) as i32
    }

    pub fn check_collision_tilemap(&mut self, tiles: &[Tile]) {
        // Check collision down
        if self.velocity.y > 0.0 {
            if check_collision(&self.position + &Vector2::new(0.0, self.velocity.y), Direction::Down, tiles) {
                self.stop_y();
            }
        }

        // Check collision up
        if self.velocity.y < 0.0 {
            while self.velocity.y < 0.0 {
                if check_collision(&self.position + &Vector2::new(0.0, self.velocity.y), Direction::Up, tiles) {
                    self.add_velocity(Vector2::new(0.0, 1.0));
                } else {
                    break;
                }
            }
        }

        // Check collision right
        if self.velocity.x > 0.0 {
            while self.velocity.x > 0.0 {
                if check_collision(&self.position + &Vector2::new(self.velocity.x, 0.0), Direction::Right, tiles) {
                    self.add_velocity(Vector2::new(-1.0, 0.0));
                } else {
                    break;
                }
            }
        }

        // Check collision left
        if self.velocity.x < 0.0 {
            while self.velocity.x < 0.0 {
                if check_collision( &self.position + &Vector2::new(self.velocity.x, 0.0), Direction::Left, tiles) {
                    self.add_velocity(Vector2::new(1.0, 0.0));
                } else {
                    break;
                }
            }
        }
    }
}

//check collision betwen the player and the tilemap
pub fn check_collision(
    position: Vector2,
    direction: Direction,
    tiles: &[Tile],
) -> bool {
    //Width and height of sprite art
    let w: f32 = 48.;
    let h: f32 = 55.;
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

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}