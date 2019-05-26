// t = current time
// b = starting value to interpolate
// c = the total change in value of b that needs to occur
// d = total time it should take to complete (duration basically)

use super::*;
use io::GlobalTime;
use core::f32::consts::PI;

pub trait GetAtTime {
	fn get(&self, t: f32) -> f32;

	fn get_gt<T: GlobalTime>(&self, gt: &T) -> f32 {
		self.get(gt.get_s())
	}
}

pub enum Easing {
	Linear,
	SineIn, SineOut, SineInOut,
	CircIn, CircOut, CircInOut,
	CubicIn, CubicOut, CubicInOut,
	QuadIn, QuadOut, QuadInOut,
	ExpoIn, ExpoOut, ExpoInOut,
	BackIn, BackOut, BackInOut,
	BounceIn, BounceOut, BounceInOut,
	ElasticIn, ElasticOut, ElasticInOut
}

pub struct Animation {
	offset: f32,
	duration: f32,

	from: f32,
	to: f32,

	easing: Easing
}

impl Animation {
	pub fn new(off: f32, dur: f32, from: f32, to: f32, easing: Easing) -> Self {
		Animation {
			offset: off, duration: dur,
			from, to, easing
		}
	}
	
	pub fn new_gt<T: GlobalTime>(gt: &T, off: f32, dur: f32, from: f32, to: f32, easing: Easing) -> Self {
		Animation {
			offset: gt.get_s() + off,
			duration: dur, from, to, easing
		}
	}
}

