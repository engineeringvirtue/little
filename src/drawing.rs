use super::*;

pub trait Pixel: Clone {
    fn hard_blend(&self) -> bool {
        true
    }

    fn soft_blend(&self) -> f32 {
        1.0
    }

    fn choose(self, other: Self, t: f32) -> Self;
}

impl Pixel for u8 {
    fn hard_blend(&self) -> bool {
        self > &126
    }

    fn soft_blend(&self) -> f32 {
        *self as f32/255.0
    }

    fn choose(self, other: Self, t: f32) -> Self {
        ((self as f32 * t) + (other as f32 * (1.0 - t))) as u8
    }
}

#[derive(Debug, Clone)]
pub struct RGB(pub u8, pub u8, pub u8);

impl Pixel for RGB {
    fn choose(self, other: Self, t: f32) -> Self {
        RGB(self.0.choose(other.0, t), self.1.choose(other.1, t), self.2.choose(other.2, t))
    }
}

#[derive(Debug, Clone)]
pub struct RGBA(pub u8, pub u8, pub u8, pub u8);

impl Pixel for RGBA {
    fn hard_blend(&self) -> bool {
        self.3.hard_blend()
    }

    fn soft_blend(&self) -> f32 {
        self.3.soft_blend()
    }

    fn choose(self, other: Self, t: f32) -> Self {
        RGBA(self.0.choose(other.0, t), self.1.choose(other.1, t), self.2.choose(other.2, t), self.3.choose(other.3, t))
    }
}

impl Into<RGB> for RGBA {
    fn into(self) -> RGB {
        RGB(self.0, self.1, self.2)
    }
}

pub enum Blend {
    Hard,
    Soft,
    Write
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
            
            const WIDTH: i32 = i32::from_le_bytes([Self::BUFFER[0], Self::BUFFER[1], Self::BUFFER[2], Self::BUFFER[3]]);
            const HEIGHT: i32 = i32::from_le_bytes([Self::BUFFER[4], Self::BUFFER[5], Self::BUFFER[6], Self::BUFFER[7]]);
            
            fn get_pos(x: i32, y: i32) -> usize {
                8+(std::mem::size_of::<$format>()*((y*Self::WIDTH) as usize + x as usize))
            }
        }

        impl little::drawing::Buffer<$format> for $name {
            fn width(&self) -> i32 {
                Self::WIDTH
            }

            fn height(&self) -> i32 {
                Self::HEIGHT
            }

            fn get_pixel(&self, x: i32, y: i32) -> $format {
                use std::mem;

                let pos = Self::get_pos(x, y);
                
                let mut buffer: [u8; mem::size_of::<$format>()] = unsafe { mem::uninitialized() };
                buffer.copy_from_slice(&Self::BUFFER[pos..pos + mem::size_of::<$format>()]);

                unsafe {
                    mem::transmute(buffer)
                }
            }

            fn set_pixel(&mut self, x: i32, y: i32, p: $format) {
                ()
            }
        }
    };
}

pub type ColorBlend<'a,'b, P> = (&'a P, &'b Blend);

pub trait Drawing<P: Pixel, OP: Pixel + Into<P>> {
    fn blend(&mut self, x: i32, y: i32, cb: ColorBlend<OP>);

    fn line(&mut self, from: &Pos, to: &Pos, color: ColorBlend<OP>, thickness: i32);
    fn rect(&mut self, from: &Pos, to: &Pos, color: ColorBlend<OP>);

    fn flat_top_triangle(&mut self, points: [&Pos; 3], color: ColorBlend<OP>);
    fn flat_bottom_triangle(&mut self, points: [&Pos; 3], color: ColorBlend<OP>);
    fn triangle(&mut self, points: [&Pos; 3], color: ColorBlend<OP>);
    fn poly(&mut self, points: &[&Pos], color: ColorBlend<OP>);
    
    fn copy<B: Buffer<OP>>(&mut self, from: Pos, to: Pos, buf: &B, blend: &Blend);

