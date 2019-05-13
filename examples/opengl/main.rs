extern crate glutin;
extern crate gl;

extern crate little_renderer;

use glutin::*;
use gl::*;

use std::ffi::{CString, CStr, c_void};
use std::ptr::null_mut;
use std::mem;

use little_renderer::*;
use little_renderer::RGB;

fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

unsafe fn gl_err() -> Result<(), types::GLuint> {
    match GetError() {
        NO_ERROR => Ok(()),
        x => Err(x)
    }
}

extern "system" fn log(_source: types::GLenum, _etype: gl::types::GLenum, _id: gl::types::GLuint, _severity: gl::types::GLenum, _msg_length: gl::types::GLsizei, err: *const gl::types::GLchar, _udata: *mut std::ffi::c_void) {
    unsafe { println!("Debug: {}", CStr::from_ptr(err).to_str().unwrap()); }
}

unsafe fn shader_err(shad: types::GLuint) -> Result<(), types::GLint> {
    let compile_log = CString::from_vec_unchecked(vec![0; 512]);
    GetShaderInfoLog(shad, 512, null_mut(), compile_log.as_ptr() as *mut types::GLchar);

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

struct TextureSurface {
    pixels: [i8; 128*128*3]
}

impl TextureSurface {
    fn new() -> Self {
        TextureSurface {
            pixels: [0; 128*128*3]
        }
    }
}

impl Buffer<RGB> for TextureSurface {
    fn width(&self) -> i32 {
        WIDTH
    }

    fn height(&self) -> i32 {
        WIDTH
    }

    fn get_pixel(&self, x: i32, y: i32) -> RGB {
        let pos = ((y * 128 * 3) + (x * 3)) as usize;
        RGB(self.pixels[pos], self.pixels[pos + 1], self.pixels[pos + 2])
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: RGB) {
        let pos = ((y * 128 * 3) + (x * 3)) as usize;
        
        self.pixels[pos] = color.0;
        self.pixels[pos+1] = color.1;
        self.pixels[pos+2] = color.2;
    }
}

fn main() { unsafe {
    let scale_factor = 4.0;

    let mut ev = EventsLoop::new();
    let win = WindowBuilder::new()
        .with_title("little \"emulator\"")
        // .with_resizable(false)
        .with_dimensions(dpi::LogicalSize::new(WIDTH as f64*scale_factor, HEIGHT as f64*scale_factor));
    
    let context = ContextBuilder::new()
        .build_windowed(win, &ev).unwrap();
    let context = context.make_current().unwrap();

    load_with(|x| context.get_proc_address(x) as *const c_void);

    Enable(DEBUG_OUTPUT);
    DebugMessageCallback(log, null_mut());

    let vert = "#version 150 core

in vec2 position;
in vec2 tex_coord;

void main()
{
    gl_Position = vec4(position, 0.0, 1.0);
}";

    let frag = "#version 150 core

out vec4 outColor;

void main()
{
    outColor = vec4(0.0, 1.0, 0.0, 1.0);
}";

    let verticies = [
        -1.0, 1.0, //0.0, 1.0,
        1.0, 1.0, //1.0, 1.0,
        1.0, -1.0, //1.0, 0.0

        1.0, -1.0, //1.0, 0.0,
        -1.0, -1.0, //0.0, 0.0
        -1.0, 1.0, //0.0, 1.0
    ];

    let vert_size = mem::size_of::<f32>() as isize;

    let mut vbo = mem::uninitialized();
    GenBuffers(1, &mut vbo);
    BindBuffer(ARRAY_BUFFER, vbo);
    BufferData(ARRAY_BUFFER, vert_size * verticies.len() as isize, verticies.as_ptr() as *const c_void, STATIC_DRAW);

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

    let prog = CreateProgram();
    AttachShader(prog, vert_s);
    AttachShader(prog, frag_s);

    BindFragDataLocation(prog, 0, cstr("outColor").as_ptr());

    LinkProgram(prog);
    UseProgram(prog);

    // let tex_attrib = GetUniformLocation(prog, cstr("tex").as_ptr());
    
    {
        let pos = GetAttribLocation(prog, cstr("position").as_ptr());
        VertexAttribPointer(pos as _, 2, FLOAT, FALSE, 0, 0 as _);
        EnableVertexAttribArray(pos as _);

    //     let tex_coord = GetAttribLocation(prog, cstr("tex_coord").as_ptr());
    //     EnableVertexAttribArray(tex_coord as _);
    //     VertexAttribPointer(tex_coord as _, 2, FLOAT, FALSE, vert_size*2, &(vert_size*2) as *const _ as *const c_void);
    //     println!("{}", tex_coord);
    }

    BindVertexArray(0);

    gl_err().unwrap();
    PolygonMode(FRONT_AND_BACK, LINE);

    loop {
        // let mut surface = TextureSurface::new();
        // surface.rect((0, 0), (128, 128), RGB(127, 0, 0));

        ClearColor(0.5, 0.5, 0.5, 1.0);
        Clear(COLOR_BUFFER_BIT);

        BindVertexArray(vao);
        BindBuffer(ARRAY_BUFFER, vbo);

        // let mut tex = mem::uninitialized();
        // GenTextures(1, &mut tex);
        // BindTexture(TEXTURE_2D, tex);

        // TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as _);
        // TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as _);

        // TexImage2D(TEXTURE_2D, 0, RGB8 as _, surface.width(), surface.height(), 0, gl::RGB, UNSIGNED_BYTE, surface.pixels.as_ptr() as *const c_void);

        // Uniform1i(tex_attrib, 0);

        DrawArrays(TRIANGLES, 0, verticies.len() as i32 / 2);
        context.swap_buffers().unwrap();
        
        BindVertexArray(0);
        gl_err().unwrap();
    
        let mut exit = false;
        ev.poll_events(|ev| {
            match ev {
                Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                    exit = true;
                },
                _ => ()
            }
        });

        if exit {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    DeleteProgram(prog);
    DeleteShader(frag_s);
    DeleteShader(vert_s);

    DeleteBuffers(1, &vbo);
    DeleteVertexArrays(1, &vao);
} }