use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Copy)]
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
    
    pub fn clamp_x(&mut self, floor: f32, ceil: f32) {
        self.x = self.x.clamp(floor, ceil);
    }
    
    pub fn clamp_y(&mut self, floor: f32, ceil: f32) {
        self.y = self.y.clamp(floor, ceil);
    }
    
    pub fn rotate_point(&self, center: Vector2, amount: f32) -> Vector2 {
        let mut origin_centered = self.clone() - center;
        origin_centered.rotate_mut(amount);
        origin_centered + center
    }
    
    pub fn rotate_point_mut(&mut self, center: Vector2, amount: f32) {
        let mut origin_centered = self.clone() - center;
        origin_centered.rotate_mut(amount);
        let new_vector = origin_centered + center;

        self.x = new_vector.x;
        self.y = new_vector.y;
    }
    
    pub fn rotate(&self, amount: f32) -> Vector2 {
        let cs = amount.cos();
        let sn = amount.sin();
        
        Vector2::new(self.x * cs - self.y * sn, self.x * sn + self.y * cs)
    }
    
    pub fn rotate_mut(&mut self, amount: f32) {
        let cs = amount.cos();
        let sn = amount.sin();
    
        let old_self = self.clone();
        
        self.x = old_self.x * cs - old_self.y * sn;
        self.y = old_self.x * sn + old_self.y * cs;
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    
    pub fn draw(&self, size: u32) {
        circ!(d = size, x = self.x as i32 - size as i32 / 2, y = self.y as i32 - size as i32 / 2, color = 0xff00ffff);
    }

    pub fn draw_at_point(&self, point: &Vector2, scale: f32) {
        path!(
            start = (point.x as i32, point.y as i32),
            end = ((point.x + self.x * scale) as i32, (point.y + self.y * scale) as i32),
            color = 0xff00ffff,
        );
    }
    
    pub fn dot(&self, vector: &Vector2) -> f32 {
        self.x * vector.x + self.y * vector.y
    }
    
    pub fn normalize(&self) -> Vector2 {
        let length = self.length();
        Vector2::new(self.x / length, self.y / length)
    }
    
    pub fn get_normal_vector(&self) -> Vector2 {
        self.rotate(PI / 2.)
    }

    pub fn lerp(&self, rhs: &Vector2, alpha: f32) -> Vector2 {
        Vector2::new(lerp(self.x, rhs.x, alpha), lerp(self.y, rhs.y, alpha))
    }
 }

pub fn lerp(a: f32, b: f32, alpha: f32) -> f32 {
    a * (1. - alpha) + b * alpha
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

impl ops::Sub for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &Vector2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Add for Vector2 {
    type Output = Vector2;
    
    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub for Vector2 {
    type Output = Vector2;
    
    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Add<Vector2> for &Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}


impl ops::AddAssign<&Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: &Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::SubAssign<&Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: &Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;
    
    fn mul(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs) 
    }
}