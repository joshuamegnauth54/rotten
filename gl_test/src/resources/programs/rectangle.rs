use std::rc::Rc;

use crate::{
    context::Gl,
    glenums::{BufferTarget, BufferUsage},
    glerror::GlError,
    memory::{
        stateful::{Buffer, VertexArray},
        GpuDataIndices, GpuDataVerts,
    },
    shaders::datatypes,
};

pub struct Rectangle {
    pub vao: VertexArray,
}

impl Rectangle {
    pub fn new(gl: Rc<Gl>) -> Result<Self, GlError> {
        let rect_verts = datatypes::Rectangle::default();
        // Create buffer object for rectangle vertices
        let vbo = Buffer::new(gl.clone(), BufferTarget::Array, "RectangleVertices")?;
        vbo.write(&rect_verts, BufferUsage::StaticDraw);

        // Element array buffer for repeated vertices
        let ebo = Buffer::new(gl.clone(), BufferTarget::ElementArray, "RectangleEBO")?;
        ebo.write(&rect_verts.indices(), BufferUsage::StaticDraw);

        // Metadata for vertex buffer
        let vao = VertexArray::new(
            gl,
            vbo,
            Some(ebo),
            &rect_verts.memory_layout(),
            "RectangleVAO",
        )?;
        Ok(Self { vao })
    }
}
