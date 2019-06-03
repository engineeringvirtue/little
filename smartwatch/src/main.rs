extern crate little;
extern crate little_emu;

mod model;

use little::*;
use little::io::*;
use little::drawing::{RGB, RGBA, Drawing, DrawText, FontBuffer, CharBuffer};
use little::{Vector2, vec2};

include_buffer!(RAINY, RGBA, "../assets/rainy.rc");
include_buffer!(EHEHE, RGBA, "../assets/ehehe.rc");
include_font!(QUESTRIAL, "../assets/Questrial/Questrial-Regular.rc");

fn main() {
	let mut platform = little_emu::OpenGLPlatform::init();

	let surface = platform.surface();

	let questrial = QUESTRIAL;
	let rainy = RAINY;

	surface.rect(vec2(0,0,), vec2(128,128), &RGB(0, 0, 0));

	surface.ellipse(vec2(50, 50), 40, 40, &RGB(0, 255, 0));

	//platform.discover();

	loop {
		if platform.connected() {
			if let Some((size, x)) = platform.recieve() {
				let s = String::from_utf8_lossy(&x[0..size]);
				println!("RECIEVED: {}", s);
			}
		}

		if platform.step() {
			break;
		}
	}
}
