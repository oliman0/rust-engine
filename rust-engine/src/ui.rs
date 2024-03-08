use std::ffi::c_void;

use gl::DEPTH_TEST;

use glfw::ffi as glfw;

use crate::shader::{shader, Shader};
use crate::texture::{ generate_texture_and_size_path, generate_texture };
use crate::vao::create_vao_and_ibo;
use crate::window::Window;

// Elements
// The Top Layer of Abstraction
pub enum UIElement {
    UIButton(UIButtonElement),
    UIDisplay(UIDisplay)
}

// Displays
// The Lower Level; Holds data for rendering
pub enum UIDisplay {
    UISprite(UISprite),
    UIText(UIText)
}

// UI Window
pub struct UIWindow {
    title: String,
    elements: Vec<UIElement>,
    close_button_hover: bool,
    collapse_button_hover: bool,
    collapsed: bool,
    titlebar_height: f32,
    position: nalgebra_glm::Vec3,
    size: nalgebra_glm::Vec2,
    original_size: nalgebra_glm::Vec2,
    collapsed_width: f32,
    colour: nalgebra_glm::Vec4,
    held: bool,
    last_mouse_position: nalgebra_glm::Vec2
}

impl UIWindow {
    pub fn add_element(&mut self, element: UIElement) { self.elements.push(element); }
    pub fn drop_element(&mut self, id: i32) { self.elements.remove(id as usize); }
}
pub fn ui_window(title: &str, position: nalgebra_glm::Vec3, size: nalgebra_glm::Vec2, colour: nalgebra_glm::Vec4, collapsed_width: f32) -> UIWindow {
    UIWindow { title: title.to_string(),
               elements: Vec::new(),
               close_button_hover: false,
               collapse_button_hover: false,
               collapsed: false,
               titlebar_height: 32.0, position: position, size: size, original_size: size, collapsed_width: collapsed_width, colour: colour, held: false, last_mouse_position: nalgebra_glm::vec2(0.0, 0.0) }
}

// Click Logic Struct
struct ClickBox {
    position1: nalgebra_glm::Vec2,
    position2: nalgebra_glm::Vec2
}

impl ClickBox {
    fn is_colliding(&self, cursor_pos: &nalgebra_glm::Vec2) -> bool {
        cursor_pos > &self.position1 && cursor_pos < &self.position2
    }
}
fn click_box(position1: nalgebra_glm::Vec2, position2: nalgebra_glm::Vec2) -> ClickBox {
    ClickBox { position1: position1, position2: position2 }
}

// Lower Level Element and Display Structs
pub struct UIButtonElement {
    display: UIDisplay,
    click_box: ClickBox,
    click_callback: Option<fn()>,
    clicked: bool
}
pub fn ui_sprite_button(tex_name: &str, size: nalgebra_glm::Vec2, position: nalgebra_glm::Vec3, callback: Option<fn()>) -> UIElement {
    UIElement::UIButton(UIButtonElement { display: ui_sprite(tex_name, size, position), click_box: click_box(position.xy(), position.xy() + size), clicked: false, click_callback: callback })
}
pub fn ui_sprite_button_notex(colour: nalgebra_glm::Vec4, size: nalgebra_glm::Vec2, position: nalgebra_glm::Vec3, callback: Option<fn()>) -> UIElement {
    UIElement::UIButton(UIButtonElement { display: ui_sprite_notex(colour, size, position), click_box: click_box(position.xy(), position.xy() + size), clicked: false, click_callback: callback })
}

pub struct UISprite {
    texture_id: u32,
    colour: nalgebra_glm::Vec4,
    position: nalgebra_glm::Vec3,
    size: nalgebra_glm::Vec2,
    using_texture: bool
}

impl Drop for UISprite {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}
pub fn ui_sprite(tex_name: &str, size: nalgebra_glm::Vec2, position: nalgebra_glm::Vec3) -> UIDisplay {
    let tex_id = generate_texture(tex_name);
    UIDisplay::UISprite(UISprite { texture_id: tex_id, size: size,
            position: position, colour: nalgebra_glm::vec4(1.0, 1.0, 1.0, 1.0), using_texture: true })
}
pub fn ui_sprite_notex(colour: nalgebra_glm::Vec4, size: nalgebra_glm::Vec2, position: nalgebra_glm::Vec3) -> UIDisplay {
    UIDisplay::UISprite(UISprite { texture_id: 0, size: size,
            position: position, colour: colour, using_texture: false })
}

