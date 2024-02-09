use rustengine::{ui::{ui_button, ui_sprite, UIElement, UI}, scene::Scene, mesh::Mesh as Obj, window::InputHandler};
use rustengine::glm;

pub fn build() -> (Vec<Obj>, Vec<UIElement>) {
    let mut uiels: Vec<UIElement> = Vec::new();

    uiels.push(ui_button(ui_sprite("blue_grad", glm::vec2(10.0, 10.0), glm::vec3(10.0, 130.0, 0.0)), test_button));
    
    (Vec::new(), uiels)
}

pub fn update(_scene: &mut Scene, _input_handler: &InputHandler, _delta_time: f32) {
    
}

fn test_button(_scene: &Scene, _ui: &UI) {
    println!("clicked");
}