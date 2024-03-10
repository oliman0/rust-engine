mod scene1;
mod leveleditor;

use rustengine::{engine_manager::engine, input};
use rustengine::glm;

const SCR_WIDTH: i32 = 1920;
const SCR_HEIGHT: i32 = 1080;
const DISP_WIDTH: i32 = 320;
const DISP_HEIGHT: i32 = 180;

//glm::vec4(0.07, 0.13, 0.17, 1.0)

fn main() {
    let mut engine = engine("OpenGL", SCR_WIDTH, SCR_HEIGHT, SCR_WIDTH, SCR_HEIGHT);
    engine.set_cursor_locked(true);

    engine.new_scene(scene1::build, glm::vec3(5.0, 2.5, 0.0), glm::vec4(0.1, 0.1, 0.1, 1.0));
    engine.new_scene(leveleditor::build, glm::vec3(0.0, 0.0, 0.0), glm::vec4(0.0, 0.0, 0.0, 1.0));
    engine.set_active_scene(0);

    while !engine.should_close() {
        if engine.window.get_key_down(input::KEY_P) && engine.get_active_scene() != 1 { engine.set_active_scene(1); engine.set_cursor_locked(false); }
        else if engine.window.get_key_down(input::KEY_P) { engine.set_active_scene(0); engine.set_cursor_locked(true); }

        engine.update();
        engine.draw();
    }
}