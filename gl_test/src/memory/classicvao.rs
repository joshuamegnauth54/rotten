use crate::{
    cleanup::Cleanup,
    gl_support::{
        gl::{
            self,
            types::{GLint, GLuint, GLvoid},
        },
        Gl,
    },
    glenums::BufferTarget,
    label::Label,
    memory::{ClassicBuffer, Layout},
};

use log::error;
use std::rc::Rc;

/// Vertex Array objects store metadata on vertex buffers
#[derive(Debug)]
pub struct ClassicVao {
    id: GLuint,
    vbo: ClassicBuffer,
    label: Rc<str>,
}

impl ClassicVao {
    pub fn new<S>(gl: &Gl, mut vbo: ClassicBuffer, layouts: &[Layout], label: S) -> Self
    where
        S: Into<Rc<str>>,
    {
        // Bind the current buffer to the ARRAY_BUFFER target.
        // Note: VBO may be bound at any time before VertexAttribPointer since that's the function
        // that reads the global state.
        vbo.rebind(gl, BufferTarget::Array);

        // Create a single Vertex Array object.
        let mut id = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut id);
            // Bind VAO; if id is still 0 BindVertexArray will just unbind any previously set VAO.
            // The id shouldn't be 0 unless something is really broken anyway.
            gl.BindVertexArray(id);
        }

        // Only checking this in debug because it shouldn't happen.
        #[cfg(debug_assertions)]
        if id == 0 {
            error!("GenVertexArrays did not create a vertex array.");
        }

        // Associate memory layout with VAO
        for layout in layouts {
            unsafe {
                // Enable a location qualifier index
                gl.EnableVertexAttribArray(layout.index as _);
                // Apply metadata to the location qualifier
                gl.VertexAttribPointer(
                    layout.index as _,
                    layout.size as _,
                    gl::FLOAT,
                    gl::FALSE,
                    layout.stride as GLint,
                    layout.start as *const GLvoid,
                );
            }
        }

        let label = label.into();
        Self { id, vbo, label }
    }

    pub fn bind(&self, gl: &Gl) {
        unsafe { gl.BindVertexArray(self.id) }
    }

    pub fn unbind(gl: &Gl) {
        unsafe { gl.BindVertexArray(0) }
    }
}

impl Cleanup for ClassicVao {
    fn cleanup(&self, gl: &Gl) {
        self.vbo.cleanup(gl);
        unsafe { gl.DeleteVertexArrays(1, &self.id) }
    }
}

impl Label for ClassicVao {
    type Output = Rc<str>;

    fn label(&self) -> Self::Output {
        self.label.clone()
    }
}
