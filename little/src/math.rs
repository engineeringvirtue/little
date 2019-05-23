use core::intrinsics::*;

pub fn transmute<T>(b: &[u8]) -> T {
	unsafe { core::ptr::read(b.as_ptr() as *const T) }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2 { pub x: i32, pub y: i32 }

#[derive(Debug, Clone, Copy)]
pub struct Vector2f { pub x: f32, pub y: f32 }

#[derive(Debug, Clone, Copy)]
pub struct Matrix2d { pub a: f32, pub b: f32, pub c: f32, pub d: f32 }

macro_rules! impl_math {
	($name: ident, $($field: ident),+) => {
		impl core::ops::Add<Self> for $name {
			type Output = Self;

			fn add(self, other: Self) -> Self {
				$name {$($field: self.$field + other.$field),*}
			}
		}
		
		impl core::ops::AddAssign<Self> for $name {		
			fn add_assign(&mut self, x: Self) {
				$(self.$field += x.$field;)*
			}
		}
		
		impl core::ops::Sub<Self> for $name {
			type Output = Self;

			fn sub(self, other: Self) -> Self {
				$name {$($field: self.$field - other.$field),*}
			}
		}
		
		impl core::ops::SubAssign<Self> for $name {		
			fn sub_assign(&mut self, x: Self) {
				$(self.$field -= x.$field;)*
			}
		}

		impl core::ops::Div<Self> for $name {
			type Output = Self;

			fn div(self, other: Self) -> Self {
				$name {$($field: self.$field / other.$field),*}
			}
		}
		
		impl core::ops::DivAssign<Self> for $name {		
			fn div_assign(&mut self, x: Self) {
				$(self.$field /= x.$field;)*
			}
		}

		impl core::ops::Rem<Self> for $name {
			type Output = Self;

			fn rem(self, other: Self) -> Self {
				$name {$($field: self.$field % other.$field),*}
			}
		}
		
		impl core::ops::RemAssign<Self> for $name {		
			fn rem_assign(&mut self, x: Self) {
				$(self.$field %= x.$field;)*
			}
		}
	};
}

impl Into<Vector2f> for Vector2 {
	fn into(self) -> Vector2f {
		vec2f(self.x as f32, self.y as f32)
	}
}

impl core::ops::Mul<Matrix2d> for Vector2f {
	type Output = Vector2f;

	fn mul(self, other: Matrix2d) -> Self {
		vec2f((self.x * other.a) + (self.x * other.c),
			(self.y * other.b) + (self.y * other.d))
	}
}

impl_math!(Vector2, x, y);
impl_math!(Vector2f, x, y);
impl_math!(Matrix2d, a, b, c, d);

pub fn vec2(x: i32, y: i32) -> Vector2 {
	Vector2 {x, y}
}

pub fn vec2f(x: f32, y: f32) -> Vector2f {
	Vector2f {x, y}
}

pub fn mat2(a: f32, b: f32, c: f32, d: f32) -> Matrix2d {
    Matrix2d {
        a, b,
        c, d
    }
}

pub fn rotation_2d(angle: f32) -> Matrix2d {
    let s: f32 = sin(angle);
    let c: f32 = cos(angle);

    mat2(
    	c, -s,
    	s, c
    )
}

pub fn rotate(pos: Vector2f, angle: f32) -> Vector2f {
    let a: Matrix2d = rotation_2d(angle);
    pos * a
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