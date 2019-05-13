#![no_std]
#![feature(const_int_conversion)]

pub use core::mem;

pub mod util;
pub use util::*;

pub mod drawing;
pub mod input;