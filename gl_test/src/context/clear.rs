use std::rc::Rc;

use crate::context::Gl;

#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Clear {
    pub color: Option<Color>,
    pub depth: Option<f32>,
    pub stencil: Option<u32>,
}

impl Default for Clear {
    fn default() -> Self {
        let color = Default::default();
        let depth = Default::default();
        let stencil = Default::default();

        Self {
            color: Some(color),
            depth: Some(depth),
            stencil: Some(stencil),
        }
    }
}

impl Clear {
    /// Set this struct as the current clear color, depth, and/or stencil.
    pub fn set(&self, gl: &Rc<Gl>) {
        if let Some(color) = self.color {
            unsafe {
                gl.ClearColor(color.red, color.green, color.blue, color.alpha);
            }
        }

        if let Some(depth) = self.depth {
            unsafe { gl.ClearDepthf(depth) }
        }

        if let Some(stencil) = self.stencil {
            unsafe { gl.ClearStencil(stencil as _) }
        }
    }
}
