use std::ffi::c_void;

use gl::DEPTH_TEST;

use crate::rglfw;
use crate::scene::Scene;
use crate::shader::Shader;
use crate::texture::{ generate_texture_and_size_path, generate_texture };
use crate::vao::create_vao_and_ibo;
use crate::window::InputHandler;

pub enum UIElement {
    UIButton(UIButtonElement),
    UIDisplay(UIDisplay)
}

pub enum UIDisplay {
    UISprite(UISprite),
    UIText(UIText)
}

pub struct UIButtonElement {
    display: UIDisplay,
    on_click_fn: fn(&Scene, &UI)
}
pub fn ui_button(display: UIDisplay, on_click: fn(&Scene, &UI)) -> UIElement {
    UIElement::UIButton(UIButtonElement { display: display, on_click_fn: on_click })
}

pub struct UISprite {
    texture_id: u32,
    colour: nalgebra_glm::Vec4,
    position: nalgebra_glm::Vec3,
    size: nalgebra_glm::Vec2,
    using_texture: bool
}

impl Drop for UISprite {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}
pub fn ui_sprite(fname: &str, size: nalgebra_glm::Vec2, position: nalgebra_glm::Vec3) -> UIDisplay {
    let tex_id = generate_texture(fname);
    UIDisplay::UISprite(UISprite { texture_id: tex_id, size: size,
            position: position, colour: nalgebra_glm::vec4(1.0, 1.0, 1.0, 1.0), using_texture: true })
}
pub fn ui_sprite_notex(colour: nalgebra_glm::Vec4, size: nalgebra_glm::Vec2, position: nalgebra_glm::Vec3) -> UIDisplay {
    UIDisplay::UISprite(UISprite { texture_id: 0, size: size,
            position: position, colour: colour, using_texture: false })
}

