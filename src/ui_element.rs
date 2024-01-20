use std::os::raw::c_void;

use nalgebra_glm;

use crate::shader::Shader;
use crate::texture::{generate_texture, generate_texture_and_size_path};
use crate::vao::create_vao_and_ibo;

pub struct UIElement {
    vao: u32,
    ibo: u32,
    i_count: i32,
    position: nalgebra_glm::Vec3,
    colour: nalgebra_glm::Vec4,
    using_texture: bool,
    texture: u32
}   

impl UIElement {
    fn create(pos: nalgebra_glm::Vec3, sizex: f32, sizey: f32, texture: u32, colour: nalgebra_glm::Vec4, using_texture: bool) -> Self {
        //index data
        let indices: [i32; 6] = [
            0, 1, 2,
            2, 3, 0
        ];

        //vertex data
        let vertices: [f32; 20] = [
            0.0, 0.0, 0.0,       0.0, 0.0,
             sizex, 0.0, 0.0,    1.0, 0.0,
             sizex,  sizey, 0.0, 1.0, 1.0,
            0.0,  sizey, 0.0,    0.0, 1.0
        ];

        let (vao, ibo) = create_vao_and_ibo(&vertices, &indices, 2);

        Self {vao: vao, ibo: ibo, i_count: 6, position: pos, colour: colour, using_texture: using_texture, texture: texture}
    }
    
    pub fn new(pos: nalgebra_glm::Vec3, sizex: f32, sizey: f32, texture: &str) -> Self {
        let tex_id = generate_texture(texture);
        Self::create(pos, sizex, sizey, tex_id, nalgebra_glm::vec4(1.0, 1.0, 1.0, 1.0), true)
    }
    pub fn new_notex(pos: nalgebra_glm::Vec3, sizex: f32, sizey: f32, colour: nalgebra_glm::Vec4) -> Self {
        Self::create(pos, sizex, sizey, 0, colour, false)
    }
    pub fn new_fontchar(pos: nalgebra_glm::Vec3, font: &str, texture: &str) -> Self {
        let (tex_id, sizex, sizey) = generate_texture_and_size_path(("fonts/".to_owned() + font + "/" + texture).as_str());
        Self::create(pos, sizex, sizey, tex_id, nalgebra_glm::vec4(1.0, 1.0, 1.0, 1.0), true)
    }
    pub fn draw(&self, shader: &Shader) {
        unsafe {
            shader.use_shader();

            let mut model: nalgebra_glm::Mat4 = nalgebra_glm::identity();
            model = nalgebra_glm::translate(&model, &self.position);
            shader.set_uniform_mat4("model", &model);
            shader.set_uniform_vec4("colour", &self.colour);
            shader.set_uniform_bool("usingTexture", self.using_texture);

            gl::BindVertexArray(self.vao);

            if self.using_texture {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, self.texture);
            }

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::DrawElements(gl::TRIANGLES, self.i_count, gl::UNSIGNED_INT, 0 as *const c_void);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            if self.using_texture {
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }
    }
    pub fn move_pos(&mut self, vec: nalgebra_glm::Vec3) { self.position += vec; }
    pub fn set_pos(&mut self, vec: nalgebra_glm::Vec3) { self.position = vec; }
}
