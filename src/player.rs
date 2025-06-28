use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct Player {
    pub rigid_body: RigidBody,
    is_facing_left: bool,
    coyote_timer: i32,
    jump_buffer_timer: i32,
    gravity: f32,
    max_gravity: f32,
    move_speed_max: f32,
    acceleration: f32,
    deceleration: f32,
    jump_force: f32,
    coyote_timer_duration: i32,
    jump_buffer_timer_duration: i32,
    movement_status: MovementStatus,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            rigid_body: RigidBody {
                position: Vector2::new(x, y),
                velocity: Vector2::zero(),
                rotation: 0.0,
            },
            move_speed_max: 12.0,
            coyote_timer: 0,
            jump_buffer_timer: 0,
            gravity: 5.,
            max_gravity: 25.,
            is_facing_left: true,
            acceleration: 6.,
            deceleration: 2.0,
            jump_force: 50.,
            coyote_timer_duration: 3,
            jump_buffer_timer_duration: 8,
            movement_status: MovementStatus::IsFalling,
        }
    }
    
    pub fn handle_input(&mut self) {
        let gp = gamepad(0);
        
        let jump_just_pressed = gp.up.just_pressed() || gp.start.just_pressed();
        let jump_pressed = gp.up.pressed() || gp.start.pressed();
        
        match self.movement_status {
            MovementStatus::IsLanded => {
                if jump_just_pressed || self.jump_buffer_timer > 0 {
                    self.rigid_body.velocity.y = -self.jump_force;
                    self.movement_status = MovementStatus::InJump;
                    audio::play("jump-sfx-nothing");
                }
            },
            MovementStatus::InJump => {
                if self.rigid_body.velocity.y > 0. {
                    self.movement_status = MovementStatus::IsFalling;
                }
            },
            MovementStatus::IsFalling => {
                if self.coyote_timer > 0 && jump_just_pressed {
                    self.rigid_body.velocity.y = -self.jump_force;
                    self.movement_status = MovementStatus::InJump;
                    audio::play("jump-sfx-nothing");
                } else if jump_just_pressed {
                    self.jump_buffer_timer = self.jump_buffer_timer_duration;
                }
            }
        }

        if gp.left.pressed() {
            self.rigid_body.add_velocity(Vector2::new(-self.acceleration, 0.0));
            self.is_facing_left = true;
        } else if gp.right.pressed() {
            self.rigid_body.add_velocity(Vector2::new(self.acceleration, 0.0));
            self.is_facing_left = false;
        } else {
            if self.rigid_body.velocity.x > 0. {
                self.rigid_body.add_velocity(Vector2::new(-self.deceleration, 0.0));
            } else if self.rigid_body.velocity.x < 0. {
                self.rigid_body.add_velocity(Vector2::new(self.deceleration, 0.0));
            }
        }

        self.rigid_body.clamp_velocity_x(Vector2::new(-self.move_speed_max, self.move_speed_max));
        if self.movement_status != MovementStatus::IsLanded {
            let current_gravity = if self.movement_status == MovementStatus::IsFalling && !jump_pressed
                {
                    Vector2::new(0., self.gravity * 6.)
                } else if self.rigid_body.velocity.y < 25. {
                    Vector2::new(0., self.gravity / 1.5)
                } else { 
                    Vector2::new(0.,  self.gravity )
                };
            self.rigid_body.add_velocity(current_gravity);
            self.rigid_body.clamp_velocity_y(Vector2::new(-self.jump_force, self.max_gravity));
        }
        // Apply custom gravity

        if self.coyote_timer > 0 {
            self.coyote_timer -= 1;
        }
        
        if self.jump_buffer_timer > 0 {
            self.jump_buffer_timer -= 1;
        }
    }
    
    pub fn check_collision_tilemap(&mut self, tiles: &[Tile]) {
        // Check collision down
        if self.rigid_body.velocity.y > 0.0 {
            match check_collision(&self.rigid_body.position + &Vector2::new(0.0, self.rigid_body.velocity.y), Direction::Down, tiles) {
                Some(tile) => {
                    self.rigid_body.stop_y();
                    self.snap_to_tile(tile, Direction::Down);
                    self.movement_status = MovementStatus::IsLanded;
                },
                None => {
                    if self.movement_status == MovementStatus::IsLanded {
                        self.coyote_timer = self.coyote_timer_duration;
                        self.movement_status = MovementStatus::IsFalling;
                    }
                }
            }
        }

        // Check collision up
        if self.rigid_body.velocity.y < 0.0 {
            while self.rigid_body.velocity.y < 0.0 {
                match check_collision(&self.rigid_body.position + &Vector2::new(0.0, self.rigid_body.velocity.y), Direction::Up, tiles) {
                    Some(_) => { self.rigid_body.add_velocity(Vector2::new(0.0, 1.0)); }
                    None => { break; }
                }
            }
        }

        // Check collision right
        if self.rigid_body.velocity.x > 0.0 {
            while self.rigid_body.velocity.x > 0.0 {
                match check_collision(&self.rigid_body.position + &Vector2::new(self.rigid_body.velocity.x, 0.0), Direction::Right, tiles) {
                    Some(_) => { self.rigid_body.add_velocity(Vector2::new(-1.0, 0.0)); }
                    None => { break; }
                }
            }
        }

        // Check collision left
        if self.rigid_body.velocity.x < 0.0 {
            while self.rigid_body.velocity.x < 0.0 {
                match check_collision( &self.rigid_body.position + &Vector2::new(self.rigid_body.velocity.x, 0.0), Direction::Left, tiles) {
                    Some(_) => { self.rigid_body.add_velocity(Vector2::new(1.0, 0.0)); }
                    None => { break; }
                }
            }
        }
    }
    
    fn snap_to_tile(self: &mut Self, tile: &Tile, direction: Direction) {
        match direction {
            Direction::Down => {
                self.rigid_body.position.y = tile.top();
            }
            _ => {}
        }
    }

    pub fn draw(&self) {
        if self.movement_status == MovementStatus::IsLanded && self.rigid_body.velocity.x != 0. {
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

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
enum MovementStatus {
    IsLanded,
    IsFalling,
    InJump,
}