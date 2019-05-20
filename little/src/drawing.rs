use super::*;
use core::{mem, ptr};
use util::transmute;

pub trait Pixel: Clone {
    fn soft(&self) -> bool;

    fn soft_blend(&self) -> f32 {
        1.0
    }

    fn mult(self, t: f32) -> Self;
    fn choose(self, other: Self, t: f32) -> Self;
}

//like Into, but for pixels to make type constraints shorter
pub trait ToPixel<T: Pixel>: Pixel {
    fn to_pixel(self) -> T;
}

impl<T: Pixel> ToPixel<T> for T {
    fn to_pixel(self) -> Self {
        self
    }
}

impl Pixel for u8 {
    fn soft(&self) -> bool {
        true
    }

    fn soft_blend(&self) -> f32 {
        *self as f32/255.0
    }

    fn mult(self, t: f32) -> Self {
        (self as f32 * t) as u8
    }

    fn choose(self, other: Self, t: f32) -> Self {
        ((self as f32 * t) + (other as f32 * (1.0-t))) as u8
    }
}

#[derive(Debug, Clone)]
pub struct RGB(pub u8, pub u8, pub u8);

impl Pixel for RGB {
    fn soft(&self) -> bool {
        false
    }

    fn mult(self, t: f32) -> Self {
        RGB(self.0.mult(t), self.1.mult(t), self.2.mult(t))
    }

    fn choose(self, other: Self, t: f32) -> Self {
        RGB(self.0.choose(other.0, t), self.1.choose(other.1, t), self.2.choose(other.2, t))
    }
}

#[derive(Debug, Clone)]
pub struct RGBA(pub u8, pub u8, pub u8, pub u8);

impl Pixel for RGBA {
    fn soft(&self) -> bool {
        true
    }

    fn soft_blend(&self) -> f32 {
        self.3.soft_blend()
    }

    fn mult(self, t: f32) -> Self {
        RGBA(self.0.mult(t), self.1.mult(t), self.2.mult(t), self.3.mult(t))
    }

    fn choose(self, other: Self, t: f32) -> Self {
        RGBA(self.0.choose(other.0, t), self.1.choose(other.1, t), self.2.choose(other.2, t), self.3.choose(other.3, t))
    }
}

impl ToPixel<RGB> for RGBA {
    fn to_pixel(self) -> RGB {
        RGB(self.0, self.1, self.2)
    }
}

impl ToPixel<RGB> for u8 {
    fn to_pixel(self) -> RGB {
        RGB(self, self, self)
    }
}

impl ToPixel<RGBA> for u8 {
    fn to_pixel(self) -> RGBA {
        RGBA(self, self, self, self)
    }
}

pub trait Buffer {
    type Format: Pixel;

    fn width(&self) -> i32;
    fn height(&self) -> i32;
 
    fn get_pixel(&self, x: i32, y: i32) -> Self::Format;
}

pub trait WriteBuffer: Buffer {
    fn set_pixel(&mut self, x: i32, y: i32, p: Self::Format);
}

pub struct StaticBuffer<Format: Pixel> {
    pub buf: &'static [u8],
    pub format: core::marker::PhantomData<Format>
}

pub fn get_bufferi<B: Buffer>(b: &B, x: i32, y: i32) -> usize {
    ((y*b.width()) + x) as usize
}

impl<F: Pixel> StaticBuffer<F> {
    fn get_vec2(&self, x: i32, y: i32) -> usize {
        8+(mem::size_of::<F>()*get_bufferi(self, x, y))
    }
}

impl<F: Pixel> Buffer for StaticBuffer<F> {
    type Format = F;

    fn width(&self) -> i32 {
        transmute(&self.buf[0..3])
    }

    fn height(&self) -> i32 {
        transmute(&self.buf[4..7])
    }

    fn get_pixel(&self, x: i32, y: i32) -> F {
        let pos = self.get_vec2(x, y);
        transmute(&self.buf[pos..pos + mem::size_of::<F>()])
    }
}

#[macro_export]
macro_rules! include_buffer {
    ($name: ident, $format: tt, $path: tt) => {
        const $name: little::drawing::StaticBuffer<$format> =
            little::drawing::StaticBuffer {
                buf: include_bytes!($path),
                format: core::marker::PhantomData
            };
    };
}

