#![allow(unused_assignments)]

mod gl_window;
mod ui_element;
mod shader;
mod rglfw;
mod camera;
mod mesh;
mod level;
mod texture;
mod framebuffer;
mod ui_manager;
mod vao;

const SCR_WIDTH: i32 = 1920;
const SCR_HEIGHT: i32 = 1080;
const DISP_WIDTH: i32 = 384;
const DISP_HEIGHT: i32 = 216;

fn gl_clear() {unsafe {gl::ClearColor(0.07, 0.13, 0.17, 1.0); gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);}}

fn main() {
    let main_window = gl_window::Window::new("OpenGL", SCR_WIDTH, SCR_HEIGHT, DISP_WIDTH, DISP_HEIGHT);
    let ui_manager = ui_manager::UIManager::new();
    let mut level = level::Level::new("level");
    let framebuffer = framebuffer::FrameBuffer::new(DISP_WIDTH, DISP_HEIGHT);
    let mut camera = camera::Camera::new(nalgebra_glm::vec3(5.0, 2.5, 0.0));
    let projection = nalgebra_glm::perspective(110.0 / camera::TO_RADIANS, SCR_WIDTH as f32/SCR_HEIGHT as f32, 0.1, 150.0);
    
    let shader = shader::Shader::new(
        "./shaders/vertex_shader.vert",
        "./shaders/fragment_shader.frag");
    shader.set_uniform_mat4("projection", &projection);

    let (mut delta_time, mut last_time, mut current_time): (f32, f32, f32) = (0.0, 0.0, 0.0);

    while !main_window.should_close() {
        current_time = rglfw::get_time();
        delta_time = current_time - last_time;
        last_time = current_time;
        
        framebuffer.bind();

        gl_clear();
        camera.update(&main_window, delta_time);
        
        level.update(&main_window);
        level.draw(&shader);
        ui_manager.draw_string("", &shader);

        shader.set_uniform_mat4("view", &camera.view());

        framebuffer.copy_to_default_buffer();

        main_window.wireframe();
        
        main_window.swap_buffers();
        main_window.poll_events();
    }
}
