use super::*;

pub trait Pixel: Clone { }

pub trait Mask: Pixel {
    fn order(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct Greyscale(pub u8);
impl Pixel for Greyscale {}

impl Mask for Greyscale {
    fn order(&self) -> bool {
        self.0 > 126
    }
}

#[derive(Debug, Clone)]
pub struct RGB(pub u8, pub u8, pub u8);
impl Pixel for RGB {}

#[derive(Debug, Clone)]
pub struct RGBA(pub u8, pub u8, pub u8, pub u8);
impl Pixel for RGBA {}

impl Mask for RGBA {
    fn order(&self) -> bool {
        self.3 > 126
    }
}

pub trait Buffer<P: Pixel> {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
 
    fn get_pixel(&self, x: i32, y: i32) -> P;
    fn set_pixel(&mut self, x: i32, y: i32, p: P);
}

#[macro_export]
macro_rules! include_buffer {
    ($name: ident, $format: tt, $path: tt) => {
        struct $name;

        impl $name {
            const BUFFER: &'static [u8] = include_bytes!($path);
            
            const WIDTH: i32 = i32::from_le_bytes($name_BUFFER[0..4]);
            const HEIGHT: i32 = i32::from_le_bytes($name_BUFFER[4..8]);
            
            fn get_pos(x: i32, y: i32) -> usize {
                8+(mem::size_of::<$format>()*(y*Self::WIDTH)*x)
            }
        }

        impl Buffer<$format> for $name {
            fn width(&self) -> i32 {
                Self::WIDTH
            }

            fn height(&self) -> i32 {
                Self::HEIGHT
            }

            fn get_pixel(&self, x: i32, y: i32) -> $format {
                let pos = Self::get_pos(x, y);
                unsafe {
                    mem::transmute_copy(Self::BUFFER[pos..pos + mem::size_of::<$format>()])
                }
            }

            fn set_pixel(&mut self, x: i32, y: i32, p: $format) {
                ()
            }
        }
    };
}

pub trait Drawing<P: Pixel> {
    fn line(&mut self, from: Pos, to: Pos, color: P, thickness: i32);
    fn rect(&mut self, from: Pos, to: Pos, color: P);
    
    fn copy<B: Buffer<P>>(&mut self, from: Pos, to: Pos, buf: &B);
    fn copy_mask<BP: Mask + Into<P>, B: Buffer<BP>>(&mut self, from: Pos, to: Pos, buf: &B);
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
        for y in from.1..to.1 {
            for x in from.0..to.0 {
                self.set_pixel(x, y, color.clone());
            }
        }
    }

    fn copy<B: Buffer<P>>(&mut self, from: Pos, to: Pos, buf: &B) {
        
    }

    fn copy_mask<BP: Mask + Into<P>, B: Buffer<BP>>(&mut self, from: Pos, to: Pos, buf: &B) {
        let length = (to.0 - from.0, to.1 - from.1);
        let scale_factor = (length.0 as f32 / buf.width() as f32)
            .min(length.1 as f32 / buf.height() as f32);
        
        for y in from.1..to.1 {
            for x in from.0..to.0 {
                let px = buf.get_pixel(((x - from.0) as f32 * scale_factor) as i32,
                    ((y - from.1) as f32 * scale_factor) as i32);
                
                if px.order() {
                    self.set_pixel(x, y, px.into());
                }
            }
        }
    }
}