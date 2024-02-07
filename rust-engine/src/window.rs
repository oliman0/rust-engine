use std::ffi::CString;
use std::ptr;
use std::sync::OnceLock;

use crate::rglfw;
use crate::rglfw::with_c_str;

static mut INPUT_HANDLER: OnceLock<InputHandler> = OnceLock::new();

pub struct InputHandler {
    keys: [bool; 1024],
    hold_keys: [bool; 1024],
    mouse_buttons: [bool; 12],
    hold_mouse_buttons: [bool; 12],
    mouse_x: f32,
    mouse_y: f32,
    mouse_x_offset: f32,
    mouse_y_offset: f32,
    sensitivity: f32,
    gl_lines: bool,
    viewport_to_window_ratio_x: f32,
    viewport_to_window_ratio_y: f32,
    window_y_offset: i32,
    window_x_offset: i32,
    window_width: i32,
    window_height: i32
}

impl InputHandler {
    pub fn get_key(&self, key: i32) -> bool { self.hold_keys[key as usize] }
    pub fn get_key_down(&self, key: i32) -> bool { self.keys[key as usize] }
    pub fn get_mouse_button(&self, button: i32) -> bool { self.hold_mouse_buttons[button as usize] }
    pub fn get_mouse_button_down(&self, button: i32) -> bool { self.mouse_buttons[button as usize] }
    pub fn get_mouse_position_raw(&self) -> nalgebra_glm::Vec2 { nalgebra_glm::vec2(self.mouse_x, self.mouse_y) }
    pub fn get_mouse_position(&self) -> nalgebra_glm::Vec2 {
        nalgebra_glm::vec2((self.mouse_x - self.window_x_offset as f32) / self.viewport_to_window_ratio_x, 
        ((self.window_height as f32 - self.mouse_y) - self.window_y_offset as f32) / self.viewport_to_window_ratio_y)
    }
    pub fn get_mouse_offset(&self) -> nalgebra_glm::Vec2 { nalgebra_glm::vec2(self.mouse_x_offset, self.mouse_y_offset) }
    pub fn reset(&mut self) { (self.mouse_x_offset, self.mouse_y_offset) = (0.0, 0.0); self.keys = [false; 1024]; self.mouse_buttons = [false; 12]; }
    pub fn wireframe(&mut self) {
        unsafe {
            if self.get_key_down(rglfw::KEY_Q) && !self.gl_lines
	        {
		        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
		        self.gl_lines = !self.gl_lines;
	        }
	        else if self.get_key_down(rglfw::KEY_Q) && self.gl_lines
	        {
		        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
		        self.gl_lines = !self.gl_lines;
	        }
        }
    }
}
fn input_handler(sensitivity: f32, window_width: i32, window_height: i32, viewport_width: i32, viewport_height: i32, framebuffer_width: i32, framebuffer_height: i32) -> InputHandler {
    InputHandler {
        keys: [false; 1024],
        hold_keys: [false; 1024],
        mouse_buttons: [false; 12],
        hold_mouse_buttons: [false; 12],
        mouse_x: 0.0,
        mouse_y: 0.0,
        mouse_x_offset: 0.0,
        mouse_y_offset: 0.0,
        sensitivity: sensitivity,
        gl_lines: false,
        viewport_to_window_ratio_x: window_width as f32 / viewport_width as f32,
        viewport_to_window_ratio_y: window_height as f32 / viewport_height as f32,
        window_x_offset: window_width - framebuffer_width,
        window_y_offset: window_height - framebuffer_height,
        window_width: window_width,
        window_height: window_height
    }
}

