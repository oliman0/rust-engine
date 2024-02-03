use std::ffi::CString;
use std::ptr;
use std::sync::OnceLock;

use crate::rglfw;
use crate::rglfw::with_c_str;

static mut INPUT_HANDLER: OnceLock<InputHandler> = OnceLock::new();

pub struct InputHandler {
    keys: [bool; 1024],
    hold_keys: [bool; 1024],
    last_x: f32,
    last_y: f32,
    x_offset: f32,
    y_offset: f32,
    mouse_input: bool,
    sensitivity: f32,
    gl_lines: bool
}

impl InputHandler {
    pub fn get_key(&self, key: i32) -> bool { self.hold_keys[key as usize] }
    pub fn get_key_down(&self, key: i32) -> bool { self.keys[key as usize] }
    pub fn get_mouse_xoffset(&self) -> f32 { self.x_offset }
    pub fn get_mouse_yoffset(&self) -> f32 { self.y_offset }
    pub fn is_mouse_input(&self) -> bool { self.mouse_input }
    pub fn reset(&mut self) { self.mouse_input = false; self.keys = [false; 1024]; }
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
fn input_handler(sensitivity: f32) -> InputHandler {
    InputHandler {
        keys: [false; 1024],
        hold_keys: [false; 1024],
        last_x: 0.0,
        last_y: 0.0,
        x_offset: 0.0,
        y_offset: 0.0,
        mouse_input: false,
        sensitivity: sensitivity,
        gl_lines: false
    }
}

pub struct Window {
    window: * mut rglfw::GLFWwindow,
    last_time: f32
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            rglfw::glfwTerminate();
        }
    }
}
impl Window {
    pub fn swap_buffers(&self) {unsafe {glfw::ffi::glfwSwapBuffers(self.window);}}
    pub fn should_close(&self) -> bool {unsafe { if rglfw::glfwWindowShouldClose(self.window) == 1 {true} else {false}}}
    pub fn poll_events(&mut self) { unsafe {rglfw::glfwPollEvents();} }
  
    pub fn get_deltatime(&mut self) -> f32 {
        let current_time = rglfw::get_time();
        let delta_time = current_time - self.last_time;
        self.last_time = current_time;

        delta_time
    }
}
pub fn window(title: &str, scr_width: i32, scr_height: i32, viewport_w: i32, viewport_h: i32, sensitivity: f32) -> (Window, &mut InputHandler) {
    unsafe {

    let _ = INPUT_HANDLER.set(input_handler(sensitivity));

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
            last_time: 0.0
    };
    
    rglfw::glfwMakeContextCurrent(window.window);
    
    rglfw::glfwSetInputMode(window.window, 0x00033001, 0x00034003);

    rglfw::glfwSetKeyCallback(window.window, std::mem::transmute(handle_keys as *const ()));
    rglfw::glfwSetCursorPosCallback(window.window, std::mem::transmute(handle_mouse as *const ()));

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| with_c_str(symbol, |symbol| {
        rglfw::glfwGetProcAddress(symbol)
    }));
    
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    rglfw::glfwGetFramebufferSize(window.window, &mut w, &mut h);
    gl::Viewport(0, 0, viewport_w, viewport_h);

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

    window_user.x_offset = xpos - window_user.last_x;
    window_user.y_offset = window_user.last_y - ypos;
    window_user.last_x = xpos;
    window_user.last_y = ypos;

    window_user.x_offset *= window_user.sensitivity;
    window_user.y_offset *= window_user.sensitivity;

    window_user.mouse_input = true;
}