pub struct UIText {
    text: String,
    position: nalgebra_glm::Vec3,
    colour: nalgebra_glm::Vec4,
    text_size: f32,
    padding: nalgebra_glm::Vec2,
    bg_colour: nalgebra_glm::Vec4,
    bg_texture_id: u32,
}
pub fn ui_text(str: &str, text_size: f32, position: nalgebra_glm::Vec3, colour: nalgebra_glm::Vec4) -> UIDisplay {
    UIDisplay::UIText(UIText { text: str.to_string(), text_size: text_size, position: position, colour: colour, padding: nalgebra_glm::vec2(0.0, 0.0), bg_colour: nalgebra_glm::vec4(0.0, 0.0, 0.0, 0.0), bg_texture_id: 0 })
}
pub fn ui_text_bg(str: &str, text_size: f32, position: nalgebra_glm::Vec3, colour: nalgebra_glm::Vec4, padding: nalgebra_glm::Vec2, bg_colour: nalgebra_glm::Vec4) -> UIDisplay {
    UIDisplay::UIText(UIText { text: str.to_string(), text_size: text_size, position: position, colour: colour, padding: padding, bg_colour: bg_colour, bg_texture_id: 0 })
}
pub fn ui_text_bg_sprite(str: &str, text_size: f32, position: nalgebra_glm::Vec3, colour: nalgebra_glm::Vec4, padding: nalgebra_glm::Vec2, bg_texture_id: u32, bg_colour: nalgebra_glm::Vec4) -> UIDisplay {
    UIDisplay::UIText(UIText { text: str.to_string(), text_size: text_size, position: position, colour: colour, padding: padding, bg_colour: bg_colour, bg_texture_id: bg_texture_id })
}

struct Character {
    texture_id: u32,
    width: f32
}

impl Drop for Character {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}
fn character(fname: &str) -> Character {
    let (tex_id, w, _h) = generate_texture_and_size_path(fname);
    Character { texture_id: tex_id, width: w }
}

// UI System and Implementation
pub struct UI {
    // FONT
    characters: Vec<Character>,
    char_height: f32,
    // UI
    elements: Vec<UIElement>,
    windows: Vec<UIWindow>,
    vao: u32,
    vbo: u32,
    ibo: u32,
    window_icons: [u32; 3],
    // PROJECTION & SHADER
    ui_projection: nalgebra_glm::Mat4,
    ui_shader: Shader,
    click_position_offset: nalgebra_glm::Vec2,
    click_scale_offset: f32,
    // INPUT
    cursor_free: bool
}

impl Drop for UI {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ibo);
        }
    }
}
impl UI {
    pub fn draw(&self) {
        for element in &self.elements { 
            match element {
                UIElement::UIButton(element) => { self.draw_display(&element.display) }
                UIElement::UIDisplay(element) => { self.draw_display(&element) }
            }
        }

        for window in &self.windows { self.draw_window(window); }
    }

