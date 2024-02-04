mod scene1;

use rustengine::engine;
use rustengine::glm;

const TO_RADIANS: f32 = 180.0/3.1415926;

const SCR_WIDTH: i32 = 1920;
const SCR_HEIGHT: i32 = 1080;
const DISP_WIDTH: i32 = 384;
const DISP_HEIGHT: i32 = 216;

fn main() {
    let mut engine = engine::engine("OpenGL", SCR_WIDTH, SCR_HEIGHT, DISP_WIDTH, DISP_HEIGHT);

    engine.new_scene(scene1::build, scene1::update, glm::vec3(5.0, 2.5, 0.0));
    engine.set_active_scene(0);

    let projection = glm::perspective(110.0 / TO_RADIANS, SCR_WIDTH as f32/SCR_HEIGHT as f32, 0.1, 150.0);
    let ui_projection = glm::ortho(0.0, DISP_WIDTH as f32, 0.0, DISP_HEIGHT as f32, -1.0, 1.0);
    
    let shader = engine::shader(
        "./shaders/shader.vert",
        "./shaders/shader.frag");
    let ui_shader = engine::shader(
        "./shaders/ui_shader.vert",
        "./shaders/ui_shader.frag");
    let text_shader = engine::shader(
        "./shaders/ui_shader.vert",
        "./shaders/text_shader.frag"
    );
    shader.set_uniform_mat4("projection", &projection);
    ui_shader.set_uniform_mat4("projection", &ui_projection);
    text_shader.set_uniform_mat4("projection", &ui_projection);

    while !engine.should_close() {
        engine.update();
        engine.draw(&shader, &ui_shader, &text_shader);
    }
}