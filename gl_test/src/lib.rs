#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// https://rust-tutorials.github.io/learn-opengl/basics/000-creating-a-window.html
// https://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html
// https://www.poor.dev/blog/terminal-anatomy/

pub(crate) mod context;
pub(crate) mod glenums;
pub(crate) mod glerror;
pub(crate) mod label;
pub(crate) mod memory;
pub(crate) mod resources;
pub(crate) mod shaders;

use glenums::{ClearKind, DrawMode};
use glutin::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, PossiblyCurrent, WindowedContext,
};

use context::{Gl, Size};
use glerror::GlError;
use log::info;
use resources::programs::{
    rectangle::Rectangle,
    triangle::{TriangleBuf, TriangleShader},
};
use std::rc::Rc;

use crate::context::{info::ContextInfo, Clear, Color};

pub struct GlTest {
    gl: Rc<Gl>,
    triangle_prog: TriangleShader,
    trianglebuf: TriangleBuf,
    rectanglebuf: Rectangle,
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

        info!("{:?}", windowed_context.get_pixel_format());
        // Load function pointers.
        let gl = Gl::load_gl(|addr| windowed_context.get_proc_address(addr));
        // Enable debug printing
        gl.enable_debug_output();
        gl.debug_message_control(
            glenums::DebugSource::DontCare,
            glenums::DebugType::DontCare,
            glenums::DebugSeverity::DontCare,
            true,
        );

        // Print information on the OpenGL context.
        let context_info = ContextInfo::new(&gl);
        info!("{}", context_info.version);
        info!("Vendor: {}", context_info.vendor);
        info!("GPU: {}", context_info.renderer);
        info!("GLSL version: {}", context_info.glsl);

        // Load shaders from files and construct buffers
        let triangle_prog = TriangleShader::new(gl.clone())?;
        let trianglebuf = TriangleBuf::new(gl.clone())?;
        let rectanglebuf = Rectangle::new(gl.clone())?;

        // Set a base clear color
        let clear = Clear {
            color: Some(Color {
                red: 220. / 255.,
                green: 205. / 255.,
                blue: 1.0,
                alpha: 1.0,
            }),
            ..Default::default()
        };
        clear.set(&gl);

        Ok(Self {
            gl,
            triangle_prog,
            trianglebuf,
            rectanglebuf,
            windowed_context,
            event_loop: el,
        })
    }

    pub fn run(self) {
        // The event loop takes ownership of self when run is called.
        // I'll figure out a less ugly way to do this later
        let Self {
            gl,
            triangle_prog,
            trianglebuf,
            rectanglebuf,
            windowed_context,
            event_loop,
        } = self;

        // Clear on start so the window has something to display.
        gl.clear(ClearKind::ColorBuffer);
        gl.viewport(context::Rect {
            size: gl.viewport_max(),
            ..Default::default()
        });
        windowed_context.swap_buffers().unwrap();
        // Default shader at the moment
        triangle_prog.shader.set_used();

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
                                gl.clear(ClearKind::ColorBuffer);
                                trianglebuf.vao.bind();
                                gl.draw_elements(DrawMode::Triangles, 3, 0);
                                windowed_context.window().request_redraw()
                            }
                            glutin::event::VirtualKeyCode::B => {
                                gl.clear(ClearKind::ColorBuffer);
                                rectanglebuf.vao.bind();
                                gl.draw_elements(DrawMode::Triangles, 6, 0);
                                windowed_context.window().request_redraw()
                            }
                            _ => (),
                        }
                    }
                    // Resize the GL ViewPort if the window is resized
                    WindowEvent::Resized(size) => {
                        windowed_context.resize(size);
                        gl.viewport(context::Rect {
                            size: Size {
                                width: size.width,
                                height: size.height,
                            },
                            ..Default::default()
                        })
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    windowed_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}