#[macro_export]
macro_rules! create_buffer {
    ($name: ident, $width: tt, $height: tt, $format: tt) => {
        struct $name {
            data: [$format; $width * $height]
        }

        impl $name {
            pub fn new() -> Self {
                $name {data: [0; $width * $height]}
            }
        }

        impl Buffer for $name {
            type Format = $format;

            fn width(&self) -> i32 {
                $width
            }

            fn height(&self) -> i32 {
                $height
            }

            fn get_pixel(&self, x: i32, y: i32) -> Self::Format {
                self.data[get_bufferi(self, x, y)].clone()
            }
        }

        impl WriteBuffer for Buffer {
            fn set_pixel(x: i32, y: i32, p: Self::Format) {
                self.data[get_bufferi(self, x, y)] = p;
            }
        }
    };
}

pub const DEFAULT_CHARS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz:;\"\'./?!@#$%^&*(),-=+1234567890 ";
pub const DEFAULT_LINE_HEIGHT: f32 = 32.0;

#[derive(Clone, Debug)]
pub struct FontCharHeader {
    pub width: i32, pub height: i32,
    pub left: i32, pub top: i32,

    pub x_advance: f32
}

pub trait CharBuffer: Buffer<Format=u8> {
    fn get_header(&self) -> &FontCharHeader;
}

pub trait FontBuffer {
    type Glyph: CharBuffer;

    fn get_char(&self, c: char) -> Option<Self::Glyph>;
    fn get_kerning(&self, c1: char, c2: char) -> Option<f32>;
}

pub type FontCharKernPair = (char, char, f32);

pub struct StaticFontBuffer {
    pub buf: &'static [u8]
}

pub struct StaticGlyphBuffer {
    pub header: FontCharHeader,
    pub buf: &'static [u8],
    pub pos: usize
}

impl Buffer for StaticGlyphBuffer {
    type Format = u8;

    fn width(&self) -> i32 {
        self.header.width
    }

    fn height(&self) -> i32 {
        self.header.height
    }

    fn get_pixel(&self, x: i32, y: i32) -> u8 {
        self.buf[self.pos+get_bufferi(self, x, y)]
    }
}

impl CharBuffer for StaticGlyphBuffer {
    fn get_header(&self) -> &FontCharHeader {
        &self.header
    }
}

impl StaticFontBuffer {
    fn pair_len(&self) -> usize {
        transmute::<u32>(&self.buf[0..mem::size_of::<u32>()]) as usize
    }
}

impl FontBuffer for StaticFontBuffer {
    type Glyph = StaticGlyphBuffer;

    fn get_char(&self, c: char) -> Option<StaticGlyphBuffer> {
        let mut pos = mem::size_of::<u32>() + (self.pair_len() * mem::size_of::<FontCharKernPair>());
        
        while pos < self.buf.len() {
            let c2: char = transmute(&self.buf[pos..pos+mem::size_of::<char>()]);
            pos += mem::size_of::<char>();

            if c == c2 {
                pos += mem::size_of::<u32>();
                let header: FontCharHeader = transmute(&self.buf[pos..pos+mem::size_of::<FontCharHeader>()]);

                return Some(StaticGlyphBuffer {
                    header, buf: self.buf,
                    pos: pos + mem::size_of::<FontCharHeader>()
                });
            } else {
                let buf_size = transmute::<u32>(&self.buf[pos..pos+mem::size_of::<u32>()]) as usize;
                pos += buf_size + mem::size_of::<u32>() + mem::size_of::<FontCharHeader>();
            }
        }

        None
    }

    fn get_kerning(&self, c1: char, c2: char) -> Option<f32> {
        for i in 0..self.pair_len() {
            let pos = i*mem::size_of::<FontCharKernPair>();
            let pair: FontCharKernPair = transmute(&self.buf[pos..pos+mem::size_of::<FontCharKernPair>()]);

            if pair.0 == c1 && pair.1 == c2 {
                return Some(pair.2);
            }
        }

        None
    }
}

pub struct DrawText<'a, F: FontBuffer> {
    pub font_size: f32,
    pub line_height: f32,

    pub font: &'a F,
    pub txt: &'a str
}

impl<'a, F: FontBuffer> DrawText<'a, F> {
    pub fn new(font: &'a F, txt: &'a str) -> Self {
        DrawText {
            font_size: 1.0, line_height: 1.0,
            font, txt
        }
    }

