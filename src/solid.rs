use crate::*;

#[turbo::serialize]
pub struct Solid {
    pub position: Vector2,
    pub width: f32,
    pub height: f32,
}

impl Bounded for Solid {
    fn get_bound(&self) -> BoundingBox {
        BoundingBox {
            top: self.position.y - self.height / 2.,
            right: self.position.x + self.width / 2.,
            bottom: self.position.y + self.height / 2.,
            left: self.position.x - self.width / 2.,
        }
    }
}
