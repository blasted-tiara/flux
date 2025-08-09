use crate::*;

pub trait Bounded {
    fn get_bound(&self) -> BoundingBox;
}

#[turbo::serialize]
pub struct BoundingBox {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "top: {}, right: {}, bottom: {}, left: {}", self.top, self.right, self.bottom, self.left)
    }
}

impl BoundingBox {
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        !(
            self.left >= other.right ||
            other.left >= self.right ||
            self.bottom <= other.top ||
            other.bottom <= self.top
        )
    }
    
    pub fn contains(&self, point: Vector2) -> bool {
        point.x < self.right && point.x > self.left
    }
    
    pub fn draw_bounding_box(&self) {
        path!(
            start = (self.left, self.top),
            end = (self.left, self.bottom),
            color = 0xff00ffff,
        );
        path!(
            start = (self.left, self.bottom),
            end = (self.right, self.bottom),
            color = 0xff00ffff,
        );
        path!(
            start = (self.right, self.bottom),
            end = (self.right, self.top),
            color = 0xff00ffff,
        );
        path!(
            start = (self.right, self.top),
            end = (self.left, self.top),
            color = 0xff00ffff,
        );
    }
}

impl ops::Add<&Vector2> for BoundingBox {
    type Output = BoundingBox;
    
    fn add(self, rhs: &Vector2) -> Self::Output {
        BoundingBox {
            top: self.top + rhs.y,
            right: self.right + rhs.x,
            bottom: self.bottom + rhs.y,
            left: self.left + rhs.x,
        }
    }
}