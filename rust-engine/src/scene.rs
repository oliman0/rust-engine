use std::fs::read_to_string;

use nalgebra_glm;

use glfw::ffi as glfw;

use crate::level::{level, Level};
use crate::mesh::{ Mesh, mesh_vertices };
use crate::ui::{ ui, UIElement, UI };
use crate::window::Window;

pub trait SceneManager {
    fn update(&mut self, level: &mut Level, ui: &mut UI, window: &Window, delta_time: f32);
}

pub struct Scene {
    // LEVEL
    level: Level,
    ui: UI,
    clear_colour: nalgebra_glm::Vec4,
    // MANAGER
    build_fn: fn() -> (Vec<Mesh>, Vec<UIElement>, Box<dyn SceneManager>),
    manager: Box<dyn SceneManager>,
    fps_tmp: i32
}

impl Scene {
    pub fn draw_level(&self) { self.level.draw(); }
    pub fn draw_ui(&self) { self.ui.draw(); self.ui.draw_string(&self.fps_tmp.to_string(), 5.0, nalgebra_glm::vec3(10.0, 1000.0, 0.0), &nalgebra_glm::vec4(0.0, 0.0, 0.0, 1.0), false); }
    pub fn add_obj(&mut self, obj: Mesh) { self.level.add_obj(obj); }
    pub fn update(&mut self, window: &mut Window, delta_time: f32) {
        self.ui.update(window);

        self.fps_tmp = window.get_fps();

        if window.get_key_down(glfw::KEY_R) {
            self.reload();
        }

        if window.get_key_down(glfw::KEY_GRAVE_ACCENT) && window.get_cursor_locked() { window.set_cursor_locked(false); }
        else if window.get_key_down(glfw::KEY_GRAVE_ACCENT) { window.set_cursor_locked(true); }

        self.manager.update(&mut self.level, &mut self.ui, window, delta_time);
    }
    
    fn reload(&mut self) { 
        let (objs, els, manager) = (self.build_fn)();
        self.level.reload(objs);
        self.ui.reload(els);
        self.manager = manager;
    }
    
    pub fn get_clear_colour(&self) -> &nalgebra_glm::Vec4 { &self.clear_colour }
}
pub fn scene(build: fn() -> (Vec<Mesh>, Vec<UIElement>, Box<dyn SceneManager>), pos: nalgebra_glm::Vec3, col: nalgebra_glm::Vec4, scr_width: f32, scr_height: f32) -> Scene {
    let (objs, elements , manager) = (build)();

    Scene { level: level(objs, pos, scr_width, scr_height), ui: ui(elements, "wave-standard", 12.0, scr_width, scr_height), 
        clear_colour: col, manager: manager, build_fn: build, fps_tmp: 0 }
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