impl GetAtTime for Animation {
	fn get(&self, mut t: f32) -> f32 {
		t += self.offset;
		let (c, d, b) = (self.to, self.duration, self.from);

		fn bounce_out(t: f32, c: f32, d: f32, b: f32) -> f32 {
			let mut td = t / d;
			if td < (1.0 / 2.75) {
				(c * (7.5625 * td * td) + b)
			} else if td < (2.0 / 2.75) {
				td -= 1.5 / 2.75;
				(c * (7.5625 * td * td + 0.75) + b)
			} else if td < (2.5 / 2.75) {
				td -= 2.25 / 2.75;
				(c * (7.5625 * td * td + 0.9375) + b)
			} else {
				td -= 2.625 / 2.75;
				(c * (7.5625 * td * td + 0.984375) + b)
			}
		}

		fn bounce_in(t: f32, c: f32, d: f32, b: f32) -> f32 {
			(c - bounce_out(d - t, 0.0, c, d) + b)
		}
		
		match self.easing {
			Easing::Linear => (c * t / d + b),
			Easing::SineIn => (-c * cos(t / d * (PI / 2.0)) + c + b),
			Easing::SineOut => (c * sin(t / d * (PI / 2.0)) + b),
			Easing::SineInOut => (-c / 2.0 * (cos(PI * t / d) - 1.0) + b),
			Easing::CircIn => {
				let td = t / d;
				(-c * ( sqrt(1.0 - td * td) - 1.0) + b)
			},
			Easing::CircOut => {
				let td = t / d - 1.0;
				(c * sqrt(1.0 - td * td) + b)
			},
			Easing::CircInOut => {
				let mut td = t / (d / 2.0);
				if td < 1.0 {
					(-c / 2.0 * (sqrt(1.0 - td * td) - 1.0) + b)
				} else {
					td -= 2.0;
					(c / 2.0 * (sqrt(1.0 - td * td) + 1.0) + b)
				}
			},
			Easing::CubicIn => {
				let td = t / d;
				(c * td * td * td + b)
			},
			Easing::CubicOut => {
				let td = t / d - 1.0;
				(c * (td * td * td + 1.0) + b)
			},
			Easing::CubicInOut => {
				let mut td = t / (d / 2.0);
				if td < 1.0 {
					(c / 2.0 * td * td * td + b)
				} else {
					td -= 2.0;
					(c / 2.0 * (td * td * td + 2.0) + b)
				}
			},
			Easing::QuadIn => {
				let td = t / d;
				(c * td * td + b)
			},
			Easing::QuadOut => {
				let td = t / d;
				(-c * td * (td - 2.0) + b)
			},
			Easing::QuadInOut => {
				let td = t / (d / 2.0);
				if td < 1.0 {
					(((c / 2.0) * (td * td)) + b)
				} else {
					(-c / 2.0 * (((td - 2.0) * (td - 1.0)) - 1.0) + b)
				}
			},
			Easing::ExpoIn => {
				if t == 0.0 {
					b
				} else {
					(c * pow(2.0f32, 10.0 * (t / d - 1.0)) + b)
				}
			},
			Easing::ExpoOut => {
				if t == d {
					(b + c)
				} else {
					(c * (-pow(2.0f32, -10.0 * t / d) + 1.0) + b)
				}
			},
			Easing::ExpoInOut => {
				if t == 0.0 { return b; }
				else if t == d { return b + c; }

				let td = t / (d / 2.0);
				if td < 1.0 {
					return c / 2.0 * pow(2.0f32, 10.0 * (t - 1.0)) + b;
				} else {
					return c / 2.0 * (-pow(2.0f32, -10.0 * td - 1.0) + 2.0) + b;
				}
			},
			Easing::BackIn => {
				let s = 1.70158f32;
				let postfix = t / d;
				(c * postfix * postfix * ((s + 1.0) * postfix - s) + b)
			},
			Easing::BackOut => {
				let s = 1.70158f32;
				let td = t / d - 1.0;
				(c * (td * td * ((s + 1.0) * td + s) + 1.0) + b)
			},
			Easing::BackInOut => {
				let mut s = 1.70158f32;
				let td = t / (d / 2.0);
				if td < 1.0 {
					s *= 1.525;
					(c / 2.0 * (td * td* ((s + 1.0) * td - s)) + b)
				} else {
					let postfix = t - 2.0;
					s *= 1.525;
					(c / 2.0 * ((postfix) * postfix * ((s + 1.0) * t + s) + 2.0) + b)
				}
			},
			Easing::BounceOut => bounce_out(t, c, d, b),
			Easing::BounceIn => bounce_in(t, c, d, b),
			Easing::BounceInOut => {
				if t < (d / 2.0) {
					(bounce_in(t * 2.0, 0.0, c, d) * 0.5) + b
				} else {
					(bounce_out(t * 2.0 - d, 0.0, c, d) * 0.5) + (c * 0.5) + b
				}
			},
			Easing::ElasticIn => {
				let mut td = t / d;
				
				if t == 0.0 { b }
				else if td == 1.0 { b + c }
				else {
					let p = d * 0.3;
					let a = c;
					let s = p / 4.0;
					td -= 1.0;
					let postfix = a * pow(2.0f32, 10.0 * td);
					(-(postfix * sin((td * d - s) * (2.0 * PI) / p)) + b)
				}
			},
			Easing::ElasticOut => {
				let td = t / d;

				if t == 0.0 { b }
				else if td == 1.0 { b + c }
				else {
					let p = d * 0.3;
					let a = c;
					let s = p / 4.0;
					(a * pow(2.0f32, -10.0 * td) * sin((td * d - s) * (2.0 * PI) / p) + c + b)
				}
			},
			Easing::ElasticInOut => {
				let mut td = t / (d / 2.0);
				
				if t == 0.0 { b }
				else if td == 2.0 { b + c }
				else {
					let p = d * (0.3 * 1.5);
					let a = c;
					let s = p / 4.0;
					if td < 1.0 {
						td -= 1.0;
						let postfix = a * pow(2.0f32, 10.0 * td);
						-0.5 * (postfix*sin((td*d-s)*(2.0 * PI)/p)) + b
					}
					else {
						td -= 1.0;
						let postfix = a * pow(2.0f32, -10.0 * td);
						(postfix * sin((td * d - s) * (2.0 * PI) / p) * 0.5 + c + b)
					}
				}
			}
		}
	}
}

pub struct LoopingAnimation {
	pub anim: Animation
}

impl GetAtTime for LoopingAnimation {
	fn get(&self, t: f32) -> f32 {
		let t = abs(self.anim.duration - t%(self.anim.duration*2.0));
		self.anim.get(t)
	}
}