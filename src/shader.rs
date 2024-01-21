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
    pub fn new(v_path: &str, f_path: &str) -> Self {
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
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(vert_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vert_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
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
                gl::GetShaderInfoLog(frag_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }

            // link shaders
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vert_shader);
            gl::AttachShader(shader_program, frag_shader);
            gl::LinkProgram(shader_program);

            // check for linking errors
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }

            gl::DeleteShader(vert_shader);
            gl::DeleteShader(frag_shader);

            Self {shader_program: shader_program} 
        }
    }
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
    pub fn set_uniform_bool(&self, name: &str, value: bool) {
        unsafe {
            let cname = CString::new(name.as_bytes()).unwrap();

            gl::UseProgram(self.shader_program);
            let iloc = gl::GetUniformLocation(self.shader_program, cname.as_ptr());
            gl::Uniform1i(iloc, value as i32);
        }
    }
}
