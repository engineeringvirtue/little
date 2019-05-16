use super::*;

pub fn transmute<T>(b: &[u8]) -> T {
    unsafe { ptr::read(b.as_ptr() as *const T) }
}

#[derive(Debug, Clone, Copy)]
pub struct Pos { pub x: i32, pub y: i32 }

impl core::ops::Mul<Pos> for Pos {
    type Output = Pos;
    fn mul(self, rhs: Pos) -> Pos {
        Pos {x: rhs.x * self.x, y: rhs.y * self.y}
    }
}

impl core::ops::MulAssign<Pos> for Pos {
    fn mul_assign(&mut self, rhs: Pos) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl core::ops::Sub<Pos> for Pos {
    type Output = Pos;
    fn sub(self, rhs: Pos) -> Pos {
        Pos {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl core::ops::Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Pos {
        Pos {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl From<(i32, i32)> for Pos {
    fn from(x: (i32, i32)) -> Pos {
        Pos {x: x.0, y: x.1}
    }
}

pub fn pos(x: i32, y: i32) -> Pos {
    Pos {x, y}
}

pub struct Region {
    pub from: Pos,
    pub to: Pos
}

impl Region {
    pub fn new(from: Pos, to: Pos) -> Self {
        Region {from, to}
    }

    pub fn inside(&self, pos: Pos) -> bool {
        pos.x >= self.from.x && pos.y >= self.from.y
            && pos.x <= self.to.x && pos.y <= self.to.y
    }
}