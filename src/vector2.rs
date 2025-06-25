use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }
    
    pub fn zero() -> Self {
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