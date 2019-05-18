#![no_std]
#![feature(core_intrinsics)]

pub mod util;
pub use util::*;

pub mod drawing;
pub mod input;
pub mod ease;

pub trait Platform<Surface: drawing::Buffer<Format=drawing::RGB> + drawing::WriteBuffer> {
    fn init() -> Self;
    
    fn surface(&mut self) -> &mut Surface;
    
    fn step(&mut self) -> bool;

    fn stop(self);
}