    pub fn update(&mut self, window: &mut Window) {
        let mouse_pos = (window.get_mouse_position() + self.click_position_offset) / self.click_scale_offset;
        let mouse_pos_static = window.get_mouse_position();

        // 
        // UI WINDOW
        //
        let mut win_close: i32 = -1;
        for (i, win) in self.windows.iter_mut().enumerate() {
            // Close Button
            if mouse_pos_static.x > (win.position.x + (win.size.x - win.titlebar_height)) && mouse_pos_static.x < (win.position.x + win.size.x) &&
               mouse_pos_static.y > (win.position.y + (win.size.y - win.titlebar_height)) && mouse_pos_static.y < (win.position.y + win.size.y) {
                if window.get_mouse_button_down(glfw::MOUSE_BUTTON_1) {
                    win_close = i as i32;
                }
                win.close_button_hover = true;
            }
            else { win.close_button_hover = false; }

            // Minimize/Maximize Button
            if mouse_pos_static.x > (win.position.x + (win.size.x - (win.titlebar_height * 2.0))) && mouse_pos_static.x < (win.position.x + (win.size.x - win.titlebar_height)) &&
               mouse_pos_static.y > (win.position.y + (win.size.y - win.titlebar_height)) && mouse_pos_static.y < (win.position.y + win.size.y) {
                if window.get_mouse_button_down(glfw::MOUSE_BUTTON_1) {
                    if win.collapsed {
                        win.collapsed = false;
                        win.position.y -= win.original_size.y - win.titlebar_height;
                        win.size = win.original_size;
                    }
                    else {
                        win.collapsed = true;
                        win.position.y += win.original_size.y - win.titlebar_height;
                        win.size = nalgebra_glm::vec2(win.collapsed_width, win.titlebar_height);
                    }
                }
                win.collapse_button_hover = true;
            }
            else { win.collapse_button_hover = false; }
            
            // Check if mouse is over window
            if mouse_pos_static.x > win.position.x && mouse_pos_static.x < (win.position.x + win.size.x) &&
               mouse_pos_static.y > win.position.y && mouse_pos_static.y < (win.position.y + win.size.y) {
                self.cursor_free = false;
            }
            else { self.cursor_free = true; }

            // Titlebar Grab
            let titlebar_pos = nalgebra_glm::vec2(win.position.x, win.position.y + (win.size.y - win.titlebar_height));
            if mouse_pos_static.x > titlebar_pos.x && mouse_pos_static.x < (titlebar_pos.x + win.size.x) &&
            mouse_pos_static.y > titlebar_pos.y && mouse_pos_static.y < (titlebar_pos.y + win.titlebar_height) {
                if window.get_mouse_button_down(glfw::MOUSE_BUTTON_1) {
                    win.last_mouse_position = mouse_pos_static;
                    win.held = true;
                }
            }
        
            // Titlebar Move
            if window.get_mouse_button(glfw::MOUSE_BUTTON_1) && win.held {
                win.position += nalgebra_glm::vec2_to_vec3(&(mouse_pos_static - win.last_mouse_position));
                win.last_mouse_position = mouse_pos_static;
            }
            else if win.held {
                win.held = false;
            }

            // Elements
            if !win.collapsed {
                update_elements(&mut win.elements, &(mouse_pos_static - win.position.xy()), window)
            }
        } 
        if win_close != -1 { self.close_window(win_close); }

        //
        // UI ELEMENTS
        //
        if self.cursor_free { update_elements(&mut self.elements, &mouse_pos, window); }
    }

    pub fn draw_string(&self, str: &str, text_size: f32, pos: nalgebra_glm::Vec3, colour: &nalgebra_glm::Vec4, using_view: bool) {
            let mut position = pos;

            for char in str.chars() {
                if char as i32 == 32 {
                    position.x += 3.0 * text_size
                }
                else {
                    self.draw_character(&self.characters[char as usize], text_size, &position, colour, using_view);
                    position.x += (self.characters[char as usize].width * text_size) + text_size;
                }
            }
        }
    pub fn draw_string_bg(&self, str: &str, text_size: f32, pos: nalgebra_glm::Vec3, colour: &nalgebra_glm::Vec4, bg_colour: &nalgebra_glm::Vec4, pad: &nalgebra_glm::Vec2, bg_texture_id: u32, using_view: bool) {
        let mut position = pos;
        let padding = pad * text_size;

        let str_size = self.string_size(str, text_size);
        self.draw_sprite(bg_texture_id, &nalgebra_glm::vec3(position.x - padding.x, position.y - padding.y, 0.0),
                        &nalgebra_glm::vec2(str_size.x + (padding.x * 2.0), str_size.y + (3.0 * text_size) + (padding.y * 2.0)),
                        bg_colour, false, using_view);

        for char in str.chars() {
            if char as i32 == 32 {
                position.x += 3.0 * text_size
            }
            else {
                self.draw_character(&self.characters[char as usize], text_size, &position, colour, using_view);
                position.x += (self.characters[char as usize].width * text_size) + text_size;
            }
        }
    }

