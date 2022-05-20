#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// https://rust-tutorials.github.io/learn-opengl/basics/000-creating-a-window.html
// https://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html
// https://www.poor.dev/blog/terminal-anatomy/

pub(crate) mod cleanup;
pub(crate) mod gl_support;
pub(crate) mod glenums;
pub(crate) mod glerror;
pub(crate) mod id;
pub(crate) mod label;
pub(crate) mod memory;
pub(crate) mod shaders;

use glutin::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, PossiblyCurrent, WindowedContext,
};

use gl_support::Gl;
use glerror::GlError;
use shaders::{ShaderDescriptor, ShaderFrom, ShaderKind, ShaderProgram, datatypes::Triangle};

pub struct GlTest {
    gl: Gl,
    windowed_context: WindowedContext<PossiblyCurrent>,
    event_loop: EventLoop<()>,
}

impl GlTest {
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
        let mut gl = Gl::load_gl(&windowed_context);
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
        ShaderProgram::from_raw(
            &mut gl,
            [
                ShaderDescriptor {
                    kind: ShaderKind::Vertex,
                    from: ShaderFrom::Source(
                        include_str!("../../src/shader_mods/triangle.vert").into(),
                    ),
                },
                ShaderDescriptor {
                    kind: ShaderKind::Fragment,
                    from: ShaderFrom::Source(
                        include_str!("../../src/shader_mods/triangle.frag").into(),
                    ),
                },
            ],
            "Triangle",
        )?;

        Ok(Self {
            gl,
            windowed_context,
            event_loop: el,
        })
    }

    pub fn run(self) {
        // The event loop takes ownership of self when run is called.
        // I'll figure out a less ugly way to do this later
        let Self {
            gl,
            windowed_context,
            event_loop,
        } = self;

        // Wayland needs the buffers to be swapped before the event loop or else the window hangs.
        // This might be fixed already since I can't find the issue anymore.
        gl.viewport(windowed_context.window().inner_size());
        windowed_context.swap_buffers().unwrap();
        windowed_context.window().set_visible(true);

        // Draw triangle
        let triangle_vao = gl.triangle_vao();
        gl.get_shader("Triangle").unwrap().set_used(&gl);
        triangle_vao.bind(&gl);

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
                        gl.viewport(size)
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    gl.draw();
                    windowed_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}
