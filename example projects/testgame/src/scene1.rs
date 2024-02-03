use rustengine::scene_builder as engine;
use rustengine::glm;

pub fn build() -> (Vec<engine::Obj>, Vec<engine::UIElement>) {
    let mut uiels: Vec<engine::UIElement> = Vec::new();

    uiels.push(engine::UIElement::UISpriteElement(engine::ui_sprite_element("largecheck", glm::vec2(10.0, 10.0), glm::vec3(100.0, 100.0, 0.0))));
    uiels.push(engine::UIElement::UITextElement(engine::ui_text_element("The quick brown fox", glm::vec3(100.0, 150.0, 0.0), glm::vec4(0.0, 0.0, 0.0, 1.0))));
    
    (engine::load_level_from_file("level"), uiels)
}

pub fn update(scene: &mut engine::Scene, input_handler: &engine::InputHandler, delta_time: f32) {
    
}

pub fn ui_update(scene: &mut engine::Scene, input_handler: &engine::InputHandler) {

}