    fn draw_window(&self, window: &UIWindow) {
        // Window BG
        if !window.collapsed { self.draw_sprite(0, &window.position, &window.size, &window.colour, false, false); }

        // Titlebar
        self.draw_sprite(0, &nalgebra_glm::vec3(window.position.x, window.position.y + (window.size.y - window.titlebar_height), 0.0), &nalgebra_glm::vec2(window.size.x, window.titlebar_height), &nalgebra_glm::vec4(0.15, 0.59, 0.75, 1.0), false, false);
        // Close Button
        if window.close_button_hover { self.draw_sprite(0, &nalgebra_glm::vec3(window.position.x + (window.size.x - window.titlebar_height), window.position.y + (window.size.y - window.titlebar_height), 0.0), &nalgebra_glm::vec2(window.titlebar_height, window.titlebar_height), &nalgebra_glm::vec4(0.2, 0.2, 0.2, 0.5), false, false); }
        self.draw_sprite(self.window_icons[0], &nalgebra_glm::vec3(window.position.x + (window.size.x - window.titlebar_height), window.position.y + (window.size.y - window.titlebar_height), 0.0), &nalgebra_glm::vec2(window.titlebar_height, window.titlebar_height), &nalgebra_glm::vec4(0.0, 0.0, 0.0, 1.0), true, false);
        // Minimize/Maximize Hover
        if window.collapse_button_hover { self.draw_sprite(0, &nalgebra_glm::vec3(window.position.x + (window.size.x - (window.titlebar_height * 2.0)), window.position.y + (window.size.y - window.titlebar_height), 0.0), &nalgebra_glm::vec2(window.titlebar_height, window.titlebar_height), &nalgebra_glm::vec4(0.2, 0.2, 0.2, 0.5), false, false); }
        // Minimize Button
        if !window.collapsed { self.draw_sprite(self.window_icons[1], &nalgebra_glm::vec3(window.position.x + (window.size.x - (window.titlebar_height * 2.0)), window.position.y + (window.size.y - window.titlebar_height), 0.0), &nalgebra_glm::vec2(window.titlebar_height, window.titlebar_height), &nalgebra_glm::vec4(0.0, 0.0, 0.0, 1.0), true, false); }
        // Maximize Button
        if window.collapsed { self.draw_sprite(self.window_icons[2], &nalgebra_glm::vec3(window.position.x + (window.size.x - (window.titlebar_height * 2.0)), window.position.y + (window.size.y - window.titlebar_height), 0.0), &nalgebra_glm::vec2(window.titlebar_height, window.titlebar_height), &nalgebra_glm::vec4(0.0, 0.0, 0.0, 1.0), true, false); }
        // Window Title
        self.draw_string(&window.title, 2.0, nalgebra_glm::vec3(window.position.x + 6.0, window.position.y + (window.size.y - window.titlebar_height), 0.0), &nalgebra_glm::vec4(0.0, 0.0, 0.0, 1.0), false);

        // Elements
        if !window.collapsed {
            for element in window.elements.iter() { 
                match element {
                    UIElement::UIButton(element) => { self.draw_window_display(&window, &element.display) }
                    UIElement::UIDisplay(element) => { self.draw_window_display(&window, &element) }
                }
            }
        }
    }
    fn draw_window_display(&self, window: &UIWindow, display: &UIDisplay) {
        match display {
            UIDisplay::UISprite(el) => {self.draw_sprite(el.texture_id, &(window.position + el.position), &el.size, &el.colour, el.using_texture, false)}
            UIDisplay::UIText(el) => {
                if el.bg_colour.w != 0.0 { self.draw_string_bg(&el.text, el.text_size, window.position + el.position, &el.colour, &el.bg_colour, &el.padding, el.bg_texture_id, false) }
                else { self.draw_string(&el.text, el.text_size, window.position + el.position, &el.colour, false) }
            }
        }
    }
    pub fn add_window(&mut self, title: &str, position: nalgebra_glm::Vec3, size: nalgebra_glm::Vec2, colour: nalgebra_glm::Vec4) -> &mut UIWindow {
        let window = self.windows.len();
        
        self.windows.push(ui_window(title, position, size, colour, self.string_size(title, 2.0).x + 86.0));

        &mut self.windows[window]
    }
    pub fn close_window(&mut self, id: i32) {
        self.windows.remove(id as usize);
        self.cursor_free = true;
    }

