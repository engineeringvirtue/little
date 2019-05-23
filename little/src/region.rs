use super::*;

pub struct Region {
	pub from: Vector2,
	pub to: Vector2
}

impl Region {
	pub fn new(from: Vector2, to: Vector2) -> Self {
		Region {from, to}
	}
}