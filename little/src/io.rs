use super::*;

#[derive(Clone, Debug)]
pub enum TouchGesture {
	Tap(Vector2),
	ScrollY(Vector2, i32),
	ScrollX(Vector2, i32)
}


#[derive(Clone, Debug)]
pub struct TouchInputState {
	pub start: Option<Vector2>,
	pub current: Option<Vector2>,
	gesture: Option<TouchGesture>,

	pub threshold: i32
}

impl TouchInputState {
	pub fn new(threshold: i32) -> Self {
		TouchInputState {
			start: None, current: None, gesture: None, threshold
		}
	}

	pub fn set_gesture(&mut self, gesture: TouchGesture) {
		self.gesture = Some(gesture);
	}

	pub fn touch_down(&mut self, pos: Vector2) {
		self.start = Some(pos);
		self.current = Some(pos);
	}

	pub fn touch_move(&mut self, pos: Vector2) {
		if let None = self.start {
			self.start = Some(pos);
		}
		
		self.current = Some(pos);

		if let (Some(current), Some(start)) = (&self.current, self.start) {
			let diff = *current - start;
			
			if diff.x.abs() > self.threshold {
				self.set_gesture(TouchGesture::ScrollX(start, diff.x));
			} else if diff.y.abs() > self.threshold {
				self.set_gesture(TouchGesture::ScrollY(start, diff.y));
			}
		}
	}

	pub fn touch_up(&mut self) {
		if let (Some(current), None) = (self.current.take(), &self.gesture) {
			self.set_gesture(TouchGesture::Tap(current));
		}

		self.start = None;
	}

	pub fn get_gesture(&self) -> &Option<TouchGesture> {
		&self.gesture
	}
}

pub trait TouchInput {
	fn touch_step(&mut self) -> &TouchInputState;
}

pub trait GlobalTime {
	fn get_ms(&self) -> usize;
	fn get_s(&self) -> f32;

	fn reset(&mut self);
}

pub trait BluetoothIO {
	fn discover(&mut self);
	fn disconnect(&mut self);
	fn connected(&mut self) -> bool;

	fn send(&mut self, data: &[u8]);
	fn recieve(&mut self) -> Option<(usize, [u8; 1024])>;
}