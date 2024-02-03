use std::ffi::{CString, c_void};

use stb_image::stb_image;

fn raw_generate_texture(fname: &str) -> u32 { 
    unsafe {
        let mut texture_id: u32 = 0;
        let (mut width, mut height, mut bit_depth): (i32, i32, i32) = (0, 0, 0); 

        stb_image::stbi_set_flip_vertically_on_load(0);
        
        let cstr_fname = CString::new(("./res/".to_owned() + fname + ".png").as_bytes()).unwrap(); 
        let tex_data = stb_image::stbi_load(
            cstr_fname.as_ptr() as *const i8,
            &mut width, &mut height,
            &mut bit_depth, 0);
            
        gl::GenTextures(1, &mut texture_id) ;
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, tex_data as *const c_void);

        gl::BindTexture(gl::TEXTURE_2D, 0);
            
        stb_image::stbi_image_free(tex_data as *mut c_void);
        
        texture_id
    }
}

fn raw_generate_texture_and_size(fname: &str) -> (u32, f32, f32) { 
    unsafe {
        let mut texture_id: u32 = 0;
        let (mut width, mut height, mut bit_depth): (i32, i32, i32) = (0, 0, 0); 

        stb_image::stbi_set_flip_vertically_on_load(0);
        
        let cstr_fname = CString::new(("./res/".to_owned() + fname + ".png").as_bytes()).unwrap(); 
        let tex_data = stb_image::stbi_load(
            cstr_fname.as_ptr() as *const i8,
            &mut width, &mut height,
            &mut bit_depth, 0);
            
        gl::GenTextures(1, &mut texture_id) ;
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, tex_data as *const c_void);

        gl::BindTexture(gl::TEXTURE_2D, 0);
            
        stb_image::stbi_image_free(tex_data as *mut c_void);
        
        (texture_id, width as f32, height as f32)
    }
}

pub fn generate_texture(fname: &str) -> u32 {
    raw_generate_texture(("textures/".to_owned() + fname).as_str())
}
pub fn generate_texture_path(fname: &str) -> u32 {
    raw_generate_texture(fname)
}
pub fn generate_texture_and_size(fname: &str) -> (u32, f32, f32) {
    raw_generate_texture_and_size(("textures/".to_owned() + fname).as_str())
}
pub fn generate_texture_and_size_path(fname: &str) -> (u32, f32, f32) {
    raw_generate_texture_and_size(fname)
}
