use std::mem;
use std::os::raw::c_void;
use std::ptr;

use gl::types::*;
use nalgebra_glm::*;

use crate::shader::Shader;
use crate::texture;

pub struct Sprite {
    vao: u32,
    ibo: u32,
    i_count: i32,
    //v_count: i32,
    position: Vec3,
    colour: Vec4,
    using_texture: bool,
    texture: u32
}   

impl Sprite {
    pub fn new_notex(pos: Vec3, sizex: f32, sizey: f32, colour: Vec4) -> Self { 
        unsafe {
            //index data
            let indices: [i32; 6] = [
                0, 1, 2,
                2, 3, 0
            ];

            //vertex data
            let vertices: [f32; 12] = [
                0.0, 0.0, 0.0, 
                 sizex, 0.0, 0.0, 
                 sizex,  sizey, 0.0,
                0.0,  sizey, 0.0
            ];

            let (mut vbo, mut ibo, mut vao) = (0, 0, 0);
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut ibo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLint>()) as GLsizeiptr,
                &indices[0] as *const i32 as *const c_void,
                gl::STATIC_DRAW);
            
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            gl::EnableVertexAttribArray(0);

            // note that this is allowed, the call to gl::VertexAttribPointer registered vbo as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            // You can unbind the vao afterwards so other vao calls won't accidentally modify this vao, but this rarely happens. Modifying other
            // vaos requires a call to glBindVertexArray anyways so we generally don't unbind vaos (nor vbos) when it's not directly necessary.
            gl::BindVertexArray(0);

            Self {vao: vao, ibo: ibo, i_count: 6, position: pos, colour: colour, using_texture: false, texture: 0}

        } 
    }
    pub fn new(pos: Vec3, sizex: f32, sizey: f32, texture: &str) -> Self { 
        unsafe {
            //index data
            let indices: [i32; 6] = [
                0, 1, 2,
                2, 3, 0
            ];

            //vertex data
            let vertices: [f32; 12] = [
                0.0, 0.0, 0.0, 
                 sizex, 0.0, 0.0, 
                 sizex,  sizey, 0.0,
                0.0,  sizey, 0.0
            ];

            let (mut vbo, mut ibo, mut vao) = (0, 0, 0);
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut ibo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLint>()) as GLsizeiptr,
                &indices[0] as *const i32 as *const c_void,
                gl::STATIC_DRAW);
            
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            gl::EnableVertexAttribArray(0);

            // note that this is allowed, the call to gl::VertexAttribPointer registered vbo as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            // You can unbind the vao afterwards so other vao calls won't accidentally modify this vao, but this rarely happens. Modifying other
            // vaos requires a call to glBindVertexArray anyways so we generally don't unbind vaos (nor vbos) when it's not directly necessary.
            gl::BindVertexArray(0);

            Self {vao: vao, ibo: ibo, i_count: 6, position: pos, colour: vec4(1.0, 1.0, 1.0, 1.0), using_texture: true, texture: texture::generate_texture(texture)}

        } 
    }
    pub fn draw(&self, shader: &Shader) {
        unsafe {
            shader.use_shader();

            let mut model: Mat4 = nalgebra_glm::identity();
            model = nalgebra_glm::translate(&model, &self.position);
            shader.set_uniform_mat4("model", &model);
            shader.set_uniform_vec4("colour", &self.colour);
            shader.set_uniform_bool("usingTexture", self.using_texture);

            gl::BindVertexArray(self.vao);

            if self.using_texture {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, self.texture);
            }

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::DrawElements(gl::TRIANGLES, self.i_count, gl::UNSIGNED_INT, 0 as *const c_void);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            if self.using_texture {
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }
    }
    pub fn move_pos(&mut self, vec: nalgebra_glm::Vec3) { self.position += vec; }
    pub fn set_pos(&mut self, vec: nalgebra_glm::Vec3) { self.position = vec; }
}
