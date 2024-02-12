use rustengine::{ui::{ui_button, ui_text_bg, UIElement, UI}, scene::{Scene, load_level_from_file}, mesh::Mesh as Obj, window::Window};
use rustengine::glm;

pub fn build() -> (Vec<Obj>, Vec<UIElement>) {
    let mut uiels: Vec<UIElement> = Vec::new();

    uiels.push(ui_button(ui_text_bg("Test Button", 1.0, glm::vec3(10.0, 100.0, 0.0), glm::vec4(0.0, 0.0, 0.0, 1.0), glm::vec2(1.0, -2.0), glm::vec4(0.0, 0.0, 0.0, 0.5)), test_button));
    //uiels.push(ui_button(ui_sprite("blue_grad", glm::vec2(10.0, 10.0), glm::vec3(10.0, 130.0, 0.0)), test_button));
    
    (load_level_from_file("level"), uiels)
}

pub fn update(_scene: &mut Scene, _window: &Window, _delta_time: f32) {
    
}

fn test_button(_scene: &Scene, _ui: &UI) {
    println!("clicked");
}