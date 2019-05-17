extern crate little;
extern crate little_opengl;

use little::*;
use little::input::*;
use little::drawing::{RGB, RGBA, interpolate, Blend, Drawing, DrawText, FontBuffer, CharBuffer};
use little::{Vector2, vec2};

include_buffer!(RAINY, RGBA, "../assets/rainy.rc");
include_buffer!(EHEHE, RGBA, "../assets/ehehe.rc");
include_font!(QUESTRIAL, "../assets/Questrial/Questrial-Regular.rc");

fn main() {
    let mut platform = little_opengl::OpenGLPlatform::init();
    
    {
        let surface = platform.surface();

        let questrial = QUESTRIAL;

        let rainy = RAINY;

        surface.rect(&vec2(0, 0), &vec2(128, 128), (&RGB(255, 255, 255), &Blend::Write));
        // surface.rect(&vec2(30, 30), &vec2(128-30, 128-30), (&RGB(0, 255, 0), &Blend::Write));

        // surface.triangle([&vec2(128, 0), &vec2(128, 128), &vec2(0, 0)], (&RGBA(0, 0, 0, 100), &Blend::Soft));
        let points = [&vec2(48, 10), &vec2(128, 87), &vec2(0, 110)];

        // surface.triangle(points, (&RGBA(0, 0, 0, 100), &Blend::Soft));
        surface.text(&DrawText::new(&questrial, "10:10").font_size(1.3), &vec2(10, 40), &vec2(128,50), (&RGBA(0, 0, 0, 255), &Blend::Soft));
        
        surface.line(&vec2(30, 0), &vec2(0, 127), (&RGBA(0, 0, 0, 100), &Blend::Soft), 5);
        
        // surface.rect((50, 50), (128-50, 128-50), RGB(0, 0, 50));
        // surface.copy(vec2(40, 55), vec2(128-40, 128-55), &EHEHE, &Blend::Soft);
    }

    loop {
        if platform.step() {
            break;
        }
    }
}
