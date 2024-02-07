#![allow(dead_code)]

mod window;
mod shader;
mod rglfw;
mod camera;
mod mesh;
mod scene;
mod texture;
mod framebuffer;
mod vao;
mod engine_manager;
mod ui;

pub mod engine {
    pub use crate::engine_manager::{ Engine, engine };
    pub use crate::scene::{ Scene, scene };
    pub use crate::shader::{ Shader, shader };
}

pub mod scene_builder {
    pub use crate::mesh::Mesh as Obj;
    pub use crate::ui::{ UI, UIElement, UIDisplay, ui_sprite, ui_sprite_notex, ui_text, ui_text_bg, ui_text_bg_sprite, ui_button };
    pub use crate::scene::{ Scene, load_level_from_file };
    pub use crate::window::InputHandler;
    pub use crate::rglfw::keys as input;
}

pub use nalgebra_glm as glm;

/*

███╗░░██╗░█████╗░████████╗███████╗░██████╗
████╗░██║██╔══██╗╚══██╔══╝██╔════╝██╔════╝
██╔██╗██║██║░░██║░░░██║░░░█████╗░░╚█████╗░
██║╚████║██║░░██║░░░██║░░░██╔══╝░░░╚═══██╗
██║░╚███║╚█████╔╝░░░██║░░░███████╗██████╔╝
╚═╝░░╚══╝░╚════╝░░░░╚═╝░░░╚══════╝╚═════╝░

Any code commented under DEV TOOLS or DEBUG to be removed later


▀█▀ █▀█ █▀▄ █▀█ ▀
░█░ █▄█ █▄▀ █▄█ ▄

    - Level System Redesign
        - Player obj
        - Player Input independant of engine

</𝒏𝒐𝒕𝒆𝒔> 
*/