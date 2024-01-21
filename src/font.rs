use std::ffi::c_void;

use gl::DEPTH_TEST;

use crate::shader::Shader;
use crate::texture::generate_texture_and_size_path;
use crate::vao::create_vao_and_ibo;

struct Character {
    texture_id: u32,
    width: f32
}

impl Drop for Character {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}
impl Character {
    fn new(fname: &str) -> Self {
        let (tex_id, w, _h) = generate_texture_and_size_path(fname);
        Self { texture_id: tex_id, width: w }
    }
}

pub struct Font {
    characters: Vec<Character>,
    char_height: f32,
    vao: u32,
    vbo: u32,
    ibo: u32
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ibo);
        }
    }
}
impl Font {
    pub fn new(fname: &str, char_height: f32) -> Self {
        let mut font = Self { characters: Vec::new(), char_height: char_height, vao: 0, ibo: 0, vbo: 0 };

        let mut i = 0;
        while i < 128 {
            font.characters.push(Character::new(&("fonts/".to_owned() + fname + "/" + &i.to_string())));
            i+=1;
        }

        let indices: [i32; 6] = [
            0, 1, 2,
            2, 3, 0
        ];

        //vertex data
        let vertices: [f32; 20] = [
            0.0, 0.0, 0.0,       0.0, 1.0,
             1.0, 0.0, 0.0,    1.0, 1.0,
             1.0,  1.0, 0.0, 1.0, 0.0,
            0.0,  1.0, 0.0,    0.0, 0.0
        ];

        (font.vao, font.vbo, font.ibo) = create_vao_and_ibo(&vertices, &indices);

        font
    }
    pub fn draw_string(&self, str: &str, pos: &nalgebra_glm::Vec2, colour: &nalgebra_glm::Vec3, shader: &Shader) {
        let mut position = *pos;

        for char in str.chars() {
            if char as i32 == 32 {
                position.x += 3.0
            }
            else {
                self.draw_character(&self.characters[char as usize], &position, colour, shader);
                position.x += self.characters[char as usize].width + 1.0;
            }
        }
    }

    fn draw_character(&self, char: &Character, position: &nalgebra_glm::Vec2, colour: &nalgebra_glm::Vec3, shader: &Shader) {
        unsafe {
            shader.use_shader();

            gl::Disable(gl::DEPTH_TEST);

            let mut model: nalgebra_glm::Mat4 = nalgebra_glm::identity();
            model = nalgebra_glm::translate(&model, &nalgebra_glm::vec3(position.x, position.y, 0.0));
            model = nalgebra_glm::scale(&model, &nalgebra_glm::vec3(char.width, self.char_height, 1.0));
            shader.set_uniform_mat4("model", &model);
            shader.set_uniform_vec4("colour", &nalgebra_glm::vec4(colour.x, colour.y, colour.z, 0.0));
            shader.set_uniform_bool("usingTexture", true);

            gl::BindVertexArray(self.vao);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, char.texture_id);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            gl::BindTexture(gl::TEXTURE_2D, 0);

            gl::Enable(DEPTH_TEST);
        }
    }
}