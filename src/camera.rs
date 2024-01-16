use nalgebra_glm::*;
use crate::rglfw;
use crate::gl_window;

pub const TO_RADIANS: f32 = 180.0/3.1415926;

pub struct Camera {
    position: Vec3,
    up: Vec3,
    front: Vec3,
    look_front: Vec3,
    speed: f32,
    yaw: f32,
    pitch: f32
}

impl Camera {
    pub fn new() -> Self { 
        Self { position: vec3(0.0, 0.0, 0.0),
               up: vec3(0.0, 1.0, 0.0),
               front: vec3(0.0, 0.0, -1.0),
               look_front: vec3(0.0, 0.0, -1.0),
               speed: 10.0,
               yaw: 0.0,
               pitch: 0.0 }
    }
    pub fn update(&mut self, window: &gl_window::Window, delta_time: f32) {
        if window.get_key(rglfw::KEY_W) {
            self.position += (self.speed * delta_time) * self.front;
        }   
        if window.get_key(rglfw::KEY_S) {
            self.position -= (self.speed * delta_time) * self.front;
        }
        if window.get_key(rglfw::KEY_A) {
            self.position -= nalgebra_glm::normalize(&nalgebra_glm::cross(&self.front, &self.up)) * (self.speed * delta_time);
        }
        if window.get_key(rglfw::KEY_D) {
            self.position += nalgebra_glm::normalize(&nalgebra_glm::cross(&self.front, &self.up)) * (self.speed * delta_time);
        }

        if window.is_mouse_input() {
            self.yaw += window.get_mouse_xoffset();
            self.pitch += window.get_mouse_yoffset();
        }

        if self.pitch > 89.0 { self.pitch =  89.0 }
        if self.pitch < -89.0 { self.pitch = -89.0 }

        let mut direction: Vec3 = vec3(0.0, 0.0, 0.0);
        direction.x = (self.yaw / TO_RADIANS).cos() * (self.pitch / TO_RADIANS).cos();
        direction.z = (self.yaw / TO_RADIANS).sin() * (self.pitch / TO_RADIANS).cos();
        self.front = normalize(&direction);
        direction.y = (self.pitch / TO_RADIANS).sin();
        self.look_front = normalize(&direction);
    }
    pub fn view(&self) -> Mat4 {
        nalgebra_glm::look_at(
            &self.position, 
            &(self.position + self.look_front),
            &self.up)
    }
}
