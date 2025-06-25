use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct Player {
    pub rigid_body: RigidBody,
    pub is_facing_left: bool,
    pub is_powering_jump: bool,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            rigid_body: RigidBody {
                position: Vector2::new(x, y),
                velocity: Vector2::zero(),
                rotation: 0.0,
                max_gravity: 15.0,
                is_falling: false,
                is_landed: false,
                coyote_timer: 0,
            },
            is_facing_left: true,
            is_powering_jump: false,
        }
    }
    pub fn handle_input(&mut self) {
        let gp = gamepad(0);
        if (gp.up.just_pressed() || gp.start.just_pressed())
            && (self.rigid_body.is_landed || self.rigid_body.coyote_timer > 0)
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
            self.rigid_body.apply_gravity();
        }
        self.rigid_body.clamp_velocity_y(Vector2::new(-PLAYER_MAX_JUMP_FORCE, self.rigid_body.max_gravity));

        if self.rigid_body.coyote_timer > 0 {
            self.rigid_body.coyote_timer -= 1;
        }
    }

    pub fn draw(&self) {
        if self.rigid_body.is_landed && self.rigid_body.velocity.x != 0. {
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
