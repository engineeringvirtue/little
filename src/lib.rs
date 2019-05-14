#![no_std]
#![feature(const_int_conversion)]

pub use core::mem;

pub mod util;
pub use util::*;

pub mod drawing;
pub mod input;

pub trait Platform<Surface: drawing::Buffer<drawing::RGB>> {
    fn init() -> Self;
    
    fn surface(&mut self) -> &mut Surface;
    
    fn input_state(&self) -> &input::InputState;
    
    fn step(&mut self) -> bool;

    fn stop(self);
}