    pub fn font_size(self, font_size: f32) -> Self {
        DrawText {font_size, ..self}
    }
    
    pub fn line_height(self, line_height: f32) -> Self {
        DrawText {line_height, ..self}
    }
}

#[macro_export]
macro_rules! include_font {
    ($name: ident, $path: tt) => {
        const $name: little::drawing::StaticFontBuffer =
            little::drawing::StaticFontBuffer {
                buf: include_bytes!($path)
            };
    };
}

pub struct Interpolator {
    x: f32,
    x1: f32,
    x2: f32,
    y1: f32,

    ylen: f32,
    xlen: f32,

    swapped: bool
}

impl Interpolator {
    pub fn line(mut x1: f32, mut x2: f32, mut y1: f32, mut y2: f32) -> Self {
        let mut swapped = false;

        if abs(y2 - y1) > abs(x2 - x1) {
            mem::swap(&mut x1, &mut y1);
            mem::swap(&mut x2, &mut y2);
            
            swapped = true;
        }

        Interpolator {
            x: x1,
            x1, x2,
            y1,
            
            xlen: x2 - x1,
            ylen: y2 - y1,
            
            swapped
        }
    }

    pub fn resolve(&self, x: f32, y: f32) -> (f32, f32) {
        if self.swapped {
            (y, x)
        } else {
            (x, y)
        }
    }

    pub fn get_co(&self) -> f32 {
        let i = (self.x-self.x1)/self.xlen;
        self.y1 + (i*self.ylen)
    }

    pub fn get_y_swap(&self) -> Option<f32> {
        if self.swapped {
            Some(self.x)
        } else {
            None
        }
    }

    pub fn get_x_swap(&self) -> Option<f32> {
        if self.swapped {
            None
        } else {
            Some(self.x)
        }
    }
}

impl core::iter::Iterator for Interpolator {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<(f32, f32)> {
        let x = self.resolve(self.x, self.get_co());

        if self.x != self.x2 {
            if self.x2 > self.x {
                self.x += 1.0;
            } else {
                self.x -= 1.0;
            }
            
            Some(x)
        } else {
            None
        }
    }
}

fn blend_multiply<P: Pixel, SP: Pixel, Source: FnOnce() -> SP>(color: P, p: Source) -> P {
    let t = p().soft_blend();
    color.mult(t)
}

pub struct DrawRegion<'a, 'b, T> {
    pub draw: &'a mut T,
    pub region: &'b Region
}

pub struct DrawColor<'a, P: Pixel, T> {
    pub draw: &'a mut T,
    pub fill: &'a P
}

impl<'a, 'b, P: Pixel, T: Buffer<Format=P>> Buffer for DrawRegion<'a, 'b, T> {
    type Format = P;

    fn width(&self) -> i32 {
        self.region.to.x - self.region.from.x
    }

    fn height(&self) -> i32 {
        self.region.to.y - self.region.from.y
    }

    fn get_pixel(&self, x: i32, y: i32) -> P {
        self.draw.get_pixel(self.region.from.x + x, self.region.from.y + y)
    }
}

impl<'a, 'b, P: Pixel, T: Buffer<Format=P> + WriteBuffer> WriteBuffer for DrawRegion<'a, 'b, T> {
    fn set_pixel(&mut self, x: i32, y: i32, p: P) {
        self.draw.set_pixel(self.region.from.x + x, self.region.from.y + y, p)
    }
}

impl<'a, 'b, 'c, P: Pixel, SP: Pixel, T: Buffer<Format=SP>> Buffer for DrawColor<'a, P, T> {
    type Format = P;

    fn width(&self) -> i32 {
        self.draw.width()
    }

    fn height(&self) -> i32 {
        self.draw.height()
    }

    fn get_pixel(&self, x: i32, y: i32) -> P {
        blend_multiply(self.fill.clone(), || self.draw.get_pixel(x, y))
    }
}

impl<'a, 'b, 'c, P: ToPixel<SP>, SP: Pixel, T: Buffer<Format=SP> + WriteBuffer> WriteBuffer for DrawColor<'a, P, T> {
    fn set_pixel(&mut self, x: i32, y: i32, p: P) {
        self.draw.set_pixel(x, y, blend_multiply(self.fill.clone(), move || p).to_pixel());
    }
}

