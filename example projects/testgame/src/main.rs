mod scene1;
mod leveleditor;

use rustengine::{engine_manager::engine, shader::shader, input};
use rustengine::glm;

const TO_RADIANS: f32 = 180.0/3.1415926;

const SCR_WIDTH: i32 = 1920;
const SCR_HEIGHT: i32 = 1080;
const DISP_WIDTH: i32 = 384;
const DISP_HEIGHT: i32 = 216;

fn main() {
    let mut engine = engine("OpenGL", SCR_WIDTH, SCR_HEIGHT, DISP_WIDTH, DISP_HEIGHT);
    engine.set_cursor_locked(true);

    engine.new_scene(scene1::build, scene1::update, glm::vec3(5.0, 2.5, 0.0), glm::vec4(0.07, 0.13, 0.17, 1.0));
    engine.new_scene(leveleditor::build, leveleditor::update, glm::vec3(0.0, 0.0, 0.0), glm::vec4(0.0, 0.0, 0.0, 1.0));
    engine.set_active_scene(0);

    let projection = glm::perspective(110.0 / TO_RADIANS, SCR_WIDTH as f32/SCR_HEIGHT as f32, 0.1, 150.0);
    let ui_projection = glm::ortho(0.0, DISP_WIDTH as f32, 0.0, DISP_HEIGHT as f32, -1.0, 1.0);
    
    let s_shader = shader(
        "./shaders/shader.vert",
        "./shaders/shader.frag");
    let ui_shader = shader(
        "./shaders/ui_shader.vert",
        "./shaders/ui_shader.frag");
    let text_shader = shader(
        "./shaders/ui_shader.vert",
        "./shaders/text_shader.frag"
    );
    s_shader.set_uniform_mat4("projection", &projection);
    ui_shader.set_uniform_mat4("projection", &ui_projection);
    text_shader.set_uniform_mat4("projection", &ui_projection);

    while !engine.should_close() {
        if engine.window.get_key_down(input::KEY_P) && engine.get_active_scene() != 1 { engine.set_active_scene(1); engine.set_cursor_locked(false); }
        else if engine.window.get_key_down(input::KEY_P) { engine.set_active_scene(0); engine.set_cursor_locked(true); }

        engine.update();
        engine.draw(&s_shader, &ui_shader, &text_shader);
    }
}