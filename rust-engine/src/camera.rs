use nalgebra_glm;

pub const TO_RADIANS: f32 = 180.0/3.1415926;

pub struct Camera {
    position: nalgebra_glm::Vec3,
    up: nalgebra_glm::Vec3,
    front: nalgebra_glm::Vec3,
    look_front: nalgebra_glm::Vec3,
    yaw: f32,
    pitch: f32
}

impl Camera {
    pub fn set_position(&mut self, pos: nalgebra_glm::Vec3) { self.position = pos; }
    pub fn move_position(&mut self, pos: &nalgebra_glm::Vec3) { self.position += pos }
    pub fn get_position(&self) -> nalgebra_glm::Vec3 { self.position }
    pub fn set_pitch_yaw(&mut self, vec: &nalgebra_glm::Vec2) {
        self.yaw = vec.x;
        self.pitch = vec.y;

        if self.pitch > 89.0 { self.pitch =  89.0 }
        if self.pitch < -89.0 { self.pitch = -89.0 }
    }
    pub fn add_pitch_yaw(&mut self, vec: &nalgebra_glm::Vec2) {
        self.yaw += vec.x;
        self.pitch += vec.y;

        if self.pitch > 89.0 { self.pitch =  89.0 }
        if self.pitch < -89.0 { self.pitch = -89.0 }
    }
    pub fn get_front(&self) -> nalgebra_glm::Vec3 { self.front }

    pub fn update_direction(&mut self) {
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
                yaw: 0.0,
                pitch: 0.0 }
}