    fn with_region<'a, 'b>(&'a mut self, region: &'b Region) -> DrawRegion<'a, 'b, Self> where Self: Sized;
}

pub struct Interpolator<'a> {
    x: i32,
    x2: i32,

    x1: &'a i32,
    y1: &'a i32,

    xlen: f32,
    ylen: f32,
    
    swapped: bool,
    first: bool
}

impl<'a> Interpolator<'a> {
    fn resolve(&self, pos: Pos) -> Pos {
        if self.swapped {
            Pos {x: pos.y, y: pos.x}
        } else {
            pos
        }
    }
}

impl<'a> core::iter::Iterator for Interpolator<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
        if self.first {
            self.first = false;

            Some(self.resolve(pos(self.x, *self.y1)))
        } else if self.x != self.x2 {
            if self.x2 > self.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }

            let i = (self.x - self.x1) as f32 / self.xlen;
            let y = self.y1 + (i * self.ylen) as i32;
            
            Some(self.resolve(pos(self.x, y)))
        } else {
            None
        }
    }
}

pub fn interpolate<'a>(mut x1: &'a i32, mut x2: &'a i32, mut y1: &'a i32, mut y2: &'a i32) -> Interpolator<'a> {
    let mut swapped = false;

    if (y2 - y1).abs() > (x2 - x1).abs() {
        mem::swap(&mut x1, &mut y1);
        mem::swap(&mut x2, &mut y2);
        
        swapped = true;
    }
    
    let xlen = (x2 - x1) as f32;
    let ylen = (y2 - y1) as f32;

    Interpolator {
        x: *x1, x2: *x2,
        xlen, ylen,
        x1, y1,
        swapped,
        first: true
    }
}

impl<P: Pixel, S: Buffer<P>, OP: Pixel + Into<P>> Drawing<P, OP> for S {
    fn blend(&mut self, x: i32, y: i32, (color, blend): ColorBlend<OP>) {
        match blend {
            Blend::Hard => {
                if color.hard_blend() {
                    self.set_pixel(x, y, color.clone().into());
                }
            },
            Blend::Soft => {
                let t = color.soft_blend();
                let px = color.clone().into().choose(self.get_pixel(x, y), t);
                
                self.set_pixel(x, y, px);
            },
            Blend::Write => {
                self.set_pixel(x, y, color.clone().into());
            }
        }
    }

    fn line(&mut self, from: &Pos, to: &Pos, color: ColorBlend<OP>, mut thickness: i32) {    
        let start_thickness = -thickness;
        thickness *= 2;

        for Pos {x,y} in interpolate(&from.x, &to.x, &from.y, &to.y) {
            for t in 0..thickness {
                let offset_thickness = start_thickness + t;
                let y_thick = y + offset_thickness;
                let x_thick = x + offset_thickness;

                if x_thick >= 0 && x_thick < self.width() {
                    self.blend(x_thick, y, color);
                }

                if y_thick >= 0 && y_thick < self.height() {
                    self.blend(x, y_thick, color);
                }
            }
        }
    }

    fn rect(&mut self, from: &Pos, to: &Pos, color: ColorBlend<OP>) {
        for y in from.y..to.y {
            for x in from.x..to.x {
                self.blend(x, y, color);
            }
        }
    }

    fn flat_top_triangle(&mut self, points: [&Pos; 3], color: ColorBlend<OP>) {
        let mut left = interpolate(&points[2].x, &points[0].x, &points[2].y, &points[0].y);
        let mut right = interpolate(&points[2].x, &points[1].x, &points[2].y, &points[1].y);

        let mut next_y = points[2].y - 1;

        loop {
            let x1 = loop {
                if let Some(Pos {x, y}) = left.next() {
                    if y <= next_y {
                        break x;
                    }
                } else {
                    return;
                }
            };

            let x2 = loop {
                if let Some(Pos {x, y}) = right.next() {
                    if y <= next_y {
                        break x;
                    }
                } else {
                    return;
                }
            };

            if x2 > x1 {
                self.rect(&pos(x1, next_y), &pos(x2, next_y+1), color);
            } else {
                self.rect(&pos(x2, next_y), &pos(x1, next_y+1), color);
            }

            next_y -= 1;
        }
    }
    
