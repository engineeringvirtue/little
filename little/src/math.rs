use core::intrinsics::*;

#[derive(Debug, Clone, Copy)]
pub struct Vector2 { pub x: i32, pub y: i32 }

#[derive(Debug, Clone, Copy)]
pub struct Vector2f { pub x: f32, pub y: f32 }

macro_rules! impl_math {
	($name: ident, $($field: ident),+) => {
		impl core::ops::Add for $name {
			type Output = Self;

			fn add(self, other: Self) -> Self {
				$name {$($field: self.$field + other.$field),*}
			}
		}
		
		impl core::ops::AddAssign for $name {		
			fn add_assign(&mut self, x: Self) {
				$(self.$field += x.$field;)*
			}
		}
		
		impl core::ops::Sub for $name {
			type Output = Self;

			fn sub(self, other: Self) -> Self {
				$name {$($field: self.$field - other.$field),*}
			}
		}
		
		impl core::ops::SubAssign for $name {		
			fn sub_assign(&mut self, x: Self) {
				$(self.$field -= x.$field;)*
			}
		}

		impl core::ops::Div for $name {
			type Output = Self;

			fn div(self, other: Self) -> Self {
				$name {$($field: self.$field / other.$field),*}
			}
		}
		
		impl core::ops::DivAssign for $name {		
			fn div_assign(&mut self, x: Self) {
				$(self.$field /= x.$field;)*
			}
		}

		impl core::ops::Rem for $name {
			type Output = Self;

			fn rem(self, other: Self) -> Self {
				$name {$($field: self.$field % other.$field),*}
			}
		}
		
		impl core::ops::RemAssign for $name {		
			fn rem_assign(&mut self, x: Self) {
				$(self.$field %= x.$field;)*
			}
		}
	};
}

macro_rules! impl_math_single {
	($name: ident, $other: ident, $($field: ident),+) => {
		impl core::ops::Add<$other> for $name {
			type Output = Self;

			fn add(self, other: $other) -> Self {
				$name {$($field: self.$field + other),*}
			}
		}
		
		impl core::ops::AddAssign<$other> for $name {		
			fn add_assign(&mut self, x: $other) {
				$(self.$field += x;)*
			}
		}
		
		impl core::ops::Sub<$other> for $name {
			type Output = Self;

			fn sub(self, other: $other) -> Self {
				$name {$($field: self.$field - other),*}
			}
		}
		
		impl core::ops::SubAssign<$other> for $name {		
			fn sub_assign(&mut self, x: $other) {
				$(self.$field -= x;)*
			}
		}

		impl core::ops::Div<$other> for $name {
			type Output = Self;

			fn div(self, other: $other) -> Self {
				$name {$($field: self.$field / other),*}
			}
		}
		
		impl core::ops::DivAssign<$other> for $name {		
			fn div_assign(&mut self, x: $other) {
				$(self.$field /= x;)*
			}
		}

		impl core::ops::Rem<$other> for $name {
			type Output = Self;

			fn rem(self, other: $other) -> Self {
				$name {$($field: self.$field % other),*}
			}
		}
		
		impl core::ops::RemAssign<$other> for $name {		
			fn rem_assign(&mut self, x: $other) {
				$(self.$field %= x;)*
			}
		}
	};
}

impl Into<Vector2f> for Vector2 {
	fn into(self) -> Vector2f {
		vec2f(self.x as f32, self.y as f32)
	}
}

impl Into<Vector2> for Vector2f {
	fn into(self) -> Vector2 {
		vec2(self.x as i32, self.y as i32)
	}
}

impl_math!(Vector2, x, y);
impl_math!(Vector2f, x, y);

impl_math_single!(Vector2, i32, x, y);
impl_math_single!(Vector2f, f32, x, y);

pub const fn vec2(x: i32, y: i32) -> Vector2 {
	Vector2 {x, y}
}

pub const fn vec2f(x: f32, y: f32) -> Vector2f {
	Vector2f {x, y}
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

// pub fn atan() -> f32 {
// 	HAYMAN
// }

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

pub const fn deg(degrees: f32) -> f32 {
	degrees / 57.3
}