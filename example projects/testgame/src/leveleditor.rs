use rustengine::{ui::{ui_button, ui_sprite_notex, UIElement, UI}, scene::Scene, mesh::Mesh as Obj, window::Window};
use rustengine::glm;

pub fn build() -> (Vec<Obj>, Vec<UIElement>) {
    let mut uiels: Vec<UIElement> = Vec::new();

    let (mut x, mut y, mut i): (f32, f32, i32) = (9.0, 9.0, 0);
    let spacing: f32 = 15.0;
    let (w, h): (i32, i32) = (25, 14);
    while i < w * h { i+=1;
        uiels.push(ui_button(ui_sprite_notex(glm::vec4(0.2, 0.2, 0.2, 1.0), glm::vec2(5.0, 5.0), glm::vec3(x, y, 0.0)), test_button));
        x+=spacing;
        if i % w == 0 { x-=spacing*w as f32; y+=spacing; }
    }
    
    (Vec::new(), uiels)
}

pub fn update(_scene: &mut Scene, _window: &Window, _delta_time: f32) {
    
}

fn test_button(_scene: &Scene, _ui: &UI) {
    println!("clicked");
}