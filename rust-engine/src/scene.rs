use std::fs::read_to_string;

use nalgebra_glm;

use crate::mesh::{ Mesh, mesh_vertices };
use crate::shader::Shader;
use crate::ui::{ ui, UIElement, UI };
use crate::window::{ InputHandler, Window };
use crate::rglfw;
use crate::camera::{ Camera, camera };

pub struct Scene {
    objects: Vec<Mesh>,
    ui: UI,
    camera: Camera,
    update_fn: fn(&mut Scene, &InputHandler, f32),
    build_fn: fn() -> (Vec<Mesh>, Vec<UIElement>),
    cursor_locked: bool,
    fps: i32
}

impl Scene {
    pub fn draw(&self, shader: &Shader, ui_shader: &Shader, text_shader: &Shader) {
        shader.set_uniform_mat4("view", &self.camera.view());

        for obj in &self.objects {
            obj.draw(shader); 
        }

        self.ui.draw(ui_shader, text_shader);
        
        // DEV TOOL
        if !self.cursor_locked { self.ui.draw_string_bg("CURSOR UNLOCKED", 2.0, &nalgebra_glm::vec3(0.0, 0.0, 0.0), &nalgebra_glm::vec4(1.0, 1.0, 1.0, 1.0), &nalgebra_glm::vec4(0.0, 0.0, 1.0, 1.0), &nalgebra_glm::vec2(1.0, 0.0), 0, ui_shader, text_shader) }
    }
    pub fn add_obj(&mut self, obj: Mesh) {
        self.objects.push(obj);
    }
    pub fn update(&mut self, window: &mut Window, input_handler: &InputHandler, delta_time: f32) {
        self.cursor_locked = window.get_cursor_locked();
        self.fps = window.get_fps();

        self.ui.update(self, input_handler);

        if input_handler.get_key_down(rglfw::KEY_R) {
            self.reload();
        }

        if self.cursor_locked { self.camera.update(input_handler, delta_time); }

        if input_handler.get_key_down(rglfw::KEY_GRAVE_ACCENT) && self.cursor_locked { window.set_cursor_locked(false); }
        else if input_handler.get_key_down(rglfw::KEY_GRAVE_ACCENT) { window.set_cursor_locked(true); }

        (self.update_fn)(self, input_handler, delta_time);
    }
    
    fn reload(&mut self) { 
        self.objects.clear();

        let (objs, els) = (self.build_fn)();
        self.objects = objs;
        self.ui = ui(els, "wave-standard", 12.0);
    }
}
pub fn scene(build: fn() -> (Vec<Mesh>, Vec<UIElement>), update: fn(&mut Scene, &InputHandler, f32), pos: nalgebra_glm::Vec3) -> Scene {
    let (objs, elements ) = (build)();
    Scene { objects: objs, ui: ui(elements, "wave-standard", 12.0), 
        camera: camera(pos), update_fn: update, build_fn: build, cursor_locked: false, fps: 0 }
}

pub fn load_level_from_file(path: &str) -> Vec<Mesh> {
        let mut objs: Vec<Mesh> = Vec::new();

        let mut lines = Vec::new();

        for line in read_to_string("./res/levels/".to_owned() + path + ".level").unwrap().lines() {
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
                let uv_x = nalgebra_glm::distance(&nalgebra_glm::vec2(x2, z2), &nalgebra_glm::vec2(x1, z1)) / uv_multiplier;
                let uv_y = y / uv_multiplier;

                let vertices: [f32; 30] = [
                    x1, 0.0, z1, 0.0,   uv_y,
                    x1, y, z1,   0.0,   0.0,
                    x2, 0.0, z2, uv_x,  uv_y,
                    x2, 0.0, z2, uv_x,  uv_y,
                    x2, y, z2,   uv_x,  0.0,
                    x1, y, z1,   0.0,   0.0
                ];
                objs.push(mesh_vertices(&vertices, 30, &lines[i+5]));
                i += 8;
            }
        }

        objs
    }