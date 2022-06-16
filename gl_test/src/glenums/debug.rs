//! Enumerations for [glDebugMessageControl](https://docs.gl/gl4/glDebugMessageControl)
//! [OpenGL Wiki](https://www.khronos.org/opengl/wiki/Debug_Output)

#![allow(non_upper_case_globals)]

use crate::context::gl::{self, types::GLenum};
use bitflags::bitflags;
use std::fmt::{self, Display, Formatter};

bitflags! {
    // Importance or severity of message
    #[repr(C)]
    pub struct DebugSeverity: GLenum {
        const DontCare = gl::DONT_CARE;
        // All errors or undefined behavior
        const High = gl::DEBUG_SEVERITY_HIGH;
        // Performance warnings and deprecated functionality
        const Medium = gl::DEBUG_SEVERITY_MEDIUM;
        // Redundant state changes and other small mistakes
        const Low = gl::DEBUG_SEVERITY_LOW;
        // Everything else
        const Notification = gl::DEBUG_SEVERITY_NOTIFICATION;
    }

    // Source of error message
    #[repr(C)]
    pub struct DebugSource: GLenum {
        const DontCare = gl::DONT_CARE;
        // Calls to the OpenGL API
        const Api = gl::DEBUG_SOURCE_API;
        // Calls to a windowing API
        const WindowSystem = gl::DEBUG_SOURCE_WINDOW_SYSTEM;
        // Shader compiler
        const ShaderCompiler = gl::DEBUG_SOURCE_SHADER_COMPILER;
        // Associated application beyond OpenGL itself
        const ThirdParty = gl::DEBUG_SOURCE_THIRD_PARTY;
        // User messages
        const Application = gl::DEBUG_SOURCE_APPLICATION;
        // Anything else
        const Other = gl::DEBUG_SOURCE_OTHER;
    }

    // Type of message such as error or performance concerns
    #[repr(C)]
    pub struct DebugType: GLenum {
        const DontCare = gl::DONT_CARE;
        // Error from the API.
        const Error = gl::DEBUG_TYPE_ERROR;
        // Deprecated OpenGL calls
        const Deprecated = gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR;
        // Undefined behavior - code may work now but may cause unpredictable errors later or on a
        // different machine
        const Undefined = gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR;
        // Operation may not be portable
        const Portability = gl::DEBUG_TYPE_PORTABILITY;
        // Code may cause performance issues
        const Performance = gl::DEBUG_TYPE_PERFORMANCE;
        // Event intended to be picked up by debuggers or other tools
        const Marker = gl::DEBUG_TYPE_MARKER;
        // Group pushing
        const Push = gl::DEBUG_TYPE_PUSH_GROUP;
        // Group popping
        const Pop = gl::DEBUG_TYPE_POP_GROUP;
        // Everything else
        const Other = gl::DEBUG_TYPE_OTHER;
    }
}

// DONT_CARE is the default for all of these. The value is NOT 0x0 so deriving Default is
// impossible.

impl Default for DebugSeverity {
    fn default() -> Self {
        DebugSeverity::DontCare
    }
}

impl Default for DebugSource {
    fn default() -> Self {
        DebugSource::DontCare
    }
}

impl Default for DebugType {
    fn default() -> Self {
        DebugType::DontCare
    }
}

// Display
impl Display for DebugSeverity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            DebugSeverity::DontCare => write!(f, "Any"),
            DebugSeverity::High => write!(f, "High"),
            DebugSeverity::Medium => write!(f, "Medium"),
            DebugSeverity::Low => write!(f, "Low"),
            DebugSeverity::Notification => write!(f, "Notification"),
            _ => unreachable!("Undefined flag passed by OpenGL for DebugSeverity"),
        }
    }
}

impl Display for DebugSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            DebugSource::DontCare => write!(f, "Any"),
            DebugSource::Api => write!(f, "API"),
            DebugSource::WindowSystem => write!(f, "Window system"),
            DebugSource::ShaderCompiler => write!(f, "Shader compiler"),
            DebugSource::ThirdParty => write!(f, "Third party"),
            DebugSource::Application => write!(f, "User"),
            DebugSource::Other => write!(f, "Other"),
            _ => unreachable!("Undefined flag passed by OpenGL for DebugSource"),
        }
    }
}

impl Display for DebugType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            DebugType::DontCare => write!(f, "Any"),
            DebugType::Error => write!(f, "Error"),
            DebugType::Deprecated => write!(f, "Deprecated"),
            DebugType::Undefined => write!(f, "Undefined behavior"),
            DebugType::Portability => write!(f, "Portability"),
            DebugType::Performance => write!(f, "Performance"),
            DebugType::Marker => write!(f, "Marker"),
            DebugType::Push => write!(f, "Push group"),
            DebugType::Pop => write!(f, "Pop group"),
            DebugType::Other => write!(f, "Other"),
            _ => unreachable!("Undefined flag passed by OpenGL for DebugType"),
        }
    }
}
