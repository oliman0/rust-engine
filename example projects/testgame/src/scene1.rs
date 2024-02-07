use rustengine::scene_builder as engine;
use rustengine::glm;

pub fn build() -> (Vec<engine::Obj>, Vec<engine::UIElement>) {
    let mut uiels: Vec<engine::UIElement> = Vec::new();

    uiels.push(engine::ui_button(engine::ui_text_bg("Test Button", 1.0, glm::vec3(10.0, 100.0, 0.0), glm::vec4(0.0, 0.0, 0.0, 1.0), glm::vec2(1.0, -2.0), glm::vec4(0.0, 0.0, 0.0, 0.5)), test_button));
    //uiels.push(engine::ui_button(engine::ui_sprite("blue_grad", glm::vec2(10.0, 10.0), glm::vec3(10.0, 130.0, 0.0)), test_button));
    
    (engine::load_level_from_file("level"), uiels)
}

pub fn update(_scene: &mut engine::Scene, _input_handler: &engine::InputHandler, _delta_time: f32) {
    
}

fn test_button(_scene: &engine::Scene, _ui: &engine::UI) {
    println!("clicked");
}