pub trait DrawingConvert: Sized {
    fn with_region<'a, 'b>(&'a mut self, region: &'b Region) -> DrawRegion<'a, 'b, Self>;
    fn with_color<'a, 'b, 'c, C: Pixel>(&'a mut self, fill: &'a C) -> DrawColor<'a, C, Self>;
}

pub trait Drawing<P: Pixel, TP: ToPixel<P>> {
    fn blend(&mut self, x: i32, y: i32, color: TP);
    fn antialiased_blend(&mut self, x: f32, y: f32, color: TP);

    fn line(&mut self, from: &Vector2, to: &Vector2, color: &TP, thickness: i32);
    fn rect(&mut self, from: &Vector2, to: &Vector2, color: &TP);
    fn ellipse(&mut self, from: &Vector2, to: &Vector2, color: &TP);
    
    fn triangle(&mut self, points: [&Vector2; 3], color: &TP);
    fn poly(&mut self, points: &[&Vector2], color: &TP);
    
    fn copy<B: Buffer<Format=TP>>(&mut self, from: Vector2, to: Vector2, buf: &B);
    fn text<F: FontBuffer>(&mut self, txt: &DrawText<F>, from: &Vector2, to: &Vector2, color: &TP) where u8: ToPixel<TP>;
}

impl<S: Buffer + Sized> DrawingConvert for S {
    fn with_region<'a, 'b>(&'a mut self, region: &'b Region) -> DrawRegion<'a, 'b, Self> {
        DrawRegion { region, draw: self }
    }

    fn with_color<'a, 'b, 'c, C: Pixel>(&'a mut self, fill: &'a C) -> DrawColor<'a, C, Self> {
        DrawColor { fill, draw: self }
    }
}

impl<S: Buffer + WriteBuffer, TP: ToPixel<S::Format>> Drawing<S::Format, TP> for S {
    fn blend(&mut self, x: i32, y: i32, color: TP) {
        if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
            return;
        }

