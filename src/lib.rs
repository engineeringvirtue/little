#![no_std]

pub trait Pixel: Clone { }

#[derive(Debug, Clone)]
pub struct Greyscale(i16);
impl Pixel for Greyscale {}

#[derive(Debug, Clone)]
pub struct RGB(i16, i16, i16);
impl Pixel for RGB {}

pub trait Buffer<P: Pixel> {
    fn width() -> i32;
    fn height() -> i32;
 
    fn get_pixel(&self, x: i32, y: i32) -> &P;
    fn set_pixel(&mut self, x: i32, y: i32, p: P);
}

pub type Pos = (i32, i32);

pub trait Drawing<P: Pixel> {
    fn line(&mut self, from: Pos, to: Pos, color: P, thickness: i32);
    fn rect(&mut self, from: Pos, to: Pos, color: P);
    
    fn copy<B: Buffer<P>>(&mut self, from: Pos, to: Pos, buf: &B);
}

impl<P: Pixel, S: Buffer<P>> Drawing<P> for S {
    fn line(&mut self, from: Pos, to: Pos, color: P, thickness: i32) {
        //get precision and step for each dimension        
        let (len, xstep, ystep) = {
            let xlen = (to.0 - from.0).abs();
            let ylen = (to.1 - from.1).abs();
        

            if xlen > ylen {
                (xlen, 1.0, (ylen as f32 / xlen as f32))
            } else {
                (ylen, (xlen as f32 / ylen as f32), 1.0)
            }
        };

        //draw
        let mut x = from.0 as f32;
        let mut y = from.1 as f32;

        let start_thickness = -(thickness / 2);

        for _ in 0..len {
            for t in 0..thickness {
                let offset_thickness = start_thickness + t;
                self.set_pixel(x as i32 + offset_thickness, y as i32 + offset_thickness, color.clone());
            }

            x += xstep;
            y += ystep;
        }
    }

    fn rect(&mut self, from: Pos, to: Pos, color: P) {
        for y in to.0..to.1 {
            for x in from.0..from.1 {
                self.set_pixel(x, y, color.clone());
            }
        }
    }

    fn copy<B: Buffer<P>>(&mut self, from: Pos, to: Pos, buf: &B) {
        
    }
}