use super::gl_support::Gl;

pub(crate) trait Cleanup {
    fn cleanup(&self, gl: &Gl);
}
