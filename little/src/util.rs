use super::*;
use core::intrinsics::*;

pub fn transmute<T>(b: &[u8]) -> T {
	unsafe { core::ptr::read(b.as_ptr() as *const T) }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2 { pub x: i32, pub y: i32 }
pub struct Matrix2d { pub a: f32, pub b: f32, pub c: f32, pub d: f32 }

#[derive(Debug, Clone, Copy)]
pub struct Vector2f { pub x: f32, pub y: f32 }

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

pub fn mat2(a: f32, b: f32, c: f32, d: f32) -> Matrix2d {
    Matrix2d {
        a, b,
        c, d
    }
}

pub fn vec2(x: i32, y: i32) -> Vector2 {
	Vector2 {x, y}
}

pub fn vec2f(x: f32, y: f32) -> Vector2f {
	Vector2f {x, y}
}

pub struct Region {
	pub from: Vector2,
	pub to: Vector2
}

impl Region {
	pub fn new(from: Vector2, to: Vector2) -> Self {
		Region {from, to}
	}
}

pub fn cos(f: f32) -> f32 {
	unsafe { cosf32(f) }
}

pub fn sin(f: f32) -> f32 {
	unsafe { sinf32(f) }
}

pub fn sqrt(f: f32) -> f32 {
	unsafe { sqrtf32(f) }
}

pub fn abs(f: f32) -> f32 {
	unsafe { fabsf32(f) }
}

pub fn floor(f: f32) -> f32 {
	unsafe { floorf32(f) }
}

pub fn ceil(f: f32) -> f32 {
	unsafe { ceilf32(f) }
}

pub fn pow(f: f32, x: f32) -> f32 {
	unsafe { powf32(f, x) }
}

pub fn rotation_2d(angle: f32) -> Matrix2d {
    let s: f32 = sin(angle);
    let c: f32 = cos(angle);

    mat2(
    	c, -s,
    	s, c
    )
}

pub fn rotate(pos: Vector2f, angle: f32) -> Vector2 {
    let a: Matrix2d = rotation_2d(angle);
    a * pos
}
