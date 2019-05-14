#![feature(const_int_conversion)]

extern crate little;
extern crate little_opengl;

use little::*;
use little::drawing::{RGB, RGBA, Drawing, Mask, Buffer};

include_buffer!(Rainy, RGBA, "../assets/rainy.rc");

fn main() {
    let mut platform = little_opengl::OpenGLPlatform::init();
    
    {
        let surface = platform.surface();
        
        let rainy = Rainy;

        surface.rect((0, 0), (128, 128), RGB(255, 255, 255));
        surface.rect((30, 30), (128-30, 128-30), RGB(0, 255, 0));
        surface.rect((50, 50), (128-50, 128-50), RGB(0, 0, 50));
        surface.copy_mask((55, 55), (128-55, 128-55), &rainy);
    }

    loop {
        if platform.step() {
            break;
        }
    }
}
