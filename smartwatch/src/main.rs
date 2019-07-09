extern crate little;
extern crate little_emu;

mod model;

use little::*;
use little::io::*;
use little::drawing::{RGB, RGBA, Drawing, DrawText, FontBuffer, CharBuffer};
use little::{deg, vec2};

include_buffer!(RAINY, RGBA, "../assets/rainy.rc");
include_buffer!(EHEHE, RGBA, "../assets/ehehe.rc");
include_font!(QUESTRIAL, "../assets/Questrial/Questrial-Regular.rc");

fn main() {
	let mut platform = little_emu::OpenGLPlatform::init();

	let surface = platform.surface();

	let questrial = QUESTRIAL;
	let rainy = RAINY;

	surface.rect(vec2(0,0), vec2(128,128), &RGB(0, 0, 0), 0);
	surface.copy_transform(vec2(20,20), vec2f(1.0, 1.0), vec2(10, 10), deg(180.0), &rainy);

	// platform.discover();

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
