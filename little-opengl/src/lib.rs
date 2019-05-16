#![feature(const_int_conversion)]

extern crate glutin;
extern crate gl;

extern crate little;

use glutin::*;
use gl::*;
use gl::types::*;

use std::ffi::{CString, CStr, c_void};
use std::ptr::null_mut;
use std::mem;

use little::*;
use drawing::{*, RGB};

fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

unsafe fn gl_err() -> Result<(), GLuint> {
    match GetError() {
        NO_ERROR => Ok(()),
        x => Err(x)
    }
}

extern "system" fn log(_source: GLenum, _etype: GLenum, _id: GLuint, _severity: GLenum, _msg_length: GLsizei, err: *const GLchar, _udata: *mut std::ffi::c_void) {
    unsafe { println!("Debug: {}", CStr::from_ptr(err).to_str().unwrap()); }
}

unsafe fn shader_err(shad: GLuint) -> Result<(), GLint> {
    let compile_log = CString::from_vec_unchecked(vec![0; 512]);
    GetShaderInfoLog(shad, 512, null_mut(), compile_log.as_ptr() as *mut GLchar);

    println!("Shader log: {}", compile_log.to_str().unwrap());

    let mut status = mem::uninitialized();
    GetShaderiv(shad, COMPILE_STATUS, &mut status);

    if status != 1 {
        Err(status)
    } else {
        Ok(())
    }
}

const WIDTH: i32 = 128;
const HEIGHT: i32 = 128;

pub struct TextureSurface {
    pixels: [u8; 128*128*3]
}

impl TextureSurface {
    fn new() -> Self {
        TextureSurface {
            pixels: [0; (WIDTH*HEIGHT*3) as usize]
        }
    }
}

impl Buffer for TextureSurface {
    type Format = RGB;

    fn width(&self) -> i32 {
        WIDTH
    }

    fn height(&self) -> i32 {
        WIDTH
    }

    fn get_pixel(&self, x: i32, y: i32) -> RGB {
        let pos = ((y * WIDTH * 3) + (x * 3)) as usize;
        RGB(self.pixels[pos], self.pixels[pos + 1], self.pixels[pos + 2])
    }
}

impl WriteBuffer for TextureSurface {
    fn set_pixel(&mut self, x: i32, y: i32, color: RGB) {
        let pos = ((y * WIDTH * 3) + (x * 3)) as usize;
        
        self.pixels[pos] = color.0;
        self.pixels[pos+1] = color.1;
        self.pixels[pos+2] = color.2;
    }
}

pub struct OpenGLPlatform {
    surface: TextureSurface,

    glutin_ctx: WindowedContext<PossiblyCurrent>,
    event_loop: EventsLoop,

    program: GLuint, tex_attrib: GLint,
    frag_s: GLuint, vert_s: GLuint,

    vbo: GLuint, vao: GLuint
}

const VERTS: [f32; 24] = [
    -1.0, 1.0, 0.0, 0.0,
    1.0, 1.0, 1.0, 0.0,
    1.0, -1.0, 1.0, 1.0,

    1.0, -1.0, 1.0, 1.0,
    -1.0, -1.0, 0.0, 1.0,
    -1.0, 1.0, 0.0, 0.0
];

