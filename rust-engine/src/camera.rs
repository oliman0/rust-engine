use nalgebra_glm;
use crate::rglfw;
use crate::window::InputHandler;

pub const TO_RADIANS: f32 = 180.0/3.1415926;

pub struct Camera {
    position: nalgebra_glm::Vec3,
    up: nalgebra_glm::Vec3,
    front: nalgebra_glm::Vec3,
    look_front: nalgebra_glm::Vec3,
    speed: f32,
    yaw: f32,
    pitch: f32
}

impl Camera {
    pub fn update(&mut self, input_handler: &InputHandler, delta_time: f32) {
        if input_handler.get_key(rglfw::KEY_W) {
            self.position += (self.speed * delta_time) * self.front;
        }   
        if input_handler.get_key(rglfw::KEY_S) {
            self.position -= (self.speed * delta_time) * self.front;
        }
        if input_handler.get_key(rglfw::KEY_A) {
            self.position -= nalgebra_glm::normalize(&nalgebra_glm::cross(&self.front, &self.up)) * (self.speed * delta_time);
        }
        if input_handler.get_key(rglfw::KEY_D) {
            self.position += nalgebra_glm::normalize(&nalgebra_glm::cross(&self.front, &self.up)) * (self.speed * delta_time);
        }

        if input_handler.is_mouse_input() {
            self.yaw += input_handler.get_mouse_xoffset() * delta_time;
            self.pitch += input_handler.get_mouse_yoffset() * delta_time;
        }

        if self.pitch > 89.0 { self.pitch =  89.0 }
        if self.pitch < -89.0 { self.pitch = -89.0 }

        let mut direction: nalgebra_glm::Vec3 = nalgebra_glm::vec3(0.0, 0.0, 0.0);
        direction.x = (self.yaw / TO_RADIANS).cos() * (self.pitch / TO_RADIANS).cos();
        direction.z = (self.yaw / TO_RADIANS).sin() * (self.pitch / TO_RADIANS).cos();
        self.front = nalgebra_glm::normalize(&direction);
        direction.y = (self.pitch / TO_RADIANS).sin();
        self.look_front = nalgebra_glm::normalize(&direction);
    }
    pub fn view(&self) -> nalgebra_glm::Mat4 {
        nalgebra_glm::look_at(
            &self.position, 
            &(self.position + self.look_front),
            &self.up)
    }
}
pub fn camera(pos: nalgebra_glm::Vec3) -> Camera { 
        Camera { position: pos,
                up: nalgebra_glm::vec3(0.0, 1.0, 0.0),
                front: nalgebra_glm::vec3(0.0, 0.0, -1.0),
                look_front: nalgebra_glm::vec3(0.0, 0.0, -1.0),
                speed: 10.0,
                yaw: 0.0,
                pitch: 0.0 }
}