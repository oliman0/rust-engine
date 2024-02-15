use rustengine::{ui::{ui_button, ui_sprite_notex, UIElement, UI}, scene::{Scene, SceneManager}, mesh::Mesh as Obj, window::Window, level::Level, input};
use rustengine::glm;

struct LevelEditor {
    last_mouse_pos: glm::Vec2,
    zoom: f32,
}

impl SceneManager for LevelEditor {
    fn update(&mut self, level: &mut Level, ui: &mut UI, window: &Window, _delta_time: f32) {
        if window.get_mouse_button_down(input::MOUSE_BUTTON_1) { self.last_mouse_pos = window.get_mouse_position(); }

        if window.get_scroll_wheel_y_offset() != 0.0 {
            self.zoom += window.get_scroll_wheel_y_offset();
            if self.zoom <= 0.0 { self.zoom = 1.0 }
            ui.ui_shader().set_uniform_mat4("view", &glm::scale(&level.camera().view(), &glm::vec3(self.zoom, self.zoom, 1.0)));
        }

        if window.get_mouse_button(input::MOUSE_BUTTON_1) {
            let mouse_pos = window.get_mouse_position();

            level.camera_mut().move_position(& -(glm::vec3(mouse_pos.x - self.last_mouse_pos.x, mouse_pos.y - self.last_mouse_pos.y, 0.0) * 5.0));

            ui.ui_shader().set_uniform_mat4("view", &glm::scale(&level.camera().view(), &glm::vec3(self.zoom, self.zoom, 1.0)));
            ui.text_shader().set_uniform_mat4("view", &level.camera().view());

            self.last_mouse_pos = mouse_pos;
        }
    }
}

pub fn build() -> (Vec<Obj>, Vec<UIElement>, Box<dyn SceneManager>) {
    let mut uiels: Vec<UIElement> = Vec::new();

    let (mut x, mut y, mut i): (f32, f32, i32) = (9.0, 9.0, 0);
    let spacing: f32 = 15.0;
    let (w, h): (i32, i32) = (25, 14);
    while i < w * h { i+=1;
        uiels.push(ui_button(ui_sprite_notex(glm::vec4(0.2, 0.2, 0.2, 1.0), glm::vec2(5.0, 5.0), glm::vec3(x, y, 0.0)), test_button));
        x+=spacing;
        if i % w == 0 { x-=spacing*w as f32; y+=spacing; }
    }
    
    (Vec::new(), uiels, Box::new(LevelEditor { last_mouse_pos: glm::vec2(0.0, 0.0), zoom: 1.0 }))
}

fn test_button(_scene: &Scene, _ui: &UI) {
    println!("clicked");
}