    fn draw_display(&self, display: &UIDisplay) {
        match display {
            UIDisplay::UISprite(el) => {self.draw_sprite(el.texture_id, &el.position, &el.size, &el.colour, el.using_texture, true)}
            UIDisplay::UIText(el) => {
                if el.bg_colour.w != 0.0 { self.draw_string_bg(&el.text, el.text_size, el.position, &el.colour, &el.bg_colour, &el.padding, el.bg_texture_id, true) }
                else { self.draw_string(&el.text, el.text_size, el.position, &el.colour, true) }
            }
        }
    }
    fn draw_sprite(&self, texture_id: u32, position: &nalgebra_glm::Vec3, size: &nalgebra_glm::Vec2, colour: &nalgebra_glm::Vec4, using_texture: bool, using_view: bool) {
        unsafe {
            self.ui_shader.use_shader();

            gl::Disable(gl::DEPTH_TEST);

            let mut model: nalgebra_glm::Mat4 = nalgebra_glm::identity();
            model = nalgebra_glm::translate(&model, &position);
            model = nalgebra_glm::scale(&model, &nalgebra_glm::vec3(size.x, size.y, 1.0));
            self.ui_shader.set_uniform_mat4("model", &model);
            self.ui_shader.set_uniform_vec4("colour", &colour);
            self.ui_shader.set_uniform_bool("usingTexture", using_texture);
            self.ui_shader.set_uniform_bool("addColour", false);
            self.ui_shader.set_uniform_bool("useView", using_view);

            gl::BindVertexArray(self.vao);

            if using_texture {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
            }

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            if using_texture {
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }

            gl::Enable(DEPTH_TEST);
        }
    }
    fn draw_character(&self, char: &Character, text_size: f32, position: &nalgebra_glm::Vec3, colour: &nalgebra_glm::Vec4, using_view: bool) {
        unsafe {
            self.ui_shader.use_shader();

            gl::Disable(gl::DEPTH_TEST);

            let mut model: nalgebra_glm::Mat4 = nalgebra_glm::identity();
            model = nalgebra_glm::translate(&model, &position);
            model = nalgebra_glm::scale(&model, &nalgebra_glm::vec3(char.width * text_size, self.char_height * text_size, 1.0));
            self.ui_shader.set_uniform_mat4("model", &model);
            self.ui_shader.set_uniform_vec4("colour", &colour);
            self.ui_shader.set_uniform_bool("usingTexture", true);
            self.ui_shader.set_uniform_bool("addColour", true);
            self.ui_shader.set_uniform_bool("useView", using_view);

            gl::BindVertexArray(self.vao);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, char.texture_id);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            gl::BindTexture(gl::TEXTURE_2D, 0);

            gl::Enable(DEPTH_TEST);
        }
    }

    pub fn string_size(&self, str: &str, text_size: f32) -> nalgebra_glm::Vec2 {
        let mut size = nalgebra_glm::vec2(0.0, self.char_height * text_size);

        for char in str.chars() {
            if char as i32 == 32 {
                size.x += 3.0 * text_size
            }
            else {
                size.x += (self.characters[char as usize].width * text_size) + text_size;
            }
        }

        size.x -= text_size;

        size
    }

    pub fn reload(&mut self, elements: Vec<UIElement>) { self.elements = elements; }

    pub fn ui_shader(&self) -> &Shader { &self.ui_shader }

    pub fn set_position_offset(&mut self, pos: nalgebra_glm::Vec2) { self.click_position_offset = pos }
    pub fn set_scale_offset(&mut self, scale: f32) { self.click_scale_offset = scale }

    pub fn get_cursor_free(&self) -> bool { self.cursor_free }
}
pub fn ui(elements: Vec<UIElement>, fname: &str, char_height: f32, projection_width: f32, projection_height: f32) -> UI {
    let mut chars: Vec<Character> = Vec::new();

    let mut i = 0;
    while i < 128 {
        chars.push(character(&("fonts/".to_owned() + fname + "/" + &i.to_string())));
        i+=1;
    }

    let indices: [i32; 6] = [
        0, 1, 2,
        2, 3, 0
    ];

    let vertices: [f32; 20] = [
        0.0, 0.0, 0.0,       0.0, 1.0,
         1.0, 0.0, 0.0,    1.0, 1.0,
         1.0,  1.0, 0.0, 1.0, 0.0,
        0.0,  1.0, 0.0,    0.0, 0.0
    ];

    let (vao, vbo, ibo) = create_vao_and_ibo(&vertices, &indices);

    let projection = nalgebra_glm::ortho(0.0, projection_width, 0.0, projection_height, -1.0, 1.0);
    let ushader = shader("./shaders/ui_shader.vert", "./shaders/ui_shader.frag");
    ushader.set_uniform_mat4("projection", &projection);
    ushader.set_uniform_mat4("view", &nalgebra_glm::identity());

    UI { characters: chars, char_height: char_height, elements: elements, windows: Vec::new(), vao: vao, ibo: ibo, vbo: vbo,
         window_icons: [generate_texture("close"), generate_texture("minimize"), generate_texture("maximize")],
         ui_projection: projection,
         ui_shader: ushader, click_position_offset: nalgebra_glm::vec2(0.0, 0.0), click_scale_offset: 1.0,
         cursor_free: true }
}

fn update_elements(elements: &mut Vec<UIElement>, mouse_pos: &nalgebra_glm::Vec2, window: &Window) {
    for el in elements.iter_mut() {
        match el {
            UIElement::UIButton(el) => {
                if el.click_box.is_colliding(mouse_pos) {
                    if window.get_mouse_button_down(glfw::MOUSE_BUTTON_1) {
                        el.clicked = true;
                        if el.click_callback.is_some() { (el.click_callback.unwrap())() }
                    }
                    else {
                        el.clicked = false;
                    }
                }
                else if el.clicked { el.clicked = false; }
            } 
            _ => ()
        }
    }
}