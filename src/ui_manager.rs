use nalgebra_glm;

use crate::ui_element::UIElement;
use crate::shader::Shader;

pub struct UIManager {
    font_characters: Vec<UIElement>
}

impl UIManager {
    pub fn new() -> Self {
        let mut uimanager = Self { font_characters: Vec::new() };

        let mut i = 0;
        while i < 128 {
            uimanager.font_characters.push(UIElement::new_fontchar(nalgebra_glm::vec3(0.0, 0.0, 0.0), "wave-standard", &i.to_string()));
            i += 1;
        }

        uimanager
    }
    pub fn draw_string(&self, str: &str, shader: &Shader) {
        self.font_characters[65].draw(shader);
    }
}
