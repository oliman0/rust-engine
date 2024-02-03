use nalgebra_glm;

use crate::shader::Shader;
use crate::texture::generate_texture;
use crate::vao::create_vao;

pub struct Mesh {
    vao: u32,
    vbo: u32,
    position: nalgebra_glm::Vec3,
    colour: nalgebra_glm::Vec4,
    using_texture: bool,
    texture: u32,
    vert_to_draw: i32
}   

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteTextures(1, &self.texture);
        }
    }
}
impl Mesh {
    pub fn draw(&self, shader: &Shader) {
        unsafe {
            shader.use_shader();

            let mut model: nalgebra_glm::Mat4 = nalgebra_glm::identity();
            model = nalgebra_glm::translate(&model, &self.position);
            shader.set_uniform_mat4("model", &model);
            shader.set_uniform_vec4("colour", &self.colour);
            shader.set_uniform_bool("usingTexture", self.using_texture);

            gl::BindVertexArray(self.vao);
    
            if self.using_texture {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, self.texture);
            }

            gl::DrawArrays(gl::TRIANGLES, 0, self.vert_to_draw);

            gl::BindVertexArray(0);

            if self.using_texture {
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }
    }
    pub fn move_pos(&mut self, vec: nalgebra_glm::Vec3) { self.position += vec; }
    pub fn set_pos(&mut self, vec: nalgebra_glm::Vec3) { self.position = vec; }
}
fn create(pos: nalgebra_glm::Vec3, sizex: f32, sizey: f32, sizez: f32, texture: u32, colour: nalgebra_glm::Vec4, using_texture: bool) -> Mesh { 
    //index data
    let vertices: [f32; 180] = [
        0.0,		 0.0,		-sizez,  0.0, 0.0,
        sizex,	 0.0,		-sizez,  1.0, 0.0,
        sizex,	 sizey,	-sizez,  1.0, 1.0,
        sizex,	 sizey,	-sizez,  1.0, 1.0,
        0.0,		 sizey,	-sizez,  0.0, 1.0,
        0.0,		 0.0,		-sizez,  0.0, 0.0,
        0.0,		 0.0,		 0.0,	  0.0, 0.0,
        sizex,	 0.0,		 0.0,	  1.0, 0.0,
        sizex,	 sizey,	 0.0,	  1.0, 1.0,
        sizex,	 sizey,	 0.0,	  1.0, 1.0,
        0.0,		 sizey,	 0.0,	  0.0, 1.0,
        0.0,		 0.0,		 0.0,	  0.0, 0.0,
        0.0,		 sizey,	 0.0,	  1.0, 1.0,
        0.0,		 sizey,	-sizez,  0.0, 1.0,
        0.0,		 0.0,		-sizez,  0.0, 0.0,
        0.0,		 0.0,		-sizez,  0.0, 0.0,
        0.0,		 0.0,		 0.0,	  1.0, 0.0,
        0.0,		 sizey,	 0.0,	  1.0, 1.0,
        sizex,	 sizey,	 0.0,	  1.0, 1.0,
        sizex,	 sizey,	-sizez,  0.0, 1.0,
        sizex,	 0.0,		-sizez,  0.0, 0.0,
        sizex,	 0.0,		-sizez,  0.0, 0.0,
        sizex,	 0.0,		 0.0,	  1.0, 0.0,
        sizex,	 sizey,	 0.0,	  1.0, 1.0,
        0.0,		 0.0,		-sizez,  0.0, 1.0,
        sizex,	 0.0,		-sizez,  1.0, 1.0,
        sizex,	 0.0,		 0.0,	  1.0, 0.0,
        sizex,	 0.0,		 0.0,	  1.0, 0.0,
        0.0,		 0.0,		 0.0,	  0.0, 0.0,
        0.0,		 0.0,		-sizez,  0.0, 1.0,
        0.0,		 sizey,	-sizez,  0.0, 1.0,
        sizex,     sizey,	-sizez,  1.0, 1.0,
        sizex,     sizey,	 0.0,	  1.0, 0.0,
        sizex,     sizey,	 0.0,	  1.0, 0.0,
        0.0,		 sizey,	 0.0,	  0.0, 0.0,
        0.0,		 sizey,	-sizez,  0.0, 1.0 ];   
    let (vao, vbo) = create_vao(&vertices);
    Mesh {vao: vao, vbo: vbo, position: pos, colour: colour, using_texture: using_texture, texture: texture, vert_to_draw: 32}
}
pub fn mesh_vertices(vertices: &[f32], numofvertices: usize, texture: &str) -> Mesh {
    let (vao, vbo) = create_vao(vertices); 
    Mesh {vao: vao, vbo: vbo, position: nalgebra_glm::vec3(0.0, 0.0, 0.0), colour: nalgebra_glm::vec4(1.0, 1.0, 1.0, 1.0),
          using_texture: true, texture: generate_texture(texture), vert_to_draw: numofvertices as i32/5}
}
pub fn mesh(pos: nalgebra_glm::Vec3, sizex: f32, sizey: f32, sizez: f32, texture: &str) -> Mesh {
    let tex_id = generate_texture(texture);
    create(pos, sizex, sizey, sizez, tex_id, nalgebra_glm::vec4(1.0, 1.0, 1.0, 1.0), true)
}
pub fn mesh_notex(pos: nalgebra_glm::Vec3, sizex: f32, sizey: f32, sizez: f32, colour: nalgebra_glm::Vec4) -> Mesh {
    create(pos, sizex, sizey, sizez, 0, colour, false)
}