    fn flat_bottom_triangle(&mut self, points: [&Pos; 3], color: ColorBlend<OP>) {
        let mut left = interpolate(&points[0].x, &points[1].x, &points[0].y, &points[1].y);
        let mut right = interpolate(&points[0].x, &points[2].x, &points[0].y, &points[2].y);

        let mut next_y = points[0].y + 1;

        loop {
            let x1 = loop {
                if let Some(Pos {x, y}) = left.next() {
                    if y >= next_y {
                        break x;
                    }
                } else {
                    return;
                }
            };

            let x2 = loop {
                if let Some(Pos {x, y}) = right.next() {
                    if y >= next_y {
                        break x;
                    }
                } else {
                    return;
                }
            };

            if x2 > x1 {
                self.rect(&pos(x1, next_y-1), &pos(x2, next_y), color);
            } else {
                self.rect(&pos(x2, next_y-1), &pos(x1, next_y), color);
            }

            next_y += 1;
        }
    }
    
    fn triangle(&mut self, mut points: [&Pos; 3], color: ColorBlend<OP>) {
        points.sort_unstable_by(|a, b| a.y.cmp(&b.y));

        if points[0].y == points[1].y {
            self.flat_top_triangle(points, color);
        } else if points[1].y == points[2].y {
            self.flat_bottom_triangle(points, color);
        } else {
            //hard math you can find it here http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
            let mid = pos(points[0].x + (((points[1].y - points[0].y) as f32 / (points[2].y - points[0].y) as f32) * (points[2].x - points[0].x) as f32) as i32, points[1].y);
            
            self.flat_bottom_triangle([points[0], points[1], &mid], color);
            self.flat_top_triangle([points[1], &mid, points[2]], color);
        }
    }

    fn poly(&mut self, points: &[&Pos], color: ColorBlend<OP>) {
        for p1 in 0..points.len() {
            if p1 > 0 {
                let p2 = points[p1 - 1];
                
                if let Some(p3) = points.get(p1+1) {
                    self.triangle([&points[p1], &p2, p3], color);
                } else {
                    self.triangle([&points[p1], &points[0], p2], color);
                }
            } else {
                self.triangle([&points[p1], points.last().unwrap(), &points[p1+1]], color);
            }
        }
    }

    fn copy<B: Buffer<OP>>(&mut self, from: Pos, to: Pos, buf: &B, blend: &Blend) {
        let length = to - from;
        let scale_factor = (buf.width() as f32 / length.x as f32)
            .min(buf.height() as f32 / length.y as f32);
        
        for y in from.y..to.y {
            for x in from.x..to.x {
                let px = buf.get_pixel(((x - from.x) as f32 * scale_factor) as i32,
                    ((y - from.y) as f32 * scale_factor) as i32);
                
                self.blend(x, y, (&px, blend))
            }
        }
    }

    fn with_region<'a, 'b>(&'a mut self, region: &'b Region) -> DrawRegion<'a, 'b, Self> where Self: Sized {
        DrawRegion {
            region, draw: self
        }
    }
}

pub struct DrawRegion<'a, 'b, T> {
    pub draw: &'a mut T,
    pub region: &'b Region
}

impl<'a, 'b, P: Pixel, T: Buffer<P>> Buffer<P> for DrawRegion<'a, 'b, T> {
    fn width(&self) -> i32 {
        self.region.to.x - self.region.from.x
    }

    fn height(&self) -> i32 {
        self.region.to.y - self.region.from.y
    }

    fn get_pixel(&self, x: i32, y: i32) -> P {
        self.draw.get_pixel(self.region.from.x + x, self.region.from.y + y)
    }

    fn set_pixel(&mut self, x: i32, y: i32, p: P) {
        self.draw.set_pixel(self.region.from.x + x, self.region.from.y + y, p)
    }
}