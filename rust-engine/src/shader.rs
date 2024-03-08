use std::fs;
use std::ffi::CString;
use std::ptr;
use std::str;

use gl::types::*;

use nalgebra_glm;

pub struct Shader {
    shader_program: u32
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.shader_program);
        }
    }
}
impl Shader {
    pub fn use_shader(&self) {unsafe{gl::UseProgram(self.shader_program)}}
    pub fn set_uniform_mat4(&self, name: &str, value: &nalgebra_glm::Mat4) {
        unsafe {
            let cname = CString::new(name.as_bytes()).unwrap();

            gl::UseProgram(self.shader_program);
            let iloc = gl::GetUniformLocation(self.shader_program, cname.as_ptr());
            gl::UniformMatrix4fv(iloc, 1, 0, value.as_ptr());
        }
    }
    pub fn set_uniform_vec4(&self, name: &str, value: &nalgebra_glm::Vec4) {
        unsafe {
            let cname = CString::new(name.as_bytes()).unwrap();

            gl::UseProgram(self.shader_program);
            let iloc = gl::GetUniformLocation(self.shader_program, cname.as_ptr());
            gl::Uniform4fv(iloc, 1, value.as_ptr());
        }
    }
    pub fn set_uniform_vec3(&self, name: &str, value: &nalgebra_glm::Vec3) {
        unsafe {
            let cname = CString::new(name.as_bytes()).unwrap();

            gl::UseProgram(self.shader_program);
            let iloc = gl::GetUniformLocation(self.shader_program, cname.as_ptr());
            gl::Uniform3fv(iloc, 1, value.as_ptr());
        }
    }
    pub fn set_uniform_vec2(&self, name: &str, value: &nalgebra_glm::Vec2) {
        unsafe {
            let cname = CString::new(name.as_bytes()).unwrap();

            gl::UseProgram(self.shader_program);
            let iloc = gl::GetUniformLocation(self.shader_program, cname.as_ptr());
            gl::Uniform2fv(iloc, 1, value.as_ptr());
        }
    }
    pub fn set_uniform_bool(&self, name: &str, value: bool) {
        unsafe {
            let cname = CString::new(name.as_bytes()).unwrap();

            gl::UseProgram(self.shader_program);
            let iloc = gl::GetUniformLocation(self.shader_program, cname.as_ptr());
            gl::Uniform1i(iloc, value as i32);
        }
    }
    pub fn set_uniform_float(&self, name: &str, value: f32) {
        unsafe {
            let cname = CString::new(name.as_bytes()).unwrap();

            gl::UseProgram(self.shader_program);
            let iloc = gl::GetUniformLocation(self.shader_program, cname.as_ptr());
            gl::Uniform1f(iloc, value);
        }
    }
}
pub fn shader(v_path: &str, f_path: &str) -> Shader {
    let vshader_s = fs::read_to_string(v_path)
        .expect("Should have been able to read the file");
    let fshader_s = fs::read_to_string(f_path)
        .expect("Should have been able to read the file");
    
    unsafe {
        //vertex shader
        let vert_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(vshader_s.as_bytes()).unwrap();
        gl::ShaderSource(vert_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vert_shader);
        // check for shader compile errors
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        let mut log_len = 0_i32;
        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
        gl::GetShaderiv(vert_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(vert_shader, 512, &mut log_len, info_log.as_mut_ptr() as *mut GLchar);
            info_log.set_len(log_len.try_into().unwrap());
            println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
        }
        // fragment shader
        let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(fshader_s.as_bytes()).unwrap();
        gl::ShaderSource(frag_shader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(frag_shader);
        // check for shader compile errors
        gl::GetShaderiv(frag_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(frag_shader, 512, &mut log_len, info_log.as_mut_ptr() as *mut GLchar);
            info_log.set_len(log_len.try_into().unwrap());
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
        }
        // link shaders
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vert_shader);
        gl::AttachShader(shader_program, frag_shader);
        gl::LinkProgram(shader_program);
        // check for linking errors
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(shader_program, 512, &mut log_len, info_log.as_mut_ptr() as *mut GLchar);
            info_log.set_len(log_len.try_into().unwrap());
            println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
        }
        gl::DeleteShader(vert_shader);
        gl::DeleteShader(frag_shader);
        Shader {shader_program: shader_program} 
    }
}
pub fn compute_shader(path: &str) -> Shader {
    let shader_s = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    
    unsafe {
        // compile shader
        let shader = gl::CreateShader(gl::COMPUTE_SHADER);
        let c_str_comp = CString::new(shader_s.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str_comp.as_ptr(), ptr::null());
        gl::CompileShader(shader);
        // check for shader compile errors
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        let mut log_len = 0_i32;
        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(shader, 512, &mut log_len, info_log.as_mut_ptr() as *mut GLchar);
            info_log.set_len(log_len.try_into().unwrap());
            println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
        }

        // link shaders
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, shader);
        gl::LinkProgram(shader_program);
        // check for linking errors
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(shader_program, 512, &mut log_len, info_log.as_mut_ptr() as *mut GLchar);
            info_log.set_len(log_len.try_into().unwrap());
            println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
        }
        gl::DeleteShader(shader);
        Shader {shader_program: shader_program} 
    }
}