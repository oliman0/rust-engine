use std::ffi::c_void;

use gl::DEPTH_TEST;

use crate::shader::Shader;
use crate::texture::{ generate_texture_and_size_path, generate_texture };
use crate::vao::create_vao_and_ibo;

pub enum UIElement {
    UISpriteElement(UISpriteElement),
    UITextElement(UITextElement)
}

pub struct UISpriteElement {
    texture_id: u32,
    colour: nalgebra_glm::Vec4,
    position: nalgebra_glm::Vec3,
    size: nalgebra_glm::Vec2,
    using_texture: bool
}

impl Drop for UISpriteElement {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}
pub fn ui_sprite_element(fname: &str, size: nalgebra_glm::Vec2, position: nalgebra_glm::Vec3) -> UISpriteElement {
    let tex_id = generate_texture(fname);
    UISpriteElement { texture_id: tex_id, size: size,
            position: position, colour: nalgebra_glm::vec4(1.0, 1.0, 1.0, 1.0), using_texture: true }
}
pub fn ui_sprite_element_notex(colour: nalgebra_glm::Vec4, size: nalgebra_glm::Vec2, position: nalgebra_glm::Vec3) -> UISpriteElement {
    UISpriteElement { texture_id: 0, size: size,
            position: position, colour: colour, using_texture: false }
}

pub struct UITextElement {
    text: String,
    position: nalgebra_glm::Vec3,
    colour: nalgebra_glm::Vec4
}
pub fn ui_text_element(str: &str, position: nalgebra_glm::Vec3, colour: nalgebra_glm::Vec4) -> UITextElement {
    UITextElement { text: str.to_string(), position: position, colour: colour }
}

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
fn character(fname: &str) -> Character {
    let (tex_id, w, _h) = generate_texture_and_size_path(fname);
    Character { texture_id: tex_id, width: w }
}

pub struct UI {
    // FONT
    characters: Vec<Character>,
    char_height: f32,
    // UI
    elements: Vec<UIElement>,
    vao: u32,
    vbo: u32,
    ibo: u32
}

impl Drop for UI {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ibo);
        }
    }
}
impl UI {
    pub fn draw(&self, ui_shader: &Shader, text_shader: &Shader) {
        for element in &self.elements {
            match element {
                UIElement::UISpriteElement(el) => {self.draw_sprite(el.texture_id, &el.position, &el.size, &el.colour, el.using_texture, ui_shader)}
                UIElement::UITextElement(el) => {self.draw_string(&el.text, &el.position, &el.colour, text_shader)}
            }
        }
    }

    fn draw_sprite(&self, texture_id: u32, position: &nalgebra_glm::Vec3, size: &nalgebra_glm::Vec2, colour: &nalgebra_glm::Vec4, using_texture: bool, shader: &Shader) {
        unsafe {
            shader.use_shader();

            gl::Disable(gl::DEPTH_TEST);

            let mut model: nalgebra_glm::Mat4 = nalgebra_glm::identity();
            model = nalgebra_glm::translate(&model, &position);
            model = nalgebra_glm::scale(&model, &nalgebra_glm::vec3(size.x, size.y, 1.0));
            shader.set_uniform_mat4("model", &model);
            shader.set_uniform_vec4("colour", &colour);
            shader.set_uniform_bool("usingTexture", using_texture);

            gl::BindVertexArray(self.vao);

            if using_texture {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
            }

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            if using_texture {
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }

            gl::Enable(DEPTH_TEST);
        }
    }

    pub fn draw_string(&self, str: &str, pos: &nalgebra_glm::Vec3, colour: &nalgebra_glm::Vec4, shader: &Shader) {
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
    fn draw_character(&self, char: &Character, position: &nalgebra_glm::Vec3, colour: &nalgebra_glm::Vec4, shader: &Shader) {
        unsafe {
            shader.use_shader();

            gl::Disable(gl::DEPTH_TEST);

            let mut model: nalgebra_glm::Mat4 = nalgebra_glm::identity();
            model = nalgebra_glm::translate(&model, &position);
            model = nalgebra_glm::scale(&model, &nalgebra_glm::vec3(char.width, self.char_height, 1.0));
            shader.set_uniform_mat4("model", &model);
            shader.set_uniform_vec4("colour", &colour);
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
pub fn ui(elements: Vec<UIElement>, fname: &str, char_height: f32) -> UI {
    let mut chars: Vec<Character> = Vec::new();

    let mut i = 0;
    while i < 128 {
        chars.push(character(&("fonts/".to_owned() + fname + "/" + &i.to_string())));
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

    let (vao, vbo, ibo) = create_vao_and_ibo(&vertices, &indices);

    UI { characters: chars, char_height: char_height, elements: elements, vao: vao, ibo: ibo, vbo: vbo }
}