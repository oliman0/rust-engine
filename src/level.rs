use std::fs::read_to_string;

use nalgebra_glm::*;

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
                let x1 = lines[i+1].parse::<f32>().unwrap();
                let z1 = lines[i+2].parse::<f32>().unwrap();
                let x2 = lines[i+3].parse::<f32>().unwrap();
                let z2 = lines[i+4].parse::<f32>().unwrap();
                let y = lines[i+6].parse::<f32>().unwrap();
                let uv_multiplier = lines[i+7].parse::<f32>().unwrap();
                let uv_x = distance(&vec2(x2, z2), &vec2(x1, z1)) / uv_multiplier;
                let uv_y = y / uv_multiplier;

                let vertices: [f32; 30] = [
                    x1, 0.0, z1, 0.0,   uv_y,
                    x1, y, z1,   0.0,   0.0,
                    x2, 0.0, z2, uv_x,  uv_y,
                    x2, 0.0, z2, uv_x,  uv_y,
                    x2, y, z2,   uv_x,  0.0,
                    x1, y, z1,   0.0,   0.0
                ];
                self.add_obj(mesh::Mesh::new_vertices(&vertices, 30, &lines[i+5]));
                i += 8;
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
                let x1 = lines[i+1].parse::<f32>().unwrap();
                let z1 = lines[i+2].parse::<f32>().unwrap();
                let x2 = lines[i+3].parse::<f32>().unwrap();
                let z2 = lines[i+4].parse::<f32>().unwrap();
                let y = lines[i+6].parse::<f32>().unwrap();
                let uv_multiplier = lines[i+7].parse::<f32>().unwrap();
                let uv_x = distance(&vec2(x2, z2), &vec2(x1, z1)) / uv_multiplier;
                let uv_y = y / uv_multiplier;

                let vertices: [f32; 30] = [
                    x1, 0.0, z1, 0.0,   uv_y,
                    x1, y, z1,   0.0,   0.0,
                    x2, 0.0, z2, uv_x,  uv_y,
                    x2, 0.0, z2, uv_x,  uv_y,
                    x2, y, z2,   uv_x,  0.0,
                    x1, y, z1,   0.0,   0.0
                ];
                self.add_obj(mesh::Mesh::new_vertices(&vertices, 30, &lines[i+5]));
                i += 8;
            }
        }
    }
}
