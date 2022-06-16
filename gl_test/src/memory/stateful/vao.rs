use crate::{
    context::{
        gl::{
            self,
            types::{GLint, GLuint, GLvoid},
        },
        Gl,
    },
    glenums::BufferTarget,
    glerror::GlError,
    label::Label,
    memory::{stateful::Buffer, Layout},
};

use log::error;
use std::rc::Rc;

/// Vertex Array objects store metadata on vertex buffers
//#[derive(Debug)]
pub struct VertexArray {
    gl: Rc<Gl>,
    id: GLuint,
    vbo: Buffer,
    ebo: Option<Buffer>,
    label: Rc<str>,
}

impl VertexArray {
    pub fn new<S>(
        gl: Rc<Gl>,
        mut vbo: Buffer,
        ebo: Option<Buffer>,
        layouts: &[Layout],
        label: S,
    ) -> Result<Self, GlError>
    where
        S: Into<Rc<str>>,
    {
        // Bind the current buffer to the ARRAY_BUFFER target.
        // Note: VBO may be bound at any time before VertexAttribPointer since that's the function
        // that reads the global state.
        vbo.rebind(BufferTarget::Array);

        // Create a single Vertex Array object.
        let mut id = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut id);
            // Bind VAO; if id is still 0 BindVertexArray will just unbind any previously set VAO.
            // The id shouldn't be 0 unless something is really broken anyway.
            gl.BindVertexArray(id);
        }

        // An id of 0 only occurs if the context is borked.
        if id == 0 {
            error!("GenVertexArrays did not create a vertex array.");
            return Err(GlError::Buffer(
                "GenVertexArrays returned a vertex array object id of 0.".into(),
            ));
        }

        // Not sure, but I think the element buffer should be bound after the VAO.
        if let Some(ebo) = ebo.as_ref() {
            ebo.bind();
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

        // Vertex arrays need to be unbound before the EBO (and VBO?) or else the unbind call is
        // saved to the VAO.
        unsafe { gl.BindVertexArray(0) }
        if let Some(ebo) = ebo.as_ref() {
            ebo.unbind()
        }
        vbo.unbind();

        let label = label.into();
        Ok(Self {
            gl,
            id,
            vbo,
            ebo,
            label,
        })
    }

    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vbo
    }

    pub fn element_buffer(&self) -> Option<&Buffer> {
        self.ebo.as_ref()
    }

    pub fn bind(&self) {
        // The buffers referenced by the VAO do not need to be bound too.
        unsafe { self.gl.BindVertexArray(self.id) }
    }

    pub fn unbind(gl: &Rc<Gl>) {
        unsafe { gl.BindVertexArray(0) }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteVertexArrays(1, &self.id) }
    }
}

impl Label for VertexArray {
    type Output = Rc<str>;

    fn label(&self) -> Self::Output {
        self.label.clone()
    }
}