impl Platform<TextureSurface> for OpenGLPlatform {
    fn init() -> Self {
        let scale_factor = 4.0;

        let ev = EventsLoop::new();
        let win = WindowBuilder::new()
            .with_title("little \"emulator\"")
            .with_resizable(false)
            .with_dimensions(dpi::LogicalSize::new(WIDTH as f64*scale_factor, HEIGHT as f64*scale_factor));
        
        unsafe {
            let context = ContextBuilder::new()
                .build_windowed(win, &ev).unwrap();
            let context = context.make_current().unwrap();

            load_with(|x| context.get_proc_address(x) as *const c_void);

            Enable(DEBUG_OUTPUT);
            DebugMessageCallback(log, null_mut());

            let vert = "#version 150 core

in vec2 position;
in vec2 tex_coord;

out vec2 texCoord;

void main()
{
    gl_Position = vec4(position, 0.0, 1.0);
    texCoord = tex_coord;
}";

            let frag = "#version 150 core

uniform sampler2D tex;
in vec2 texCoord;

out vec4 outColor;

void main()
{
    outColor = texture(tex, texCoord);
}";

            let vert_size = mem::size_of::<f32>() as i32;

            let mut vbo = mem::uninitialized();
            GenBuffers(1, &mut vbo);
            BindBuffer(ARRAY_BUFFER, vbo);
            BufferData(ARRAY_BUFFER, vert_size as isize * VERTS.len() as isize, VERTS.as_ptr() as *const c_void, STATIC_DRAW);

            let mut vao = mem::uninitialized();    
            GenVertexArrays(1, &mut vao);
            BindVertexArray(vao);

            let vert_s = CreateShader(VERTEX_SHADER);
            ShaderSource(vert_s, 1, &cstr(vert).as_ptr(), null_mut());
            CompileShader(vert_s);
            shader_err(vert_s).unwrap();

            let frag_s = CreateShader(FRAGMENT_SHADER);
            ShaderSource(frag_s, 1, &cstr(frag).as_ptr(), null_mut());
            CompileShader(frag_s);
            shader_err(frag_s).unwrap();

            let program = CreateProgram();
            AttachShader(program, vert_s);
            AttachShader(program, frag_s);

            BindFragDataLocation(program, 0, cstr("outColor").as_ptr());

            LinkProgram(program);
            UseProgram(program);

            let tex_attrib = GetUniformLocation(program, cstr("tex").as_ptr());
            
            {
                let pos = GetAttribLocation(program, cstr("position").as_ptr());
                VertexAttribPointer(pos as _, 2, FLOAT, FALSE, vert_size*4, 0 as _);
                EnableVertexAttribArray(pos as _);

                let tex_coord = GetAttribLocation(program, cstr("tex_coord").as_ptr());
                VertexAttribPointer(tex_coord as _, 2, FLOAT, FALSE, vert_size*4, (vert_size*2) as _);
                EnableVertexAttribArray(tex_coord as _);
            }

            gl_err().unwrap();

            let mut tex = mem::uninitialized();
            GenTextures(1, &mut tex);
            BindTexture(TEXTURE_2D, tex);

            TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as _);
            TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as _);

            OpenGLPlatform {
                surface: TextureSurface::new(),
                
                glutin_ctx: context, event_loop: ev,
                program, tex_attrib, frag_s, vert_s, vbo, vao
            }
        }
    }

    fn surface(&mut self) -> &mut TextureSurface {
        &mut self.surface
    }

    fn step(&mut self) -> bool {
        unsafe {
            TexImage2D(TEXTURE_2D, 0, gl::RGB8 as _, self.surface.width(), self.surface.height(), 0, gl::RGB, UNSIGNED_BYTE, self.surface.pixels.as_ptr() as *const c_void);
            Uniform1i(self.tex_attrib, 0);

            DrawArrays(TRIANGLES, 0, VERTS.len() as i32 / 2);
            self.glutin_ctx.swap_buffers().unwrap();
            
            gl_err().unwrap();
        }
    
        let mut exit = false;
        self.event_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                    exit = true;
                },
                _ => ()
            }
        });

        if exit {
            return true;
        }

        //tbh i dunno how to do main loops but i dont wanna constantly use the cpu
        std::thread::sleep(std::time::Duration::from_millis(50));
        false
    }

    fn stop(self) {
        unsafe {
            DeleteProgram(self.program);
            DeleteShader(self.frag_s);
            DeleteShader(self.vert_s);

            DeleteBuffers(1, &self.vbo);
            DeleteVertexArrays(1, &self.vao);
            gl_err().unwrap();
        }
    }
}