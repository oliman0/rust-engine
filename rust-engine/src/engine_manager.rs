use crate::engine::scene;
use crate::mesh::Mesh;
use crate::scene::Scene;
use crate::ui::UIElement;
use crate::shader::Shader;
use crate::window::{window, InputHandler, Window};
use crate::framebuffer::{FrameBuffer, framebuffer};

pub struct Engine<'a> {
    window: Window,
    input_handler: &'a mut InputHandler,
    scenes: Vec<Scene>,
    active_scene: i32,
    framebuffer: FrameBuffer,
    delta_time: f32
}

impl<'a> Engine<'a> {
    pub fn draw(&self, shader: &Shader, ui_shader: &Shader, text_shader: &Shader) {
        self.framebuffer.bind();
        
        clear_screen();

        if self.active_scene >= 0 && self.active_scene < self.scenes.len() as i32 { self.scenes[self.active_scene as usize].draw(shader, ui_shader, text_shader) }

        self.framebuffer.copy_to_default_buffer();
    }
    pub fn update(&mut self) {
        self.input_handler.wireframe();
        self.window.swap_buffers();
        self.input_handler.reset();
        self.window.poll_events();

        self.delta_time = self.window.get_deltatime();

        if self.active_scene >= 0 && self.active_scene < self.scenes.len() as i32 { self.scenes[self.active_scene as usize].update(&mut self.window, self.input_handler, self.delta_time) }     
    }
    pub fn new_scene(&mut self, build: fn() -> (Vec<Mesh>, Vec<UIElement>), update: fn(&mut Scene, &InputHandler, f32), pos: nalgebra_glm::Vec3) {
        self.scenes.push(scene(build, update, pos))
    }
    pub fn set_active_scene(&mut self, scene: i32) { self.active_scene = scene }
    pub fn should_close(&self) -> bool { self.window.should_close() }
    pub fn get_cursor_locked(&self) -> bool { self.window.get_cursor_locked() }
    pub fn set_cursor_locked(&mut self, locked: bool) { self.window.set_cursor_locked(locked) }
}
pub fn engine(name: &str, scr_w: i32, scr_h: i32, disp_w: i32, disp_h: i32) -> Engine {
    let (win, ih) = window(name, scr_w, scr_h, disp_w, disp_h, 5.0);

    Engine { window: win, input_handler: ih,
         scenes: Vec::new(), active_scene: -1, framebuffer: framebuffer(disp_w, disp_h), delta_time: 0.0 }
}

fn clear_screen() {unsafe {gl::ClearColor(0.07, 0.13, 0.17, 1.0); gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);}}