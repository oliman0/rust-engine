#![allow(dead_code)]

pub mod window;
pub mod shader;
pub mod rglfw;
pub mod camera;
pub mod mesh;
pub mod scene;
pub mod texture;
pub mod framebuffer;
pub mod vao;
pub mod engine_manager;
pub mod ui;

pub use nalgebra_glm as glm;

/*

███╗░░██╗░█████╗░████████╗███████╗░██████╗
████╗░██║██╔══██╗╚══██╔══╝██╔════╝██╔════╝
██╔██╗██║██║░░██║░░░██║░░░█████╗░░╚█████╗░
██║╚████║██║░░██║░░░██║░░░██╔══╝░░░╚═══██╗
██║░╚███║╚█████╔╝░░░██║░░░███████╗██████╔╝
╚═╝░░╚══╝░╚════╝░░░░╚═╝░░░╚══════╝╚═════╝░

Any code commented under DEV TOOLS or DEBUG to be removed later

File Formats:
    blf - Binary Level Format


▀█▀ █▀█ █▀▄ █▀█ ▀
░█░ █▄█ █▄▀ █▄█ ▄

    - Level System Redesign
        - Player obj
        - Player Input independant of engine

</𝒏𝒐𝒕𝒆𝒔> 
*/