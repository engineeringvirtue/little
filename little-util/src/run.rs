use clap::*;
use error::{Result, ResultExt};

use std::path;
use std::fs;
use std::mem;

/// CLI Runtime
pub fn run() -> Result<i32> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                    .version(env!("CARGO_PKG_VERSION"))
                    .author(env!("CARGO_PKG_AUTHORS"))
                    .about("Utility program for little.")
                      
                    .subcommand(SubCommand::with_name("pack-image")
                        .arg_from_usage("<PATH> 'Relative path to image'"))

                    .subcommand(SubCommand::with_name("pack-font")
                        .arg_from_usage("<PATH> 'Relative path to font'"))

                    .get_matches();

    match matches.subcommand() {
        ("pack-font", Some(matches)) => {
            use freetype::*;
            
            let path = path::Path::new(matches.value_of("PATH").unwrap());
            
            let lib = Library::init()?;
            let face = lib.new_face(path, 0)?;
            // face.set_char_size(char_width: isize, char_height: isize, horz_resolution: u32, vert_resolution: u32)
        },
        ("pack-image", Some(matches)) => {
            println!("Reading...");
            let path = path::Path::new(matches.value_of("PATH").unwrap());
            let png = lodepng::decode32_file(path)?;

            let width: [u8; 4] = unsafe { mem::transmute(png.width as i32) };
            let height: [u8; 4] = unsafe { mem::transmute(png.height as i32) };
            
            const PX_SIZE: usize = mem::size_of::<little::drawing::RGBA>();
            println!("Encoding...");
            let mut img = vec![0; 8 + (png.width*png.height*PX_SIZE)];
            
            for i in 0..4 {
                img[i] = width[i];
            }
            
            for i in 0..4 {
                img[i+4] = height[i];
            }

            let mut i = 8;
            for px in png.buffer {
                let buf: [u8; PX_SIZE] = unsafe { mem::transmute(little::drawing::RGBA(px.r, px.g, px.b, px.a)) };
                
                for bufx in 0..PX_SIZE {
                    img[i + bufx] = buf[bufx];
                }

                i += PX_SIZE;
            }

            let path = path.with_extension("rc");
            fs::write(&path, img).chain_err(|| format!("Error writing to path {}", path.to_str().unwrap()))?;
            println!("Finished!");
        },
        _ => ()
    }

    Ok(0)
}