use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct Player {
    pub rigid_body: RigidBody,
    is_facing_left: bool,
    coyote_timer: i32,
    gravity: f32,
    fall_gravity: f32,
    max_gravity: f32,
    move_speed_max: f32,
    acceleration: f32,
    deceleration: f32,
    jump_force: f32,
    coyote_timer_duration: u32,
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
            move_speed_max: 8.0,
            coyote_timer: 0,
            gravity: 6.5,
            fall_gravity: 15.,
            max_gravity: 15.,
            is_facing_left: true,
            acceleration: 4.,
            deceleration: 2.,
            jump_force: 60.,
            coyote_timer_duration: 3,
            movement_status: MovementStatus::IsFalling,
        }
    }
    
    pub fn handle_input(&mut self) {
        let gp = gamepad(0);
        
        let jump_just_pressed = gp.up.just_pressed() || gp.start.just_pressed();
        
        match self.movement_status {
            MovementStatus::IsLanded => {
                if jump_just_pressed {
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
        // Apply custom gravity
        let current_gravity = if self.movement_status == MovementStatus::IsFalling 
            {
                Vector2::new(0., self.fall_gravity)
            } else { 
                Vector2::new(0.,  self.gravity )
            };
        self.rigid_body.add_velocity(current_gravity);
        self.rigid_body.clamp_velocity_y(Vector2::new(-self.jump_force, self.max_gravity));

        if self.coyote_timer> 0 {
            self.coyote_timer -= 1;
        }
    }
    
    pub fn check_collision_tilemap(&mut self, tiles: &[Tile]) {
        // Check collision down
        if self.rigid_body.velocity.y > 0.0 {
            if check_collision(&self.rigid_body.position + &Vector2::new(0.0, self.rigid_body.velocity.y), Direction::Down, tiles) {
                self.rigid_body.stop_y();
                self.movement_status = MovementStatus::IsLanded;
            } else {
                if self.movement_status == MovementStatus::IsLanded {
                    self.movement_status = MovementStatus::IsFalling;
                    self.coyote_timer = self.coyote_timer;
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