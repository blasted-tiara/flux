use crate::*;

const DASH_TIMER: u32 = 6;
const DASH_SPEED_X: f32 = 12.;
const DASH_SPEED_Y: f32 = 8.;
const DASH_FLUX_THRESHOLD: f32 = 10.;

#[turbo::serialize]
pub struct Player {
    pub id: String,
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
    used_dash: bool,
    dash_timer: u32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self::new_with_id(String::new(), x, y)
    }

    pub fn new_with_id(id: String, x: f32, y: f32) -> Self {
        Self {
            id,
            actor: Actor::new(Vector2::new(x, y),20., 35.),
            velocity: Vector2::new(0., 0.),
            move_speed_max: 4.0,
            coyote_timer: 0,
            jump_buffer_timer: 0,
            gravity: 1.4,
            max_gravity: 15.,
            is_facing_left: true,
            acceleration: 0.5,
            deceleration: 0.5,
            jump_force: 12.0,
            coyote_timer_duration: 3,
            jump_buffer_timer_duration: 8,
            movement_status: MovementStatus::IsFalling,
            try_pick_item: false,
            picked_item: Option::None,
            used_dash: false,
            dash_timer: 0,
        }
    }
   
    pub fn handle_input(&mut self, actor_manager: &mut ActorManager, user_input: &UserInput, flux_field: Vector2) {
        match self.movement_status {
            MovementStatus::IsLanded => {
                if user_input.jump_just_pressed || self.jump_buffer_timer > 0 {
                    // Add jump force
                    self.velocity.y = -self.jump_force;
                    self.movement_status = MovementStatus::InJump;

                    // Set-up jump animation for idle and running animations
                    if self.velocity.x.abs() > 0.1 {
                        let anim = animation::get("player_character_walk");
                        anim.use_sprite("ChipmunckCharacter_jump");
                        anim.set_speed(2.0);
                        anim.set_repeat(1);
                    } else {
                        let anim = animation::get("player_character_idle");
                        anim.use_sprite("ChipmunckCharacter_jump");
                        anim.set_speed(2.0);
                        anim.set_repeat(1);
                    }

                    audio::play("jump-sfx-nothing");
                }
            },
            MovementStatus::InJump => {
                if !self.used_dash && user_input.jump_just_pressed {
                    if flux_field.length() > DASH_FLUX_THRESHOLD {
                        self.dash_timer = DASH_TIMER;
                        self.movement_status = MovementStatus::InDash;
                    }
                }
                if self.velocity.y > 0. {
                    self.movement_status = MovementStatus::IsFalling;
                }
            },
            MovementStatus::IsFalling => {
                if user_input.jump_just_pressed {
                    if self.coyote_timer > 0 {
                        // If coyote timer is active and jump is pressed, do a regular jump
                        self.velocity.y = -self.jump_force;
                        self.movement_status = MovementStatus::InJump;
                        audio::play("jump-sfx-nothing");
                    } else {
                        if self.used_dash {
                            // If dash is already used, buffer jump
                            self.jump_buffer_timer = self.jump_buffer_timer_duration;
                        } else {
                            // Otherwise, DASH!
                            
                            if flux_field.length() > DASH_FLUX_THRESHOLD {
                                self.dash_timer = DASH_TIMER;
                                self.movement_status = MovementStatus::InDash;
                            }
                        }
                    }
                }
            }
            MovementStatus::InDash => {
                if self.dash_timer > 0 {
                    if user_input.jump_pressed {
                        self.velocity = Vector2::new(0., -DASH_SPEED_Y);
                    } else if user_input.right_pressed {
                        self.velocity = Vector2::new(DASH_SPEED_X, 0.);
                    } else if user_input.left_pressed {
                        self.velocity = Vector2::new(-DASH_SPEED_X, 0.);
                    } else {
                        self.dash_timer = 0;
                        return;
                    }
                    self.dash_timer -= 1;
                    return;
                } else {
                    self.movement_status = MovementStatus::IsFalling;
                    self.used_dash = true;
                }

            }
        }

        if user_input.left_pressed {
            self.velocity += &Vector2::new(-self.acceleration, 0.0);
            self.is_facing_left = true;
        } else if user_input.right_pressed {
            self.velocity += & Vector2::new(self.acceleration, 0.0);
            self.is_facing_left = false;
        } else {
            if self.velocity.x > 0. {
                self.velocity.x = (self.velocity.x - self.deceleration).max(0.);
            } else if self.velocity.x < 0. {
                self.velocity.x = (self.velocity.x + self.deceleration).min(0.);
            }
        }

        self.velocity.clamp_x(-self.move_speed_max, self.move_speed_max);
        let current_gravity = if self.movement_status == MovementStatus::IsFalling && !user_input.jump_pressed
            {
                Vector2::new(0., self.gravity * 0.7)
            } else if self.velocity.y.abs() < 4.0 {
                Vector2::new(0., self.gravity / 4.)
            } else { 
                Vector2::new(0.,  self.gravity )
            };
        self.velocity += &current_gravity;
        self.velocity.clamp_y(-self.jump_force, self.max_gravity);

        if self.coyote_timer > 0 {
            self.coyote_timer -= 1;
        }
        
        if self.jump_buffer_timer > 0 {
            self.jump_buffer_timer -= 1;
        }

        if user_input.a_just_pressed {
            match self.picked_item {
                None => {
                    self.try_pick_item = true;
                },
                Some(actor_id) => {
                    match actor_manager.get_actor_mut(actor_id) {
                        Some(actor) => {
                            self.picked_item = Option::None;
                            actor.is_child = false;
                        },
                        None => {},
                    }
                }
            }
        }
    }
    
    pub fn pick_item(&mut self, actor_manager: &mut ActorManager) {
        if self.try_pick_item {
            let vertical_distance_tolerance = 20.0;
            let horizontal_distance_tolerance = 10.0;
            let player_bounding_box = self.actor.get_bound();
            for (actor_id, actor) in &mut actor_manager.actors {
                if actor.is_child == true {
                    continue;
                }
                let item_bounding_box = actor.get_bound();
                if (player_bounding_box.bottom - item_bounding_box.bottom).abs() < vertical_distance_tolerance {
                    if self.is_facing_left {
                        // check if there's an item close by to the left
                        if (item_bounding_box.right - player_bounding_box.left).abs() < horizontal_distance_tolerance {
                            self.picked_item = Option::Some(*actor_id);
                            actor.is_child = true;
                        }
                    } else {
                        if (item_bounding_box.left - player_bounding_box.right).abs() < horizontal_distance_tolerance {
                            self.picked_item = Option::Some(*actor_id);
                            actor.is_child = true;
                        }
                    }
                }
            }
            self.try_pick_item = false;
        }
    }
    
    pub fn actor_move(&mut self, tiles: &Vec<&Solid>, actor_manager: &mut ActorManager) {
        let current_velocity_x = self.velocity.x;
        let current_velocity_y = self.velocity.y;

        let on_x_collision = || {
            self.velocity.x = 0.;
        };
        self.actor.move_x(tiles, current_velocity_x, on_x_collision);

        let on_y_collision = |collision_happened: bool| {
            if collision_happened {
                if self.velocity.y >= 0.0 {
                    self.used_dash = false;
                    if self.movement_status == MovementStatus::IsFalling {
                        if self.velocity.x.abs() > 0.1 {
                            let anim = animation::get("player_character_walk");
                            anim.use_sprite("ChipmunckCharacter_land");
                            anim.set_speed(2.0);
                            anim.set_repeat(1);
                        } else {
                            let anim = animation::get("player_character_idle");
                            anim.use_sprite("ChipmunckCharacter_land");
                            anim.set_speed(2.0);
                            anim.set_repeat(1);
                        }
                        self.movement_status = MovementStatus::IsLanded;
                    }

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

        match self.picked_item {
            Some(actor_id) => {
                let item_option = actor_manager.get_actor_mut(actor_id);
                match item_option {
                    Some(actor) => {
                        let backpack_offset_x = 16.;
                        let backpack_offset_y = 9.;
                        if self.is_facing_left {
                            actor.position.x = self.actor.position.x + backpack_offset_x;
                        } else {
                            actor.position.x = self.actor.position.x - backpack_offset_x;
                        }
                        actor.position.y = self.actor.position.y - backpack_offset_y;
                    }
                    None => {},
                }
            },
            None => {},
        }
    }
    
    pub fn get_position(&self) -> Vector2 {
        Vector2 { x: self.actor.position.x, y: self.actor.position.y }
    }
    
    pub fn draw(&self) {
        let x_offset_holder = if self.is_facing_left { 17. } else { 19. };
        let y_offset_holder = 18.;
        
        match self.picked_item {
            Some(_) => {
                sprite!(
                    "energy_box_holder",
                    x = self.actor.position.x - x_offset_holder,
                    y = self.actor.position.y - y_offset_holder,
                    flip_x = self.is_facing_left,
                )
            },
            None => {}
        }

        let BoundingBox {top, right, bottom, left} = self.actor.get_bound();
        if self.movement_status == MovementStatus::IsLanded && self.velocity.x != 0. {
            let x_offset = if self.is_facing_left { 5 } else { 10 };
            let y_offset = 0;
            sprite!(
                animation_key = "player_character_walk",
                default_sprite = "ChipmunckCharacter_walk",
                x = left as i32 - x_offset,
                y = top as i32 - y_offset,
                flip_x = self.is_facing_left,
            );
        } else {
            let x_offset = if self.is_facing_left { 5 } else { 10 };
            let y_offset = 0;
            sprite!(
                animation_key = "player_character_idle",
                default_sprite = "ChipmunckCharacter_idle_36",
                x = left as i32 - x_offset,
                y = top as i32 - y_offset,
                flip_x = self.is_facing_left,
            );
        }
    }
    
    pub fn draw_bounding_box(&self) {
        self.actor.get_bound().draw_bounding_box();
    }
    
    pub fn write_info(&self) {
        let position = self.actor.position;
        let mut a = "Speed: ".to_owned();
        a.push_str(&(self.velocity.to_string()));

        text!(
            &a,
            x = position.x as i32 - 15,
            y = position.y as i32 - 28,
        );
    }
}

// Move these to a new file
pub fn draw_shader_distortion_parameter_pixel(net_flux_field: f32) {
    let color = ((net_flux_field as u32) << 16) | 0x000000ff;

    rect!(
        color = color,
        w = 1,
        h = 1,
        x = 0,
        y = 0,
        fixed = true,
    );
}

pub fn draw_menu_distortion_parameter_pixel() {
    rect!(
        color = 0x000000ff,
        w = 1,
        h = 1,
        x = 0,
        y = 0,
        fixed = true,
    );
}

#[turbo::serialize]
#[derive(PartialEq)]
enum MovementStatus {
    IsLanded,
    IsFalling,
    InJump,
    InDash,
}

impl std::fmt::Display for MovementStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MovementStatus::InJump => write!(f, "InJump"),
            MovementStatus::IsFalling => write!(f, "IsFalling"),
            MovementStatus::IsLanded => write!(f, "IsLanded"),
            MovementStatus::InDash => write!(f, "InDash"),
        }
    }
}