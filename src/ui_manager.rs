use nalgebra_glm;

//use crate::ui_element::UIElement;
use crate::font::Font;
use crate::shader::Shader;

pub struct UIManager {
    font: Font
}

impl UIManager {
    pub fn new() -> Self {
        Self { font: Font::new("wave-standard", 12.0) }
    }
    pub fn draw(&self, shader: &Shader, text_shader: &Shader) {
        self.draw_string("Testing text rendering", &nalgebra_glm::vec2(0.0, 0.0),
                         &nalgebra_glm::vec3(0.0, 0.0, 0.0), text_shader);
    }

    fn draw_string(&self, str: &str, position: &nalgebra_glm::Vec2, colour: &nalgebra_glm::Vec3, shader: &Shader) {
        self.font.draw_string(str, position, colour, shader);
    }
}
