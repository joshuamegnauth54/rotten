use crate::{
    context::{gl, Gl},
    glenums::{ContextFlags, ContextProfile, GetString},
};
use std::{borrow::Cow, fmt, rc::Rc};

#[derive(Debug, Clone, Copy)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
}

impl ApiVersion {
    pub fn new(gl: &Rc<Gl>) -> Self {
        let mut major = 0;
        let mut minor = 0;

        unsafe {
            gl.GetIntegerv(gl::MAJOR_VERSION, &mut major);
            gl.GetIntegerv(gl::MINOR_VERSION, &mut minor);
        }

        Self {
            major: major as _,
            minor: minor as _,
        }
    }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OpenGL v{}.{}", self.major, self.minor)
    }
}

/// Information on the current OpenGL context such as the version or supported extensions.
#[derive(Debug, Clone)]
pub struct ContextInfo {
    pub version: ApiVersion,
    pub vendor: String,
    pub renderer: String,
    pub extensions: Vec<String>,
    pub profile: ContextProfile,
    pub flags: ContextFlags,
    pub glsl: String,
}

impl ContextInfo {
    pub fn new(gl: &Rc<Gl>) -> Self {
        let version = ApiVersion::new(gl);
        let vendor = gl
            .get_string(GetString::Vendor, None)
            .unwrap_or_default()
            .into();
        let renderer = gl
            .get_string(GetString::Renderer, None)
            .unwrap_or_default()
            .into();
        let extensions = ExtensionsIter::iter(gl).map(Into::into).collect();
        let glsl = gl
            .get_string(GetString::GlslVersion, None)
            .unwrap_or_default()
            .into();

        let mut profile_bits = 0;
        unsafe { gl.GetIntegerv(gl::CONTEXT_PROFILE_MASK, &mut profile_bits) }
        let profile = ContextProfile::from_bits_truncate(profile_bits as _);

        let mut flags_bits = 0;
        unsafe { gl.GetIntegerv(gl::CONTEXT_FLAGS, &mut flags_bits) }
        let flags = ContextFlags::from_bits_truncate(flags_bits as _);

        Self {
            version,
            vendor,
            renderer,
            extensions,
            profile,
            flags,
            glsl,
        }
    }
}

// Iterator for extensions supported by this context
pub struct ExtensionsIter<'gl> {
    gl: &'gl Rc<Gl>,
    end: usize,
    current: usize,
}

impl<'gl> ExtensionsIter<'gl> {
    fn iter(gl: &'gl Rc<Gl>) -> Self {
        let mut end = 0;
        unsafe { gl.GetIntegerv(gl::NUM_EXTENSIONS, &mut end) }

        Self {
            gl,
            end: end as _,
            current: 0,
        }
    }
}

impl<'gl> Iterator for ExtensionsIter<'gl> {
    type Item = Cow<'gl, str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let extension = self
                .gl
                .get_string(GetString::Extensions, Some(self.current));
            self.current += 1;
            extension
        } else {
            None
        }
    }
}
