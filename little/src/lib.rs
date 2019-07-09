#![no_std]
#![feature(core_intrinsics)]
#![feature(const_fn)]

pub mod util;
pub use util::*;

pub mod math;
pub use math::*;

pub mod region;
pub use region::{Bounded, Region};

pub mod drawing;
pub mod io;
pub mod anim;

pub fn transmute<T>(b: &[u8]) -> T {
	unsafe { core::ptr::read(b.as_ptr() as *const T) }
}

pub trait Platform<Surface: drawing::Buffer<Format=drawing::RGB> + drawing::WriteBuffer> {
	fn init() -> Self;
	
	fn surface(&mut self) -> &mut Surface;
	
	fn step(&mut self) -> bool;

	fn stop(self);
}
