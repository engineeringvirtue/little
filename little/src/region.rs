use super::*;

pub trait Bounded {
	fn inside(&self, pos: Vector2) -> bool;
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

impl Bounded for Region {
	fn inside(&self, pos: Vector2) -> bool {
		pos.x >= self.from.x && pos.y >= self.from.y && pos.x < self.to.x && pos.y < self.to.y
	}
}