pub struct Window {
    window: * mut rglfw::GLFWwindow,
    last_time: f32,
    cursor_locked: bool,
    count_frames: i32,
    last_frame_time: f32,
    fps: i32,
    window_width: i32,
    window_height: i32,
    viewport_width: i32,
    viewport_height: i32
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            rglfw::glfwTerminate();
        }
    }
}
impl Window {
    pub fn swap_buffers(&mut self) {
        unsafe { glfw::ffi::glfwSwapBuffers(self.window); }

        self.count_frames += 1;
        let current_time = rglfw::get_time();

        if current_time - self.last_frame_time >= 1.0 {
            self.fps = self.count_frames;
            self.last_frame_time = current_time;
            self.count_frames = 0;
        }
    }
    pub fn should_close(&self) -> bool {unsafe { if rglfw::glfwWindowShouldClose(self.window) == 1 {true} else {false}}}
    pub fn poll_events(&mut self) { unsafe {rglfw::glfwPollEvents();} }
    pub fn get_cursor_locked(&self) -> bool { self.cursor_locked }
    pub fn set_cursor_locked(&mut self, locked: bool) {
        self.cursor_locked = locked;

        unsafe {
            if locked { rglfw::glfwSetInputMode(self.window, rglfw::CURSOR, rglfw::CURSOR_DISABLED); }
            else { rglfw::glfwSetInputMode(self.window, rglfw::CURSOR, rglfw::CURSOR_NORMAL); }
        }
    }
    pub fn get_deltatime(&mut self) -> f32 {
        let current_time = rglfw::get_time();
        let delta_time = current_time - self.last_time;
        self.last_time = current_time;

        delta_time
    }
    pub fn get_fps(&self) -> i32 { self.fps }
    pub fn get_screen_size(&self) -> nalgebra_glm::Vec2 { nalgebra_glm::vec2(self.viewport_width as f32, self.viewport_height as f32) }
}
pub fn window(title: &str, scr_width: i32, scr_height: i32, viewport_w: i32, viewport_h: i32, sensitivity: f32) -> (Window, &mut InputHandler) {
    unsafe {

    // glfw: initialize and configure
    // ------------------------------
    if rglfw::glfwInit() == 0 {
        println!("Failed to initialize GLFW");
        glfw::ffi::glfwTerminate();
    }
    //Setup GLFW window properties
    //OpenGL version
    rglfw::glfwWindowHint(rglfw::GLFW_CONTEXT_VERSION_MAJOR, 3);
    rglfw::glfwWindowHint(rglfw::GLFW_CONTEXT_VERSION_MINOR, 3);
    // Core profile = No backwards compatibility
    rglfw::glfwWindowHint(rglfw::GLFW_OPENGL_PROFILE, rglfw::GLFW_OPENGL_CORE_PROFILE);
    // Allow forward compatibility
    rglfw::glfwWindowHint(rglfw::GLFW_OPENGL_FORWARD_COMPAT, rglfw::GL_TRUE);
    // Set Window non-resizable
    rglfw::glfwWindowHint(rglfw::GLFW_RESIZABLE, rglfw::GL_FALSE);

    // glfw window creation
    // --------------------
    let ctitle = CString::new(title.as_bytes()).unwrap();
    let window = Window {
            window: rglfw::glfwCreateWindow(scr_width, scr_height, ctitle.as_ptr(), ptr::null_mut(), ptr::null_mut()),
            last_time: 0.0,
            cursor_locked: true,
            count_frames: 0, last_frame_time: 0.0, fps: 0,
            window_width: scr_width, window_height: scr_height, viewport_width: viewport_w, viewport_height: viewport_h
    };
    
    rglfw::glfwMakeContextCurrent(window.window);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| with_c_str(symbol, |symbol| {
        rglfw::glfwGetProcAddress(symbol)
    }));
    
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    rglfw::glfwGetFramebufferSize(window.window, &mut w, &mut h);
    gl::Viewport(0, 0, viewport_w, viewport_h);

    let _ = INPUT_HANDLER.set(input_handler(sensitivity, scr_width, scr_height, viewport_w, viewport_h, w, h));

    rglfw::glfwSetKeyCallback(window.window, std::mem::transmute(handle_keys as *const ()));
    rglfw::glfwSetCursorPosCallback(window.window, std::mem::transmute(handle_mouse as *const ()));
    rglfw::glfwSetMouseButtonCallback(window.window, std::mem::transmute(handle_mouse_buttons as *const ()));

    gl::Enable(gl::DEPTH_TEST);
    gl::DepthFunc(gl::LEQUAL);

    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

    rglfw::glfwSwapInterval(1);

    (window, INPUT_HANDLER.get_mut().unwrap())
}
}

fn handle_keys(window: &mut rglfw::GLFWwindow, key: i32, _code: i32, action: i32, _mode: i32) {
    let window_user: &mut InputHandler = unsafe { INPUT_HANDLER.get_mut().unwrap() };

    if key == rglfw::KEY_ESCAPE && action == rglfw::PRESS {
        unsafe {
            rglfw::glfwSetWindowShouldClose(window, rglfw::GL_TRUE);
        }
    }

    if key >= 0 && key < 1024
	{
		if action == rglfw::PRESS
		{
			window_user.keys[key as usize] = true;
		    window_user.hold_keys[key as usize] = true;
		}
		else if action == rglfw::RELEASE
		{
		    window_user.hold_keys[key as usize] = false;
		}
	}
}

fn handle_mouse(_window: &mut rglfw::GLFWwindow, xpos_in: f64, ypos_in: f64) {
    let window_user: &mut InputHandler = unsafe { INPUT_HANDLER.get_mut().unwrap() };
    
    let xpos = xpos_in as f32;
    let ypos = ypos_in as f32;

    window_user.mouse_x_offset = xpos - window_user.mouse_x;
    window_user.mouse_y_offset = window_user.mouse_y - ypos;
    window_user.mouse_x = xpos;
    window_user.mouse_y = ypos;

    window_user.mouse_x_offset *= window_user.sensitivity;
    window_user.mouse_y_offset *= window_user.sensitivity;
}

fn handle_mouse_buttons(_window: &mut rglfw::GLFWwindow, button: i32, action: i32, _mods: i32) {
    let window_user: &mut InputHandler = unsafe { INPUT_HANDLER.get_mut().unwrap() };

    if button >= 0 && button < 12
	{
		if action == rglfw::PRESS
		{
			window_user.mouse_buttons[button as usize] = true;
		    window_user.hold_mouse_buttons[button as usize] = true;
		}
		else if action == rglfw::RELEASE
		{
		    window_user.hold_mouse_buttons[button as usize] = false;
		}
	}
}