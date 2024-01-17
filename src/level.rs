use std::fs::read_to_string;

use crate::mesh;
use crate::shader;
use crate::gl_window;
use crate::rglfw;

pub struct Level {
    objects: Vec<mesh::Mesh>,
    levelname: String
}

impl Level {
    pub fn new(levelname: &str) -> Self {
        let mut level = Self { objects: Vec::new(), levelname: levelname.to_string() };
        level.load(levelname);
        level
    }
    pub fn draw(&self, shader: &shader::Shader) {
        for obj in &self.objects {
            obj.draw(shader); 
        }
    }
    pub fn add_obj(&mut self, obj: mesh::Mesh) {
        self.objects.push(obj);
    }
    pub fn update(&mut self, window: &gl_window::Window) {
        if window.get_key_down(rglfw::KEY_R) {
            self.objects.clear();
            self.reload();
        }
    }
    fn load(&mut self, path: &str) {
        let mut lines = Vec::new();

        for line in read_to_string("C:\\Users\\FiercePC\\projects\\rustgraphics\\res\\levels\\".to_owned() + path + ".level").unwrap().lines() {
            lines.push(line.to_string());
        }

        let mut i = 0;
        while i < lines.len() {
            if lines[i] == "[WALL]" {
                let vertices: [f32; 30] = [
                    lines[i+1].parse::<f32>().unwrap(), 0.0, lines[i+2].parse::<f32>().unwrap(), 0.0, 0.0,
                    lines[i+1].parse::<f32>().unwrap(), 10.0, lines[i+2].parse::<f32>().unwrap(), 0.0, 1.0,
                    lines[i+3].parse::<f32>().unwrap(), 0.0, lines[i+4].parse::<f32>().unwrap(), 1.0, 0.0,
                    lines[i+3].parse::<f32>().unwrap(), 0.0, lines[i+4].parse::<f32>().unwrap(), 1.0, 0.0,
                    lines[i+3].parse::<f32>().unwrap(), 10.0, lines[i+4].parse::<f32>().unwrap(), 1.0, 1.0,
                    lines[i+1].parse::<f32>().unwrap(), 10.0, lines[i+2].parse::<f32>().unwrap(), 0.0, 1.0
                ];
                self.add_obj(mesh::Mesh::new_vertices(&vertices, 30, &lines[i+5]));
                i += 6;
            }
        }
    }
    fn reload(&mut self) {
        let mut lines = Vec::new();

        for line in read_to_string("C:\\Users\\FiercePC\\projects\\rustgraphics\\res\\levels\\".to_owned() + self.levelname.as_str() + ".level").unwrap().lines() {
            lines.push(line.to_string());
        }

        let mut i = 0;
        while i < lines.len() {
            if lines[i] == "[WALL]" {
                let vertices: [f32; 30] = [
                    lines[i+1].parse::<f32>().unwrap(), 0.0, lines[i+2].parse::<f32>().unwrap(), 0.0, 0.0,
                    lines[i+1].parse::<f32>().unwrap(), 10.0, lines[i+2].parse::<f32>().unwrap(), 0.0, 1.0,
                    lines[i+3].parse::<f32>().unwrap(), 0.0, lines[i+4].parse::<f32>().unwrap(), 1.0, 0.0,
                    lines[i+3].parse::<f32>().unwrap(), 0.0, lines[i+4].parse::<f32>().unwrap(), 1.0, 0.0,
                    lines[i+3].parse::<f32>().unwrap(), 10.0, lines[i+4].parse::<f32>().unwrap(), 1.0, 1.0,
                    lines[i+1].parse::<f32>().unwrap(), 10.0, lines[i+2].parse::<f32>().unwrap(), 0.0, 1.0
                ];
                self.add_obj(mesh::Mesh::new_vertices(&vertices, 30, &lines[i+5]));
                i += 6;
            }
        }
    }
}
