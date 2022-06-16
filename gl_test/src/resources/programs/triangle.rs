use crate::{
    context::Gl,
    glenums::{BufferTarget, BufferUsage},
    glerror::GlError,
    memory::{
        stateful::{Buffer, VertexArray},
        GpuDataIndices, GpuDataVerts,
    },
    shaders::{datatypes::Triangle, ShaderDescriptor, ShaderFrom, ShaderKind, ShaderProgram},
};
use std::rc::Rc;

pub struct TriangleShader {
    pub shader: ShaderProgram,
}

impl TriangleShader {
    pub fn new(gl: Rc<Gl>) -> Result<Self, GlError> {
        // Load triangle shaders from source and link them into a program
        let shader = ShaderProgram::from_raw(
            gl,
            [
                ShaderDescriptor {
                    kind: ShaderKind::Vertex,
                    from: ShaderFrom::FilePath("assets/shaders/triangle.vert".into()),
                },
                ShaderDescriptor {
                    kind: ShaderKind::Fragment,
                    from: ShaderFrom::FilePath("assets/shaders/triangle.frag".into()),
                },
            ],
            "TriangleShader",
        )?;

        Ok(Self { shader })
    }
}

pub struct TriangleBuf {
    pub vao: VertexArray,
}

impl TriangleBuf {
    pub fn new(gl: Rc<Gl>) -> Result<Self, GlError> {
        // Allocate buffers and read data into them
        let triangle_verts = Triangle::default();
        let vbo = Buffer::new(gl.clone(), BufferTarget::Array, "TriangleVerts")?;
        vbo.write(&triangle_verts, BufferUsage::StaticDraw);

        // Element array
        let ebo = Buffer::new(gl.clone(), BufferTarget::ElementArray, "TriangleEBO")?;
        ebo.write(&triangle_verts.indices(), BufferUsage::StaticDraw);

        // Triangle vertices metadata
        let vao = VertexArray::new(
            gl,
            vbo,
            Some(ebo),
            &triangle_verts.memory_layout(),
            "TriangleVao",
        )?;

        Ok(Self { vao })
    }
}
