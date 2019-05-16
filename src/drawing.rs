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

//like Into, but for pixels to make type constraints shorter
pub trait ToPixel<T: Pixel>: Pixel {
    fn to_pixel(self) -> T;
}

impl<T: Pixel> ToPixel<T> for T {
    fn to_pixel(self) -> Self {
        self
    }
}

impl Pixel for bool {
    fn hard_blend(&self) -> bool {
        *self
    }

    fn soft_blend(&self) -> f32 {
        if *self {
            1.0
        } else {
            0.0
        }
    }

    fn choose(self, other: Self, t: f32) -> Self {
        if t < 1.0 {
            self
        } else {
            other
        }
    }
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

//pixel conversions ahead

impl ToPixel<bool> for u8 {
    fn to_pixel(self) -> bool {
        self > 126
    }
}

impl ToPixel<RGB> for RGBA {
    fn to_pixel(self) -> RGB {
        RGB(self.0, self.1, self.2)
    }
}

impl ToPixel<u8> for bool {
    fn to_pixel(self) -> u8 {
        if self {
            255u8
        } else {
            0u8
        }
    }
}

impl ToPixel<RGB> for u8 {
    fn to_pixel(self) -> RGB {
        RGB(self, self, self)
    }
}

impl ToPixel<RGB> for bool {
    fn to_pixel(self) -> RGB {
        RGB(self.to_pixel(), self.to_pixel(), self.to_pixel())
    }
}

impl ToPixel<RGBA> for bool {
    fn to_pixel(self) -> RGBA {
        RGBA(self.to_pixel(), self.to_pixel(), self.to_pixel(), self.to_pixel())
    }
}

pub enum Blend {
    Hard,
    Soft,
    Write
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

impl<F: Pixel> StaticBuffer<F> {
    fn get_pos(&self, x: i32, y: i32) -> usize {
        8+(mem::size_of::<F>()*((y*self.width()) as usize + x as usize))
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
        let pos = self.get_pos(x, y);
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

pub const DEFAULT_CHARS: &'static str = "ABCDEFGHIJKLMNOPabcdefghijklmnop.,1234567890 ";

#[derive(Clone, Debug)]
pub struct FontCharHeader {
    pub width: i32, pub height: i32,
    pub left: i32, pub top: i32,
    
    pub x_advance: f32
}

pub trait CharBuffer: Buffer<Format=bool> {
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

impl StaticGlyphBuffer {
    const MAPPING: [u8; 8] = [0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01];

    fn get_pos(&self, x: i32, y: i32) -> (usize, u8) {
        let i = (y * self.width() + x) as usize;
        ((i/8), Self::MAPPING[i%8])
    }
}

impl Buffer for StaticGlyphBuffer {
    type Format = bool;

    fn width(&self) -> i32 {
        self.header.width
    }

    fn height(&self) -> i32 {
        self.header.height
    }

    fn get_pixel(&self, x: i32, y: i32) -> bool {
        let (pos, bit) = self.get_pos(x, y);
        (self.buf[self.pos+pos] & bit).count_ones() == 1
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
                pos += transmute::<u32>(&self.buf[pos..pos+mem::size_of::<u32>()]) as usize
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

#[macro_export]
macro_rules! include_font {
    ($name: ident, $path: tt) => {
        const $name: little::drawing::StaticFontBuffer =
            little::drawing::StaticFontBuffer {
                buf: include_bytes!($path)
            };
    };
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

pub type ColorBlend<'a,'b, P> = (&'a P, &'b Blend);

//todo: closure that evaluates p
fn blend_colors<P: Pixel, TP: ToPixel<P>>((color, blend): ColorBlend<P>, p: TP) -> P {
    match blend {
        Blend::Hard => {
            if p.hard_blend() {
                color.clone()
            } else {
                p.to_pixel()
            }
        },
        Blend::Soft => {
            let t = p.soft_blend();
            color.clone().choose(p.to_pixel(), t)
        },
        Blend::Write => {
            color.clone()
        }
    }
}

pub struct DrawRegion<'a, 'b, T> {
    pub draw: &'a mut T,
    pub region: &'b Region
}

pub struct DrawColor<'a, 'b, 'c, P: Pixel, T> {
    pub draw: &'a mut T,
    pub fill: ColorBlend<'b, 'c, P>
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

impl<'a, 'b, 'c, P: Pixel, TP: ToPixel<P>, T: Buffer<Format=TP>> Buffer for DrawColor<'a, 'b, 'c, P, T> {
    type Format = P;

    fn width(&self) -> i32 {
        self.draw.width()
    }

    fn height(&self) -> i32 {
        self.draw.height()
    }

    fn get_pixel(&self, x: i32, y: i32) -> P {
        blend_colors(self.fill, self.draw.get_pixel(x, y))
    }
}

impl<'a, 'b, 'c, P: ToPixel<TP>, TP: ToPixel<P>, T: Buffer<Format=TP> + WriteBuffer> WriteBuffer for DrawColor<'a, 'b, 'c, P, T> {
    fn set_pixel(&mut self, x: i32, y: i32, p: P) {
        self.draw.set_pixel(x, y, blend_colors(self.fill, p).to_pixel());
    }
}

pub trait DrawingConvert: Sized {
    fn with_region<'a, 'b>(&'a mut self, region: &'b Region) -> DrawRegion<'a, 'b, Self>;
    fn with_color<'a, 'b, 'c, C: Pixel>(&'a mut self, color: ColorBlend<'b, 'c, C>) -> DrawColor<'a, 'b, 'c, C, Self>;
}

pub trait Drawing<P: Pixel, TP: ToPixel<P>> {
    fn blend(&mut self, x: i32, y: i32, cb: ColorBlend<TP>);

    fn line(&mut self, from: &Pos, to: &Pos, color: ColorBlend<TP>, thickness: i32);
    fn rect(&mut self, from: &Pos, to: &Pos, color: ColorBlend<TP>);

    fn flat_top_triangle(&mut self, points: [&Pos; 3], color: ColorBlend<TP>);
    fn flat_bottom_triangle(&mut self, points: [&Pos; 3], color: ColorBlend<TP>);
    fn triangle(&mut self, points: [&Pos; 3], color: ColorBlend<TP>);
    fn poly(&mut self, points: &[&Pos], color: ColorBlend<TP>);
    
    fn copy<B: Buffer<Format=TP>>(&mut self, from: Pos, to: Pos, buf: &B, blend: &Blend);
}

pub trait TextDrawing<P: Pixel, TP: ToPixel<P>, F: FontBuffer>: Drawing<P, TP> {
    fn glyph(&mut self, x: &mut f32, y: f32, font_size: f32, glyph: F::Glyph, blend: ColorBlend<TP>);
    fn text(&mut self, txt: &str, from: &Pos, font_size: f32, font: &F, blend: ColorBlend<TP>);
    fn text_multiline(&mut self, txt: &str, from: &Pos, to: &Pos, font_size: f32, line_height: f32, font: &F, blend: ColorBlend<TP>);
}

impl<S: Buffer + Sized> DrawingConvert for S {
    fn with_region<'a, 'b>(&'a mut self, region: &'b Region) -> DrawRegion<'a, 'b, Self> {
        DrawRegion { region, draw: self }
    }

    fn with_color<'a, 'b, 'c, C: Pixel>(&'a mut self, fill: ColorBlend<'b, 'c, C>) -> DrawColor<'a, 'b, 'c, C, Self> {
        DrawColor { fill, draw: self }
    }
}

impl<S: Buffer + WriteBuffer, TP: ToPixel<S::Format>> Drawing<S::Format, TP> for S {
    fn blend(&mut self, x: i32, y: i32, (color, blend): ColorBlend<TP>) {
        match blend {
            Blend::Hard => {
                if color.hard_blend() {
                    self.set_pixel(x, y, color.clone().to_pixel());
                }
            },
            Blend::Soft => {
                let t = color.soft_blend();
                let px = color.clone().to_pixel().choose(self.get_pixel(x, y), t);
                
                self.set_pixel(x, y, px);
            },
            Blend::Write => {
                self.set_pixel(x, y, color.clone().to_pixel());
            }
        }
    }

    fn line(&mut self, from: &Pos, to: &Pos, color: ColorBlend<TP>, mut thickness: i32) {    
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

    fn rect(&mut self, from: &Pos, to: &Pos, color: ColorBlend<TP>) {
        for y in from.y..to.y {
            for x in from.x..to.x {
                self.blend(x, y, color);
            }
        }
    }

    fn flat_top_triangle(&mut self, points: [&Pos; 3], color: ColorBlend<TP>) {
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
    
    fn flat_bottom_triangle(&mut self, points: [&Pos; 3], color: ColorBlend<TP>) {
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
    
    fn triangle(&mut self, mut points: [&Pos; 3], color: ColorBlend<TP>) {
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

    fn poly(&mut self, points: &[&Pos], color: ColorBlend<TP>) {
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

    fn copy<B: Buffer<Format=TP>>(&mut self, from: Pos, to: Pos, buf: &B, blend: &Blend) {
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
}

impl<P: Pixel, TP: ToPixel<P>, S: Drawing<P, TP>, F: FontBuffer> TextDrawing<P, TP, F> for S where bool: drawing::ToPixel<TP> {
    fn glyph(&mut self, x: &mut f32, y: f32, font_size: f32, glyph: F::Glyph, cb: ColorBlend<TP>) {
        // let head = glyph.get_header();
        // let from = pos(*x as i32 + (head.left as f32*font_size) as i32, y as i32 + (head.top as f32*font_size) as i32);
        // let to = from + pos((head.width as f32*font_size) as i32, (head.height as f32*font_size) as i32);
        
        // self.copy(from, to, &glyph.with_color((cb.0, &Blend::Hard)), cb.1);

        // *x += head.x_advance * font_size;
    }

    fn text(&mut self, txt: &str, from: &Pos, font_size: f32, font: &F, cb: ColorBlend<TP>) {
        let mut iter = txt.chars().peekable();
        
        let mut x = from.x as f32;
        
        while let Some(c) = iter.next() {
            let kern_next = iter.peek();

            let mut glyph: F::Glyph = font.get_char(c).unwrap();
            
            let (from, to) = {
                let head = glyph.get_header();
                
                let from = pos(x as i32 + (head.left as f32*font_size) as i32, from.y + (head.top as f32*font_size) as i32);
                let to = from + pos((head.width as f32*font_size) as i32, (head.height as f32*font_size) as i32);
                
                x += head.x_advance * font_size;

                (from, to)
            };
            
            self.copy(from, to, &glyph.with_color((cb.0, &Blend::Hard)), cb.1);

            x += kern_next.and_then(|c2| font.get_kerning(c, *c2)).unwrap_or(0.0) * font_size;
        }
    }

    fn text_multiline(&mut self, txt: &str, from: &Pos, to: &Pos, font_size: f32, line_height: f32, font: &F, blend: ColorBlend<TP>) {

    }
}