        if color.soft() {
            let t = color.soft_blend();
            let px = color.to_pixel().choose(self.get_pixel(x, y), t);
            
            self.set_pixel(x, y, px);
        } else {
            self.set_pixel(x, y, color.to_pixel());
        }
    }

    fn antialiased_blend(&mut self, x: f32, y: f32, color: TP) { unsafe {
        let (xf, yf) = (x%1.0, y%1.0);   
        //set floor coordinate
        self.blend((x-xf) as i32, (y-yf) as i32, color.clone().mult(1.0-xf-yf));
        //set ceil coordinate
        self.blend(ceil(x) as i32, ceil(y) as i32, color.mult((xf+yf)/2.0));
    } }

    fn line(&mut self, from: &Vector2, to: &Vector2, color: &TP, thickness: i32) {    
        let start_thickness = (-thickness/2) as f32;

        for (x,y) in Interpolator::line(from.x as f32, to.x as f32, from.y as f32, to.y as f32) {
            for t in 0..thickness {
                let offset_thickness = start_thickness + t as f32;
                let y_thick = y + offset_thickness;
                let x_thick = x + offset_thickness;

                self.antialiased_blend(x_thick, y, color.clone());
                self.antialiased_blend(x, y_thick, color.clone());
            }
        }
    }

    fn rect(&mut self, from: &Vector2, to: &Vector2, color: &TP) {
        for y in from.y..to.y {
            for x in from.x..to.x {
                self.blend(x, y, color.clone());
            }
        }
    }

    fn ellipse(&mut self, from: &Vector2, to: &Vector2, color: &TP) {
        let (h, w) = (to.y - from.y, to.x - from.x);

        for y in 0..h {
            for x in 0..w {
                if (x*x*h*h) + (y*y*w*w) <= h*h*w*w {
                    self.blend(from.x + x, from.y + y, color.clone());
                }
            }
        }
    }
    
    fn triangle(&mut self, mut points: [&Vector2; 3], color: &TP) {
        points.sort_unstable_by(|a, b| a.y.cmp(&b.y));

        fn flat_triangle<B: Buffer + WriteBuffer, TP: ToPixel<B::Format>>(b: &mut B, ult: &Vector2, left: &Vector2, right: &Vector2, color: &TP, top: bool) {
            //ignore the casts :^)
            let step = if top { -1.0 } else { 1.0 };
            let (mut left, mut right) =
                (Interpolator::line(ult.x as f32, left.x as f32, ult.y as f32, left.y as f32 + step),
                Interpolator::line(ult.x as f32, right.x as f32, ult.y as f32, right.y as f32 + step));

            let mut next_y = ult.y as f32;

            loop {
                if !top {
                    next_y += step;
                }

                fn compare_y(y: f32, next_y: f32, top: bool) -> bool {
                    (y < next_y && top) || (y > next_y && !top)
                }

                fn check_line_x(interp: &mut Interpolator, next_y: f32, top: bool) -> Option<f32> {
                    if let Some(y) = interp.get_y_swap() {
                        if compare_y(y, next_y, top) {
                            return Some(interp.get_co());
                        }
                    }
                    
                    loop {
                        if let Some((x, y)) = interp.next() {
                            if compare_y(y, next_y, top) {
                                break Some(x);
                            }
                        } else {
                            break None;
                        }
                    }
                }

                let (mut x1, mut x2) =
                    match (check_line_x(&mut left, next_y, top), check_line_x(&mut right, next_y, top)) {
                        (Some(x1), Some(x2)) => (x1 as i32, x2 as i32),
                        _ => return
                    };

                if x2 < x1 {
                    mem::swap(&mut x1, &mut x2);
                }

                if (x2-x1) > 1 {
                    b.antialiased_blend(x1 as f32 + 0.9, next_y, color.clone());
                }
                
                b.antialiased_blend(x2 as f32 + 0.4, next_y, color.clone());

                for x in x1+2..x2 {
                    b.blend(x, next_y as i32, color.clone());
                }

                if top {
                    next_y += step;
                }
            }
        }

        if points[0].y == points[1].y {
            flat_triangle(self, points[2], points[0], points[1], color, true);
        } else if points[1].y == points[2].y {
            flat_triangle(self, points[0], points[1], points[2], color, false);
        } else {
            //hard math you can find it here http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
            let mid = vec2(points[0].x + (((points[1].y - points[0].y) as f32 / (points[2].y - points[0].y) as f32) * (points[2].x - points[0].x) as f32) as i32, points[1].y);
            
            flat_triangle(self, points[0], &mid, points[1], color, false);
            flat_triangle(self, points[2], &mid, points[1], color, true);
        }
    }

    fn poly(&mut self, points: &[&Vector2], color: &TP) {
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

    fn copy<B: Buffer<Format=TP>>(&mut self, from: Vector2, to: Vector2, buf: &B) {
        let length = to - from;

        let scale_x = buf.width() as f32 / length.x as f32;
        let scale_y = buf.height() as f32 / length.y as f32;
        
        for y in from.y..to.y {
            for x in from.x..to.x {
                let px = buf.get_pixel(((x - from.x) as f32 * scale_x) as i32,
                    ((y - from.y) as f32 * scale_y) as i32);
                
                self.blend(x, y, px)
            }
        }
    }
    
    fn text<F: FontBuffer>(&mut self, txt: &DrawText<F>, from: &Vector2, to: &Vector2, color: &TP) where u8: ToPixel<TP> {
        let mut iter = txt.txt.chars().peekable();
        
        let mut x = from.x as f32;
        let mut y = from.y as f32;
        
        while let Some(c) = iter.next() {
            if c == '\n' || x as i32 > to.x {
                x = from.x as f32;
                y += txt.line_height * txt.font_size * DEFAULT_LINE_HEIGHT;

                continue;
            }

            if y as i32 > to.y {
                return;
            }

            let kern_next = iter.peek();

            if let Some(mut glyph) = txt.font.get_char(c) { //warning: this will skip over chars that are not included in the font
                let (from, to) = {
                    let head = glyph.get_header();
                    
                    let from = vec2((x + (head.left as f32*txt.font_size)) as i32, (y - ((head.height - head.top) as f32*txt.font_size)) as i32);
                    let to = vec2(from.x + (head.width as f32*txt.font_size) as i32, y as i32);
                    
                    x += head.x_advance * txt.font_size;

                    (from, to)
                };
                
                self.copy(from, to, &glyph.with_color(color));

                x += kern_next.and_then(|c2| txt.font.get_kerning(c, *c2)).unwrap_or(0.0) * txt.font_size;
            }
        }
    }
}
