use super::*;

pub fn transmute<T>(b: &[u8]) -> T {
    unsafe { ptr::read(b.as_ptr() as *const T) }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2 { pub x: i32, pub y: i32 }

impl Vector2 {
    pub const MAX: Self = Vector2 {x: i32::max_value(), y: i32::max_value()};
}

impl core::ops::Mul<Vector2> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {x: rhs.x * self.x, y: rhs.y * self.y}
    }
}

impl core::ops::MulAssign<Vector2> for Vector2 {
    fn mul_assign(&mut self, rhs: Vector2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl core::ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl core::ops::Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl From<(i32, i32)> for Vector2 {
    fn from(x: (i32, i32)) -> Vector2 {
        Vector2 {x: x.0, y: x.1}
    }
}

pub fn vec2(x: i32, y: i32) -> Vector2 {
    Vector2 {x, y}
}

pub struct Region {
    pub from: Vector2,
    pub to: Vector2
}

impl Region {
    pub fn new(from: Vector2, to: Vector2) -> Self {
        Region {from, to}
    }

    pub fn inside(&self, pos: Vector2) -> bool {
        pos.x >= self.from.x && pos.y >= self.from.y
            && pos.x <= self.to.x && pos.y <= self.to.y
    }
}
