use rustengine::{ui::{UIElement, UI}, scene::{load_level_from_file, SceneManager}, mesh::Mesh as Obj, window::Window, level::Level, input};
use rustengine::glm;

struct Scene1 {
    move_speed: f32
}

impl SceneManager for Scene1 {
    fn update(&mut self, level: &mut Level, _ui: &mut UI, window: &Window, delta_time: f32) {
        let mut movement = glm::vec3(0.0, 0.0, 0.0);

        if window.get_key(input::KEY_W) {
            movement += level.camera().get_front();
        }   
        if window.get_key(input::KEY_S) {
            movement -= level.camera().get_front();
        }
        if window.get_key(input::KEY_A) {
            movement -= glm::normalize(&glm::cross(&level.camera().get_front(), &glm::vec3(0.0, 1.0, 0.0)));
        }
        if window.get_key(input::KEY_D) {
            movement += glm::normalize(&glm::cross(&level.camera().get_front(), &glm::vec3(0.0, 1.0, 0.0)));
        }

        level.camera_mut().move_position(&(movement * self.move_speed * delta_time));

        level.camera_mut().add_pitch_yaw(&(window.get_mouse_offset()));
        level.camera_mut().update_direction();
    }
}

pub fn build() -> (Vec<Obj>, Vec<UIElement>, Box<dyn SceneManager>) {
    let uiels: Vec<UIElement> = Vec::new();

    //uiels.push(ui_button(ui_text_bg("Test Button", 5.0, glm::vec3(10.0, 100.0, 0.0), glm::vec4(0.0, 0.0, 0.0, 1.0), glm::vec2(1.0, -2.0), glm::vec4(0.0, 0.0, 0.0, 0.5))));
    //uiels.push(ui_button(ui_sprite("blue_grad", glm::vec2(10.0, 10.0), glm::vec3(10.0, 130.0, 0.0)), test_button));
    
    (load_level_from_file("level"), uiels, Box::new(Scene1 {move_speed: 10.0}))
}