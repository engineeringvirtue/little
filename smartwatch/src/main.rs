extern crate little;
extern crate little_opengl;

mod model;

use little::*;
use little::drawing::{RGB, RGBA, Drawing, DrawText, FontBuffer, CharBuffer};
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

		surface.rect(vec2(0, 0), vec2(128, 128), &RGB(255, 255, 255));
		//since its a straight line it isnt perfectly antialiased really at all since i dont really know how antialiasing actually works
		surface.triangle([vec2(0, 0), vec2(50, 0), vec2(127, 127)], &RGBA(0, 0, 0, 100));
		//surface.copy_transform(vec2(-20, 0), vec2f(1.0, 1.0), 45.0, vec2f(0.0, 0.0), &RAINY);
		
		surface.ellipse(vec2(50, 50), 30, 30, 50, &RGBA(0, 255, 0, 255));
		// surface.ellipse(&vec2(50, 50), &vec2(128, 128), &RGB(0, 255, 0));
		// surface.text(&DrawText::new(&questrial, "10:10").font_size(1.3), vec2(10, 40), vec2(128,50), &RGBA(0, 0, 0, 255));
		// surface.line(vec2(0, 0), vec2(100, 128), &RGBA(0,0,0,255), 4);
		
		// surface.rect((50, 50), (128-50, 128-50), RGB(0, 0, 50));
		// surface.copy(vec2(40, 55), vec2(128-40, 128-55), &EHEHE, &Blend::Soft);
	}

	loop {

		if platform.step() {
			break;
		}
	}
}
