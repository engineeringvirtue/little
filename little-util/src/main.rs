#[macro_use]
extern crate error_chain;
extern crate clap;

extern crate freetype;
extern crate little;

extern crate lodepng;

mod error;
mod run;

use std::io::{self, Write};
use std::process;

/// CLI Entry Point
fn main() {
    match run::run() {
        Ok(i) => process::exit(i),
        Err(e) => {
            writeln!(io::stderr(), "{}", e).expect("Unable to write to stderr!");
            process::exit(1)
        }
    }
}