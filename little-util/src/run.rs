use clap::*;
use error::{Result, ResultExt};

use std::path;
use std::fs;
use std::io::{self, Write};
use std::mem;

use little::drawing::*;

fn packfile(path: &path::Path, buf: Vec<u8>) -> Result<()> {
    let path = path.with_extension("rc");
    fs::write(&path, buf).chain_err(|| format!("Error writing to path {}", path.to_str().unwrap()))?;

    Ok(())
}

/// CLI Runtime
pub fn run() -> Result<i32> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                    .version(env!("CARGO_PKG_VERSION"))
                    .author(env!("CARGO_PKG_AUTHORS"))
                    .about("Utility program for little.")
                      
                    .subcommand(SubCommand::with_name("pack-font")
                        .arg_from_usage("<PATH> 'Relative path to font'")
                        .arg_from_usage("-h --height [HEIGHT] 'Height of glyphs'")
                        .arg_from_usage("-c --char [CHAR] 'Extra characters'")) //TODO

                    .subcommand(SubCommand::with_name("pack-image")
                        .arg_from_usage("<PATH> 'Relative path to image'")
                        .arg_from_usage("--rgb 'Skip alpha channel'"))

                    .get_matches();

    match matches.subcommand() {
        ("pack-font", Some(matches)) => {
            use freetype::*;
            println!("Loading font...");
            
            let path = path::Path::new(matches.value_of("PATH").unwrap());
            let height = matches.value_of("HEIGHT")
                .and_then(|x| x.parse().ok()).unwrap_or(48);
            
            let lib = Library::init()?;
            let face = lib.new_face(path, 0)?;
            face.set_pixel_sizes(0, height)?;

            let mut buf = Vec::new();

            let mut pairs = Vec::new();
            for c in DEFAULT_CHARS.chars() {
                println!("Rendering characters... {}", &c);
                face.load_char(c as usize, face::LoadFlag::RENDER | face::LoadFlag::MONOCHROME)?;

                let glyph = face.glyph();
                let bmp = glyph.bitmap();


                for c2 in DEFAULT_CHARS.chars() {
                    let k = face.get_kerning(c as u32, c2 as u32, face::KerningMode::KerningDefault)?;

                    if k.x != 0 {
                        pairs.push((c, c2, k.x as f32/64.0));
                    }
                }

                let header = FontCharHeader {
                    height: bmp.rows(),
                    width: bmp.width(),
                    left: glyph.bitmap_left(),
                    top: bmp.rows() - glyph.bitmap_top(),

                    pitch: bmp.pitch().abs(),

                    x_advance: glyph.advance().x as f32 / 64.0
                };

                unsafe {
                    buf.extend_from_slice(&mem::transmute::<char, [u8; 4]>(c));
                    buf.extend_from_slice(&mem::transmute::<u32, [u8; 4]>((bmp.buffer().len()) as u32));

                    let header_bytes: [u8; mem::size_of::<FontCharHeader>()] = mem::transmute(header);
                    buf.extend_from_slice(&header_bytes);

                    buf.extend_from_slice(bmp.buffer());
                }
            }

            let mut final_buf = Vec::new();

            unsafe {
                let pair_l: [u8; 4] = mem::transmute(pairs.len() as u32);
                final_buf.extend_from_slice(&pair_l);

                for pair in pairs {
                    let pair_bytes: [u8; mem::size_of::<FontCharKernPair>()] = mem::transmute(pair);
                    final_buf.extend_from_slice(&pair_bytes);
                }
            }

            final_buf.append(&mut buf);

            packfile(path, final_buf)?;
            println!("Finished!");
        },
        ("pack-image", Some(matches)) => {
            println!("Reading...");
            let path = path::Path::new(matches.value_of("PATH").unwrap());
            let rgb = matches.is_present("--rgb");
            let png = lodepng::decode32_file(path)?;
            
            println!("Encoding...");
            let mut img = Vec::new();

            unsafe {
                let width: [u8; 4] = mem::transmute(png.width as i32);
                let height: [u8; 4] = mem::transmute(png.height as i32);

                img.extend_from_slice(&width);
                img.extend_from_slice(&height);

                for px in png.buffer {
                    if rgb {
                        let buf: [u8; mem::size_of::<RGB>()] = mem::transmute(RGB(px.r, px.g, px.b));
                        img.extend_from_slice(&buf);
                    } else {
                        let buf: [u8; mem::size_of::<RGBA>()] = mem::transmute(RGBA(px.r, px.g, px.b, px.a));
                        img.extend_from_slice(&buf);
                    }
                }
            }

            packfile(path, img)?;
            println!("Finished!");
        },
        _ => {
            writeln!(io::stderr(), "Subcommand not supplied! See usage:")?;
            println!("{}", matches.usage());
        }
    }

    Ok(0)
}