#![feature(const_int_conversion)]

extern crate little;
extern crate little_opengl;

use little::*;
use little::drawing::{RGB, RGBA, interpolate, Blend, Drawing, DrawText, FontBuffer, CharBuffer};
use little::{Pos, pos};

include_buffer!(RAINY, RGBA, "../assets/rainy.rc");
include_buffer!(EHEHE, RGBA, "../assets/ehehe.rc");
include_font!(QUESTRIAL, "../assets/Questrial/Questrial-Regular.rc");

fn main() {
    let mut platform = little_opengl::OpenGLPlatform::init();
    
    {
        let surface = platform.surface();

        let questrial = QUESTRIAL;

        let rainy = RAINY;

        surface.rect(&pos(0, 0), &pos(128, 128), (&RGB(255, 255, 255), &Blend::Write));
        // surface.rect(&pos(30, 30), &pos(128-30, 128-30), (&RGB(0, 255, 0), &Blend::Write));

        // surface.triangle([&pos(128, 0), &pos(128, 128), &pos(0, 0)], (&RGBA(0, 0, 0, 100), &Blend::Soft));
        let points = [&pos(48, 10), &pos(128, 87), &pos(0, 110)];

        // surface.triangle(points, (&RGBA(0, 0, 0, 100), &Blend::Soft));
        surface.text(&DrawText::new(&questrial, "10:10").font_size(0.6), &pos(0, 50), &pos(128,50), (&RGBA(0, 0, 0, 255), &Blend::Hard));
        
        // surface.line(&pos(30, 0), &pos(0, 128), (&RGBA(0, 0, 0, 100), &Blend::Soft), 5);
        
        // surface.rect((50, 50), (128-50, 128-50), RGB(0, 0, 50));
        // surface.copy(pos(40, 55), pos(128-40, 128-55), &EHEHE, &Blend::Soft);
    }

    loop {
        if platform.step() {
            break;
        }
    }
}