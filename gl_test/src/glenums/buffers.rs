#![allow(non_upper_case_globals)]

use crate::context::gl::{self, types::GLenum};
use bitflags::bitflags;

bitflags! {
    /// Buffer target flags for [https://docs.gl/gl4/glBindBuffer](glBindBuffer).
    #[repr(C)]
    pub struct BufferTarget: GLenum {
        /// Vertex attributes
        const Array = gl::ARRAY_BUFFER;
        /// Atomic counter storage
        const AtomicCounter = gl::ATOMIC_COUNTER_BUFFER;
        /// Buffer copy source
        const CopyRead = gl::COPY_READ_BUFFER;
        /// Buffer copy destination
        const CopyWrite = gl::COPY_WRITE_BUFFER;
        /// Indirect compute dispatch commands
        const DispatchIndirect = gl::DISPATCH_INDIRECT_BUFFER;
        /// Indirect command arguments
        const DrawIndirect = gl::DRAW_INDIRECT_BUFFER;
        /// Vertex array indices
        const ElementArray = gl::ELEMENT_ARRAY_BUFFER;
        /// Pixel read target
        const PixelPack = gl::PIXEL_PACK_BUFFER;
        /// Texture data source
        const PixelUnpack = gl::PIXEL_UNPACK_BUFFER;
        /// Query result buffer
        const Query = gl::QUERY_BUFFER;
        /// Read-write storage for shaders
        const ShaderStorage = gl::SHADER_STORAGE_BUFFER;
        /// Texture data buffer
        const TextureBuffer = gl::TEXTURE_BUFFER;
        /// Transform feedback buffer
        const TransformFeedback = gl::TRANSFORM_FEEDBACK_BUFFER;
        /// Uniform block storage
        const Uniform = gl::UNIFORM_BUFFER;
    }
}

bitflags! {
    /// Buffer usage flags for [glBufferData](https://docs.gl/gl4/glBufferData).
    /// Usage flags hint to the implementation how the buffer will be accessed. The buffer may
    /// still be used in other ways, but those operations would be slower than if the proper flag
    /// were applied.
    ///
    /// ## Frequency of access
    /// - **Stream:** Buffer's data will be set repeatedly but used a few times.
    /// - **Static:** Data will be set once and used many times.
    /// - **Dynamic:** Modified many times and used many times.
    ///
    /// ## Nature of access:
    /// - **Draw:**
    /// - **Read:**
    /// - **Copy:**
    ///
    /// [Source](https://www.khronos.org/opengl/wiki/Buffer_Object)
    #[repr(C)]
    pub struct BufferUsage: GLenum {
        const StreamDraw = gl::STREAM_DRAW;
        const StreamRead = gl::STREAM_READ;
        const StreamCopy = gl::STREAM_COPY;
        const StaticDraw = gl::STATIC_DRAW;
        const StaticRead = gl::STATIC_READ;
        const StaticCopy = gl::STATIC_COPY;
        const DynamicDraw = gl::DYNAMIC_DRAW;
        const DynamicRead = gl::DYNAMIC_READ;
        const DynamicCopy = gl::DYNAMIC_COPY;
    }
}
