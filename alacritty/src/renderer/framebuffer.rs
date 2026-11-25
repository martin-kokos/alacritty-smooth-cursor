use crate::gl;
use crate::gl::types::*;

use crate::renderer::rects::Vertex;

use std::{mem, ptr, cell::Cell};

#[derive(Debug)]
pub struct FramebufferRenderer {
    fb:  GLuint,
    tex: GLuint,

    width:  GLsizei,
    height: GLsizei,

    // Framebuffer that was active when the `enable` was called
    enable_fb: Cell<GLuint>,
}

impl FramebufferRenderer {
    pub fn new() -> Self {
        let mut fb:  GLuint = 0;
        let mut tex: GLuint = 0;

        let init_pixels: GLsizei = 128;

        let size: f32 = 1.0;

        unsafe {
            gl::GenFramebuffers(1, &mut fb);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fb);

            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                init_pixels,
                init_pixels,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex, 0
            );

            assert!(
                gl::CheckFramebufferStatus(gl::FRAMEBUFFER)
                == gl::FRAMEBUFFER_COMPLETE
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            let mut attribute_offset = 0;

            // Position.
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as i32,
                attribute_offset as *const _,
            );
            gl::EnableVertexAttribArray(0);
            attribute_offset += mem::size_of::<f32>() * 2;

            // Color.
            gl::VertexAttribPointer(
                1,
                4,
                gl::UNSIGNED_BYTE,
                gl::TRUE,
                mem::size_of::<Vertex>() as i32,
                attribute_offset as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Self{
            fb, tex,
            width:  init_pixels,
            height: init_pixels,
            enable_fb: Cell::new(0 as GLuint),
        }
    } // <-- FramebufferRenderer::new()

    pub fn resize(&mut self, width: i32, height: i32) {
        if width == self.width && height == self.height {
            return
        }

        let mut cur_tex: GLint = 0;
        let mut cur_fb:  GLint = 0;

        self.width  = width;
        self.height = height;

        unsafe {
            gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut cur_fb);
            gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut cur_tex);

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fb);

            gl::BindTexture(gl::TEXTURE_2D, self.tex);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.tex, 0
            );

            gl::BindTexture(gl::TEXTURE_2D, cur_tex as GLuint);
            gl::BindFramebuffer(gl::FRAMEBUFFER, cur_fb as GLuint);
        }
    } // <-- FramebufferRenderer::resize(self, width, height)

    pub fn enable(&self) {
        let mut old_fb: GLint = 0;
        unsafe {
            gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut old_fb);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fb);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.enable_fb.set(old_fb as GLuint);
    } // <-- FramebufferRenderer::enable(self)

    pub fn disable(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.enable_fb.get()); }
    } // <-- FramebufferRenderer::disable(self)

    pub fn get_tex(&self) -> GLuint { self.tex }
}

impl Drop for FramebufferRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.tex);
            gl::DeleteFramebuffers(1, &mut self.fb);
        }
    } // <-- Drop for FramebufferRenderer::drop(self)
}
