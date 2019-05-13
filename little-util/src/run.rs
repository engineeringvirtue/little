use clap::*;
use error::{Result, ResultExt};
use std::io::{self, Write};

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

                    .get_matches();

    match matches.subcommand() {
        ("pack-image", Some(matches)) => {
            let path = path::Path::new(matches.value_of("PATH").unwrap());
            let png = lodepng::decode32_file(path)?;

            let width: [u8; 4] = unsafe { mem::transmute(png.width as i32) };
            let height: [u8; 4] = unsafe { mem::transmute(png.height as i32) };
            
            const PX_SIZE: usize = mem::size_of::<little::drawing::RGBA>();
            let mut img = vec![0; 8 + (png.width*png.height*PX_SIZE)];
            
            for i in 0..3 {
                img[i] = width[i];
            }
            
            for i in 0..3 {
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
        },
        _ => ()
    }

    Ok(0)
}