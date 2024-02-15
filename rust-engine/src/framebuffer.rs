use std::ffi::c_void;

pub struct FrameBuffer {
    fbo: u32,
    texture: u32,
    rbo: u32,
    width: i32,
    height: i32,
    scr_width: i32,
    scr_height: i32
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fbo);
            gl::DeleteRenderbuffers(1, &self.rbo);
            gl::DeleteTextures(1, &self.texture);
        }
    }
}
impl FrameBuffer {
    /*pub fn draw(&self, shader: &shader::Shader) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            shader.use_shader();
            gl::BindVertexArray(self.vao);
            gl::Disable(gl::DEPTH_TEST);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            gl::Enable(gl::DEPTH_TEST);
        }
    }*/
    pub fn copy_to_default_buffer(&self) {
        unsafe {
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
            gl::BlitFramebuffer(0, 0, self.width, self.height, 0, 0, self.scr_width, self.scr_height, gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT, gl::NEAREST);
        }
    }
    pub fn bind(&self) { unsafe { gl::Viewport(0, 0, self.width, self.height); gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo); } }
    pub fn unbind(&self) { unsafe { gl::Viewport(0, 0, self.scr_width, self.scr_height); gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0); } }
}
pub fn framebuffer(width:i32, height: i32, scr_width: i32, scr_height: i32) -> FrameBuffer {
    unsafe {
        let mut framebuffer = FrameBuffer {fbo: 0, texture: 0, rbo: 0, width: width, height: height, scr_width: scr_width, scr_height: scr_height};

        gl::GenFramebuffers(1, &mut framebuffer.fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer.fbo); 
        
        gl::GenTextures(1, &mut framebuffer.texture);
        gl::BindTexture(gl::TEXTURE_2D, framebuffer.texture);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, scr_width, scr_height, 0, gl::RGB, gl::UNSIGNED_BYTE, 0 as *const c_void);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, framebuffer.texture, 0);

        gl::GenRenderbuffers(1, &mut framebuffer.rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, framebuffer.rbo);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, scr_width, scr_height);
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, framebuffer.rbo);
            
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("framebuffer failed");
            //return FrameBuffer::new_blank()
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        /*let vertices: [f32; 24] = [
            -1.0,  1.0,  0.0, 1.0,
            -1.0, -1.0,  0.0, 0.0,
            1.0, -1.0,  1.0, 0.0,

            -1.0,  1.0,  0.0, 1.0,
             1.0, -1.0,  1.0, 0.0,
             1.0,  1.0,  1.0, 1.0
        ];

        let mut vbo: u32 = 0;
        gl::GenVertexArrays(1, &mut framebuffer.vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(framebuffer.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(gl::ARRAY_BUFFER, (24 * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void, gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<GLfloat>() as GLsizei,
                                    (2 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);*/

        framebuffer
        }
    }