use crate::{
    cleanup::Cleanup,
    gl_support::{
        gl::types::{GLsizeiptr, GLuint, GLvoid},
        Gl,
    },
    glenums::{BufferTarget, BufferUsage},
    label::Label,
    memory::GpuData,
};
use log::error;
use std::rc::Rc;

// Refactor this later into DSA and non-DSA buffer types
// https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_direct_state_access.txt
#[derive(Debug)]
pub struct ClassicBuffer {
    id: GLuint,
    target: BufferTarget,
    label: Rc<str>,
}

impl ClassicBuffer {
    /// Allocate a new GPU buffer with `BufferTarget` as the target.
    pub fn new<S>(gl: &Gl, target: BufferTarget, label: S) -> Self
    where
        S: Into<Rc<str>>,
    {
        // 0 is a reserved object name.
        let mut id = 0;
        unsafe {
            // Reserve one buffer object. VRAM is not allocated here.
            gl.GenBuffers(1, &mut id);
        }

        // GenBuffers shouldn't return an id of 0, so I'll just check this in debug only
        #[cfg(debug_assertions)]
        if id == 0 {
            error!("GenBuffers failed to reserve a buffer name.\nObject id = 0 for {target:?}");
        }

        let label = label.into();
        Self { id, target, label }
    }

    /// Current target that will be set in calls to [bind]
    pub fn target(&self) -> BufferTarget {
        self.target
    }

    /// Bind this buffer to the currently set target.
    pub fn bind(&self, gl: &Gl) {
        unsafe { gl.BindBuffer(self.target.bits(), self.id) }
    }

    /// Bind this buffer to a new target.
    pub fn rebind(&mut self, gl: &Gl, target: BufferTarget) {
        self.target = target;
        self.bind(gl)
    }

    /// Unbind this buffer from the current target
    pub fn unbind(&self, gl: &Gl) {
        Self::unbind_any(gl, self.target)
    }

    /// Unbind arbitrary buffer from a target
    pub fn unbind_any(gl: &Gl, target: BufferTarget) {
        unsafe { gl.BindBuffer(target.bits(), 0) }
    }

    /// Copy data into buffer.
    pub fn write<D, const N: usize>(&self, gl: &Gl, data: &D, usage: BufferUsage)
    where
        D: GpuData<N>,
    {
        // Bind current buffer to copy the data to the appropriate object
        self.bind(gl);

        unsafe {
            // Allocate VRAM of size data.size() and copy the data into the buffer
            // BufferData is a non-DSA function that modifies the global target binding
            gl.BufferData(
                self.target.bits(),
                data.size_total() as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                usage.bits(),
            )
        }
    }
}

impl Cleanup for ClassicBuffer {
    fn cleanup(&self, gl: &Gl) {
        unsafe { gl.DeleteBuffers(1, &self.id) }
    }
}

impl Label for ClassicBuffer {
    type Output = Rc<str>;

    fn label(&self) -> Self::Output {
        self.label.clone()
    }
}
