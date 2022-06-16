//! Mode enumerations for [glDrawElements](https://docs.gl/gl4/glDrawElements) or [glDrawArrays](https://docs.gl/gl4/glDrawArrays)
#![allow(non_upper_case_globals)]

use crate::context::gl::{self, types::GLenum};
use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    pub struct DrawMode: GLenum {
        const Points = gl::POINTS;
        const LineStrip = gl::LINE_STRIP;
        const LineLoop = gl::LINE_LOOP;
        const Lines = gl::LINES;
        const LineStripAdjacency = gl::LINE_STRIP_ADJACENCY;
        const TriangleStrip = gl::TRIANGLE_STRIP;
        const TriangleFan = gl::TRIANGLE_FAN;
        const Triangles = gl::TRIANGLES;
        const TriangleStripAdjacency = gl::TRIANGLE_STRIP_ADJACENCY;
        const TrianglesAdjacency = gl::TRIANGLES_ADJACENCY;
        const Patches = gl::PATCHES;
    }
}
