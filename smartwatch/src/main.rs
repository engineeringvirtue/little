#![feature(const_int_conversion)]

extern crate little;
extern crate little_opengl;

use little::*;
use little::drawing::{RGB, RGBA, interpolate, Blend, Drawing};
use little::{Pos, pos};

include_buffer!(Rainy, RGBA, "../assets/rainy.rc");

fn main() {
    let mut platform = little_opengl::OpenGLPlatform::init();
    
    {
        let surface = platform.surface();

        let rainy = Rainy;

        surface.rect(&pos(0, 0), &pos(128, 128), (&RGB(255, 255, 255), &Blend::Write));
        surface.rect(&pos(30, 30), &pos(128-30, 128-30), (&RGB(0, 255, 0), &Blend::Write));

        // surface.triangle(&mut [pos(128, 0), pos(128, 128), pos(0, 0)], (&RGBA(0, 0, 0, 100), &Blend::Soft));
        surface.triangle(&mut [pos(0, 129), pos(129, 129), pos(30, 0)], (&RGBA(0, 0, 0, 100), &Blend::Soft));
        
        // surface.line(&pos(30, 0), &pos(0, 128), (&RGBA(0, 0, 0, 100), &Blend::Soft), 5);
        
        // surface.rect((50, 50), (128-50, 128-50), RGB(0, 0, 50));
        // surface.copy_mask((55, 55), (128-55, 128-55), &rainy);
    }

    loop {
        if platform.step() {
            break;
        }
    }
}
