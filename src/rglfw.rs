#![allow(dead_code)]

use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::CString;

pub use glfw::ffi::*;

pub fn with_c_str<F, T>(s: &str, f: F) -> T
    where
        F: FnOnce(*const c_char) -> T,
{
    let c_str = CString::new(s.as_bytes());
    f(c_str.unwrap().as_bytes_with_nul().as_ptr() as *const _)
}

pub fn get_time() -> f32 {
    unsafe {
        glfwGetTime() as f32
    }
}

pub const GL_TRUE: c_int = 1;
pub const GL_FALSE: c_int = 0;

pub const GLFW_CONTEXT_VERSION_MAJOR: c_int = 0x00022002;
pub const GLFW_CONTEXT_VERSION_MINOR: c_int = 0x00022003;
pub const GLFW_OPENGL_PROFILE: c_int = 0x00022008;
pub const GLFW_OPENGL_CORE_PROFILE: c_int = 0x00032001;
pub const GLFW_OPENGL_FORWARD_COMPAT: c_int = 0x00022006;
pub const GLFW_RESIZABLE: c_int = 0x000020003;
