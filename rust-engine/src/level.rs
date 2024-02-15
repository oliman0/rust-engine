use crate::mesh::Mesh;
use crate::camera::{ Camera, camera };
use crate::shader::{shader, Shader};

pub struct Level {
    objects: Vec<Mesh>,
    camera: Camera,
    projection: nalgebra_glm::Mat4,
    shader: Shader
}

impl Level {
    pub fn draw(&self) {
        self.shader.set_uniform_mat4("view", &self.camera.view());

        for obj in &self.objects {
            obj.draw(&self.shader);
        }
    }

    pub fn add_obj(&mut self, obj: Mesh) {
        self.objects.push(obj);
    }

    pub fn camera(&self) -> &Camera { &self.camera }
    pub fn camera_mut(&mut self) -> &mut Camera { &mut self.camera }
    pub fn shader(&self) -> &Shader { &self.shader }

    pub fn reload(&mut self, objs: Vec<Mesh>) { self.objects = objs; }
}
pub fn level(objs: Vec<Mesh>, pos: nalgebra_glm::Vec3, scr_width: f32, scr_height: f32) -> Level {
    let projection = nalgebra_glm::perspective(110.0 / (180.0/3.1415926), scr_width/scr_height, 0.1, 150.0);
    let shader = shader("./shaders/shader.vert", "./shaders/shader.frag");
    shader.set_uniform_mat4("projection", &projection);

    Level { objects: objs, camera: camera(pos), projection: projection, shader: shader }
}