#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// https://rust-tutorials.github.io/learn-opengl/basics/000-creating-a-window.html
// https://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html
// https://www.poor.dev/blog/terminal-anatomy/

pub(crate) mod gl_support;
pub(crate) mod glerror;
pub(crate) mod id;
pub(crate) mod label;
pub(crate) mod shaders;

use std::ffi::CString;

use glutin::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, PossiblyCurrent, WindowedContext,
};

use gl_support::{
    gl::{
        self,
        types::{GLint, GLsizeiptr, GLuint, GLvoid},
    },
    Gl,
};
use glerror::GlError;
use shaders::{Shader, ShaderKind, ShaderProgram};

pub struct GlTest<'gl> {
    gl: Gl<'gl>,
    windowed_context: WindowedContext<PossiblyCurrent>,
    event_loop: EventLoop<()>,
}

impl<'gl> GlTest<'gl> {
    pub fn new(width: f32, height: f32) -> Result<Self, GlError> {
        let el = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title("Meow?")
            .with_maximized(true)
            .with_inner_size(LogicalSize::new(width, height));
        let windowed_context = ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &el)
            .expect("Fail to build an OpenGL context.");

        // Make OpenGL context current for this thread(?)
        let windowed_context = unsafe {
            windowed_context
                .make_current()
                .expect("Failed to make new OpenGL context current.")
        };

        dbg!(windowed_context.get_pixel_format());
        // Load function pointers.
        let gl = gl_support::Gl::load_gl(&windowed_context);
        // Enable debug printing
        unsafe {
            gl.register_debug_callback();
        }
        gl.debug_message_control(
            gl_support::DebugSource::DontCare,
            gl_support::DebugType::DontCare,
            gl_support::DebugSeverity::DontCare,
        );

        // Load shaders from files
        let vert_shader = Shader::from_source(
            &gl,
            &CString::new(include_str!("../../src/shader_mods/triangle.vert"))
                .map_err(|_| GlError::Shader("Invalid CString: triangle.vert".to_string()))?,
            ShaderKind::Vertex,
        )?;

        let frag_shader = Shader::from_source(
            &gl,
            &CString::new(include_str!("../../src/shader_mods/triangle.frag"))
                .map_err(|_| GlError::Shader("Invalid CString: triangle.frag".to_string()))?,
            ShaderKind::Fragment,
        )?;

        let shaders = vec![vert_shader, frag_shader];
        let program = ShaderProgram::from_shaders(&gl, &shaders, "Triangle")?;
        gl.insert_shader(program);

        Ok(Self {
            gl,
            windowed_context,
            event_loop: el,
        })
    }

    // Have to refactor these into Gl
    fn viewport(gl: &Gl, size: PhysicalSize<u32>) {
        unsafe {
            // Viewport = actual viewing area.
            gl.Viewport(0, 0, size.width as _, size.height as _);
            gl.ClearColor(0.5, 0.0, 1.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn draw(gl: &Gl) {
        unsafe {
            gl.ClearColor(0.5, 0.0, 1.0, 1.0);
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    fn triangle_vao(gl: &Gl, vertices: &[f32; 18]) -> GLuint {
        // Vertex buffer
        let mut vbo: GLuint = 0;
        unsafe {
            // Create one buffer object name.
            gl.GenBuffers(1, &mut vbo);
            // Bind a buffer to the name in vbo.
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            // Allocate and copy data.
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of_val(vertices)) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            // Unbind buffer
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        // Vertex Array Objects store information about VBOs
        let mut vao: GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            // Make vao current
            gl.BindVertexArray(vao);
            // Rebind vbo because it needs to associated with vao.
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            // location = 0; vertices
            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as GLint,
                std::ptr::null(),
            );
            // location = 1; color information
            gl.EnableVertexAttribArray(1);
            gl.VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as GLint,
                (3 * std::mem::size_of::<f32>()) as *const GLvoid,
            );
        }
        // Unbind VBO and VAO
        unsafe {
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindVertexArray(0);
        }
        // Return Vertex Array object
        vao
    }

    pub fn run(self) {
        // The event loop takes ownership of self when run is called.
        // I'll figure out a less ugly way to do this later
        let Self {
            gl,
            windowed_context,
            event_loop,
        } = self;

        // Use Program with compiled shaders
        shader_program.set_used();

        // Wayland needs the buffers to be swapped before the event loop or else the window hangs.
        // This might be fixed already since I can't find the issue anymore.
        GlTest::viewport(&gl, windowed_context.window().inner_size());
        windowed_context.swap_buffers().unwrap();
        windowed_context.window().set_visible(true);

        // Draw triangle
        #[rustfmt::skip]
        let triangle = vec![-0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
                                    0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
                                    0.0, 0.5, 0.0, 0.0, 0.0, 1.0];
        let triangle_vao = GlTest::triangle_vao(&gl, &triangle[..18].try_into().unwrap());
        shader_program.set_used();
        unsafe {
            gl.BindVertexArray(triangle_vao);
        }

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => (),
                Event::WindowEvent { event, .. } => match event {
                    // Can't figure out scan codes yet.
                    WindowEvent::KeyboardInput { input, .. } if input.virtual_keycode.is_some() => {
                        match input
                            .virtual_keycode
                            .expect("Virtual keycode is empty despite is_some()")
                        {
                            glutin::event::VirtualKeyCode::A => {
                                println!("A pressed")
                            }
                            glutin::event::VirtualKeyCode::B => println!("B pressed"),
                            _ => (),
                        }
                    }
                    // Resize the GL ViewPort if the window is resized
                    WindowEvent::Resized(size) => {
                        windowed_context.resize(size);
                        GlTest::viewport(&gl, size)
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    GlTest::draw(&gl);
                    windowed_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}
