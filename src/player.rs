use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct Player {
    pub actor: Actor,
    velocity: Vector2,
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
    try_pick_item: bool,
    picked_item: Option<ActorId>,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            actor: Actor::new(Vector2::new(x, y),45., 62.),
            velocity: Vector2::new(0., 0.),
            move_speed_max: 12.0,
            coyote_timer: 0,
            jump_buffer_timer: 0,
            gravity: 6.,
            max_gravity: 25.,
            is_facing_left: true,
            acceleration: 6.,
            deceleration: 2.0,
            jump_force: 30.,
            coyote_timer_duration: 3,
            jump_buffer_timer_duration: 8,
            movement_status: MovementStatus::IsFalling,
            try_pick_item: false,
            picked_item: Option::None,
        }
    }
   
    pub fn handle_input(&mut self) {
        let gp = gamepad(0);
        
        let jump_just_pressed = gp.up.just_pressed() || gp.start.just_pressed();
        let jump_pressed = gp.up.pressed() || gp.start.pressed();
        
        match self.movement_status {
            MovementStatus::IsLanded => {
                if jump_just_pressed || self.jump_buffer_timer > 0 {
                    self.velocity.y = -self.jump_force;
                    self.movement_status = MovementStatus::InJump;
                    audio::play("jump-sfx-nothing");
                }
                if gp.a.just_pressed() {
                    self.try_pick_item = true;
                }
            },
            MovementStatus::InJump => {
                if self.velocity.y > 0. {
                    self.movement_status = MovementStatus::IsFalling;
                }
            },
            MovementStatus::IsFalling => {
                if self.coyote_timer > 0 && jump_just_pressed {
                    self.velocity.y = -self.jump_force;
                    self.movement_status = MovementStatus::InJump;
                    audio::play("jump-sfx-nothing");
                } else if jump_just_pressed {
                    self.jump_buffer_timer = self.jump_buffer_timer_duration;
                }
            }
        }

        if gp.left.pressed() {
            self.velocity += &Vector2::new(-self.acceleration, 0.0);
            self.is_facing_left = true;
        } else if gp.right.pressed() {
            self.velocity += & Vector2::new(self.acceleration, 0.0);
            self.is_facing_left = false;
        } else {
            if self.velocity.x > 0. {
                self.velocity += &Vector2::new(-self.deceleration, 0.0);
            } else if self.velocity.x < 0. {
                self.velocity += &Vector2::new(self.deceleration, 0.0);
            }
        }

        self.velocity.clamp_x(-self.move_speed_max, self.move_speed_max);
        if self.movement_status != MovementStatus::IsLanded {
            let current_gravity = if self.movement_status == MovementStatus::IsFalling && !jump_pressed
                {
                    Vector2::new(0., self.gravity * 6.)
                } else if self.velocity.y < 25. {
                    Vector2::new(0., self.gravity / 3.)
                } else { 
                    Vector2::new(0.,  self.gravity )
                };
            self.velocity += &current_gravity;
            self.velocity.clamp_y(-self.jump_force, self.max_gravity);
        }

        if self.coyote_timer > 0 {
            self.coyote_timer -= 1;
        }
        
        if self.jump_buffer_timer > 0 {
            self.jump_buffer_timer -= 1;
        }
    }
    
    pub fn pick_item(&mut self, actors: &Vec<&Actor>) {
        
    }
    
    pub fn actor_move(&mut self, tiles: &Vec<&Solid>) {
        let current_velocity_x = self.velocity.x;
        let current_velocity_y = self.velocity.y;

        let on_x_collision = || {
            self.velocity.x = 0.;
        };
        self.actor.move_x(tiles, current_velocity_x, on_x_collision);

        let on_y_collision = |collision_happened: bool| {
            if collision_happened {
                if self.velocity.y > 0.0 {
                    self.movement_status = MovementStatus::IsLanded;
                }
                self.velocity.y = 0.;
            } else {
                if self.movement_status == MovementStatus::IsLanded {
                    self.coyote_timer = self.coyote_timer_duration;
                    self.movement_status = MovementStatus::IsFalling;
                }
            }

        };

        self.actor.move_y(tiles, current_velocity_y, on_y_collision);
    }
    
    pub fn get_position(&self) -> Vector2 {
        Vector2 { x: self.actor.position.x, y: self.actor.position.y }
    }
    
    pub fn draw(&self) {
        let BoundingBox {top, right, bottom, left} = self.actor.get_bound();
        if self.movement_status == MovementStatus::IsLanded && self.velocity.x != 0. {
            sprite!(
                "player1",
                x = left as i32,
                y = top as i32,
                flip_x = self.is_facing_left,
            );
        } else {
            let x_offset = if self.is_facing_left { 8 } else { 10 };
            let y_offset = 2;
            sprite!(
                "player1",
                x = left as i32 - x_offset,
                y = top as i32 - y_offset,
                flip_x = self.is_facing_left,
            );
        }
    }
    
    pub fn draw_bounding_box(&self) {
        self.actor.get_bound().draw_bounding_box();
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
enum MovementStatus {
    IsLanded,
    IsFalling,
    InJump,
}