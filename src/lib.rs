#![no_std]
#![feature(const_int_conversion)]

pub use core::{mem, ptr};

pub mod util;
pub use util::*;

pub mod drawing;
pub mod input;

pub trait Platform<Surface: drawing::Buffer<Format=drawing::RGB> + drawing::WriteBuffer> {
    fn init() -> Self;
    
    fn surface(&mut self) -> &mut Surface;
        
    fn step(&mut self) -> bool;

    fn stop(self);
}