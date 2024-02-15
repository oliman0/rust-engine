
use crate::mesh::Mesh;
use crate::scene::{scene, Scene, SceneManager};
use crate::ui::UIElement;
use crate::window::{window, Window};
use crate::framebuffer::{FrameBuffer, framebuffer};

pub struct Engine<'a> {
    pub window: &'a mut Window,
    scenes: Vec<Scene>,
    active_scene: i32,
    framebuffer: FrameBuffer,
    delta_time: f32
}

impl<'a> Engine<'a> {
    pub fn draw(&self) {
        self.framebuffer.bind();
        
        clear_screen(self.scenes[self.active_scene as usize].get_clear_colour());

        if self.active_scene >= 0 && self.active_scene < self.scenes.len() as i32 { self.scenes[self.active_scene as usize].draw_level() }

        self.framebuffer.copy_to_default_buffer();
        self.framebuffer.unbind();

        if self.active_scene >= 0 && self.active_scene < self.scenes.len() as i32 { self.scenes[self.active_scene as usize].draw_ui() }
    }
    pub fn update(&mut self) {
        self.window.swap_buffers();
        self.window.poll_events();

        self.delta_time = self.window.get_deltatime();

        if self.active_scene >= 0 && self.active_scene < self.scenes.len() as i32 { self.scenes[self.active_scene as usize].update(&mut self.window, self.delta_time) }     
    }
    pub fn new_scene(&mut self, build: fn() -> (Vec<Mesh>, Vec<UIElement>, Box<dyn SceneManager>), pos: nalgebra_glm::Vec3, col: nalgebra_glm::Vec4) {
        self.scenes.push(scene(build, pos, col, self.window.get_window_size().x, self.window.get_window_size().y))
    }
    pub fn set_active_scene(&mut self, scene: i32) { self.active_scene = scene }
    pub fn get_active_scene(&self) -> i32 { self.active_scene }
    pub fn should_close(&self) -> bool { self.window.should_close() }
    pub fn get_cursor_locked(&self) -> bool { self.window.get_cursor_locked() }
    pub fn set_cursor_locked(&mut self, locked: bool) { self.window.set_cursor_locked(locked) }
}
pub fn engine(name: &str, scr_w: i32, scr_h: i32, disp_w: i32, disp_h: i32) -> Engine {
    Engine { window: window(name, scr_w, scr_h, disp_w, disp_h, 5.0),
         scenes: Vec::new(), active_scene: -1, framebuffer: framebuffer(disp_w, disp_h, scr_w, scr_h), delta_time: 0.0 }
}

fn clear_screen(col: &nalgebra_glm::Vec4) {unsafe {gl::ClearColor(col.x, col.y, col.z, col.w); gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);}}