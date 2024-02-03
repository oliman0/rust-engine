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
    pub use crate::ui::{ UIElement, ui_sprite_element, ui_sprite_element_notex, ui_text_element };
    pub use crate::scene::{ Scene, load_level_from_file };
    pub use crate::window::InputHandler;
    pub use crate::rglfw::keys as input;
}

pub use nalgebra_glm as glm;