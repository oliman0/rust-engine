use crate::mesh;
use crate::shader;

pub struct Level {
    objects: Vec<mesh::Mesh>
}

impl Level {
    pub fn new() -> Self { Self { objects: Vec::new() } }
    pub fn draw(&self, shader: &shader::Shader) {
        for obj in &self.objects {
            obj.draw(shader); 
        }
    }
    pub fn add_obj(&mut self, obj: mesh::Mesh) {
        self.objects.push(obj);
    }
}
