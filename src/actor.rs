use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct Actor {
    pub position: Vector2,
    pub is_child: bool,
    remainder: Vector2,
    width: f32,
    height: f32,
    carried_by_player: bool,
}

impl Actor {
    pub fn new(position: Vector2, width: f32, height: f32) -> Self {
        Self {
            is_child: false,
            position,
            remainder: Vector2::zero(),
            width,
            height,
            carried_by_player: false,
        }
    }

    pub fn move_x<F: FnMut()>(&mut self, solids: &Vec<&Solid>, amount: f32, mut on_collide: F) {
        let mut steps_to_move = amount.trunc() as i32; 
        
        if steps_to_move != 0 {
            self.remainder.x -= steps_to_move as f32;
            let sign = steps_to_move.signum();
            while steps_to_move != 0 {
                let next_step = self.get_bound() + &Vector2::new(sign as f32, 0.);
                if !collide_at(&solids, &next_step) {
                    self.position.x += sign as f32;
                    steps_to_move -= sign;
                } else {
                    on_collide();
                    return;
                }
            }
        }
    }
    
    pub fn move_y<F: FnMut(bool)>(&mut self, solids: &Vec<&Solid>, amount: f32, mut on_collide: F) {
        let mut steps_to_move = amount.trunc() as i32; 
        
        if steps_to_move != 0 {
            self.remainder.y -= steps_to_move as f32;
            let sign = steps_to_move.signum();
            while steps_to_move != 0 {
                let next_step = self.get_bound() + &Vector2::new(0., sign as f32);
                if !collide_at(&solids, &next_step) {
                    self.position.y += sign as f32;
                    steps_to_move -= sign;
                } else {
                    on_collide(true);
                    return;
                }
            }
            on_collide(false);
        }
    }
}

impl Bounded for Actor {
    fn get_bound(&self) -> BoundingBox {
        BoundingBox {
            top: self.position.y - self.height / 2.,
            right: self.position.x + self.width / 2.,
            bottom: self.position.y + self.height / 2.,
            left: self.position.x - self.width / 2.,
        }
    }
}

//check collision betwen an actor and a set of solids
pub fn collide_at(solids: &Vec<&Solid>, bounding_box: &BoundingBox) -> bool {
    for solid in solids {
        if solid.get_bound().intersects(&bounding_box) {
            return true;
        }
    }
    false
}

pub fn collide_with(actors: &Vec<&Actor>, bounding_box: &BoundingBox) -> bool {
    for actor in actors {
        if actor.get_bound().intersects(&bounding_box) {
            return true;
        }
    }
    false
}
