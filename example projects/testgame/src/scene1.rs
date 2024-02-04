use rustengine::scene_builder as engine;
use rustengine::glm;

pub fn build() -> (Vec<engine::Obj>, Vec<engine::UIElement>) {
    let mut uiels: Vec<engine::UIElement> = Vec::new();
    
    (engine::load_level_from_file("level"), uiels)
}

pub fn update(scene: &mut engine::Scene, input_handler: &engine::InputHandler, delta_time: f32) {
    
}