pub struct UIText {
    text: String,
    position: nalgebra_glm::Vec3,
    colour: nalgebra_glm::Vec4,
    text_size: f32,
    padding: nalgebra_glm::Vec2,
    bg_colour: nalgebra_glm::Vec4,
    bg_texture_id: u32,
}
pub fn ui_text(str: &str, text_size: f32, position: nalgebra_glm::Vec3, colour: nalgebra_glm::Vec4) -> UIDisplay {
    UIDisplay::UIText(UIText { text: str.to_string(), text_size: text_size, position: position, colour: colour, padding: nalgebra_glm::vec2(0.0, 0.0), bg_colour: nalgebra_glm::vec4(0.0, 0.0, 0.0, 0.0), bg_texture_id: 0 })
}
pub fn ui_text_bg(str: &str, text_size: f32, position: nalgebra_glm::Vec3, colour: nalgebra_glm::Vec4, padding: nalgebra_glm::Vec2, bg_colour: nalgebra_glm::Vec4) -> UIDisplay {
    UIDisplay::UIText(UIText { text: str.to_string(), text_size: text_size, position: position, colour: colour, padding: padding, bg_colour: bg_colour, bg_texture_id: 0 })
}
pub fn ui_text_bg_sprite(str: &str, text_size: f32, position: nalgebra_glm::Vec3, colour: nalgebra_glm::Vec4, padding: nalgebra_glm::Vec2, bg_texture_id: u32, bg_colour: nalgebra_glm::Vec4) -> UIDisplay {
    UIDisplay::UIText(UIText { text: str.to_string(), text_size: text_size, position: position, colour: colour, padding: padding, bg_colour: bg_colour, bg_texture_id: bg_texture_id })
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
                UIElement::UIButton(element) => { self.draw_display(&element.display, ui_shader, text_shader) }
                UIElement::UIDisplay(element) => { self.draw_display(&element, ui_shader, text_shader) }
            }
        }
    }

    pub fn update(&self, scene: &Scene, input_handler: &InputHandler) {
        let mouse_pos = input_handler.get_mouse_position();

        for el in &self.elements {
            match el {
                UIElement::UIButton(el) => { match &el.display {
                    UIDisplay::UISprite(disp) => {
                        if mouse_pos.x > disp.position.x && mouse_pos.x < (disp.position.x + disp.size.x) &&
                           mouse_pos.y > disp.position.y && mouse_pos.y < (disp.position.y + disp.size.y) {
                            if input_handler.get_mouse_button_down(rglfw::MOUSE_BUTTON_1) {
                                (el.on_click_fn)(scene, self);
                            }
                        }
                    }
                    UIDisplay::UIText(disp) => {
                        if mouse_pos.x > (disp.position.x - (disp.padding.x * disp.text_size)) && mouse_pos.x < (disp.position.x + self.get_string_size(&disp.text, disp.text_size).x + (disp.padding.x * disp.text_size)) &&
                           mouse_pos.y > (disp.position.y + ((disp.padding.y * disp.text_size) + (3.0 * disp.text_size))) && mouse_pos.y < (disp.position.y + (self.char_height * disp.text_size) + ((disp.padding.y + (3.0 * disp.text_size)) * disp.text_size)) {
                            if input_handler.get_mouse_button_down(rglfw::MOUSE_BUTTON_1) {
                                (el.on_click_fn)(scene, self);
                            }
                        }
                    }
                } }
                _ => ()
            }
        }
    }

    pub fn draw_string(&self, str: &str, text_size: f32, pos: &nalgebra_glm::Vec3, colour: &nalgebra_glm::Vec4, shader: &Shader) {
            let mut position = *pos;

            for char in str.chars() {
                if char as i32 == 32 {
                    position.x += 3.0 * text_size
                }
                else {
                    self.draw_character(&self.characters[char as usize], text_size, &position, colour, shader);
                    position.x += (self.characters[char as usize].width * text_size) + text_size;
                }
            }
        }
    pub fn draw_string_bg(&self, str: &str, text_size: f32, pos: &nalgebra_glm::Vec3, colour: &nalgebra_glm::Vec4, bg_colour: &nalgebra_glm::Vec4, pad: &nalgebra_glm::Vec2, bg_texture_id: u32, shader: &Shader, text_shader: &Shader) {
        let mut position = *pos;
        let padding = pad * text_size;

        let str_size = self.get_string_size(str, text_size);
        self.draw_sprite(bg_texture_id, &nalgebra_glm::vec3(position.x - padding.x, position.y - padding.y, 0.0),
                        &nalgebra_glm::vec2(str_size.x + (padding.x * 2.0), str_size.y + (3.0 * text_size) + (padding.y * 2.0)),
                        bg_colour, false, shader);

        for char in str.chars() {
            if char as i32 == 32 {
                position.x += 3.0 * text_size
            }
            else {
                self.draw_character(&self.characters[char as usize], text_size, &position, colour, text_shader);
                position.x += (self.characters[char as usize].width * text_size) + text_size;
            }
        }
    }

    fn draw_display(&self, display: &UIDisplay, ui_shader: &Shader, text_shader: &Shader) {
        match display {
            UIDisplay::UISprite(el) => {self.draw_sprite(el.texture_id, &el.position, &el.size, &el.colour, el.using_texture, ui_shader)}
            UIDisplay::UIText(el) => {
                if el.bg_colour.w != 0.0 { self.draw_string_bg(&el.text, el.text_size, &el.position, &el.colour, &el.bg_colour, &el.padding, el.bg_texture_id, ui_shader, text_shader) }
                else { self.draw_string(&el.text, el.text_size, &el.position, &el.colour, text_shader) }
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
    fn draw_character(&self, char: &Character, text_size: f32, position: &nalgebra_glm::Vec3, colour: &nalgebra_glm::Vec4, shader: &Shader) {
        unsafe {
            shader.use_shader();

            gl::Disable(gl::DEPTH_TEST);

            let mut model: nalgebra_glm::Mat4 = nalgebra_glm::identity();
            model = nalgebra_glm::translate(&model, &position);
            model = nalgebra_glm::scale(&model, &nalgebra_glm::vec3(char.width * text_size, self.char_height * text_size, 1.0));
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

    pub fn get_string_size(&self, str: &str, text_size: f32) -> nalgebra_glm::Vec2 {
        let mut size = nalgebra_glm::vec2(0.0, self.char_height * text_size);

        for char in str.chars() {
            if char as i32 == 32 {
                size.x += 3.0 * text_size
            }
            else {
                size.x += (self.characters[char as usize].width * text_size) + text_size;
            }
        }

        size.x -= text_size;

        size
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

    let vertices: [f32; 20] = [
        0.0, 0.0, 0.0,       0.0, 1.0,
         1.0, 0.0, 0.0,    1.0, 1.0,
         1.0,  1.0, 0.0, 1.0, 0.0,
        0.0,  1.0, 0.0,    0.0, 0.0
    ];

    let (vao, vbo, ibo) = create_vao_and_ibo(&vertices, &indices);

    UI { characters: chars, char_height: char_height, elements: elements, vao: vao, ibo: ibo, vbo: vbo }
}