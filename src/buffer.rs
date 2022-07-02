use gl::types::GLenum;

use crate::Texture2D;
use crate::textures::State;

pub struct FrameBuffer {
    pub id: u32,
    pub width: u32,
    pub height: u32,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> FrameBuffer {
        unsafe {
            let mut id = 0;
            gl::GenFramebuffers(1, &mut id);
            let buffer = FrameBuffer {
                id,
                width,
                height,
            };
            buffer
        }
    }
    pub fn attach_texture(&self, texture: &Texture2D, target: GLenum) {
        if texture.width != self.width || texture.height != self.height {
            panic!("Texture coordinates are not the same as Framebuffer");
        }
        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, target, gl::TEXTURE_2D,
                                     texture.id, 0);
        }
    }

    pub fn attach_buffer(&self, buffer: &RenderBuffer, attachment: GLenum, target: GLenum) {
        if buffer.width != self.width || buffer.height != self.height {
            panic!("Texture coordinates are not the same as Framebuffer");
        }
        unsafe {
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, attachment, target, buffer.id);
        }
    }

    pub fn assert_status(&self) {
        unsafe {
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer is not complete!");
            }
        }
    }
}

impl State for FrameBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }
    fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

pub struct RenderBuffer {
    pub id: u32,
    pub width: u32,
    pub height: u32,
}

impl RenderBuffer {
    pub fn new(width: u32, height: u32) -> RenderBuffer {
        unsafe {
            let mut id = 0;
            gl::GenRenderbuffers(1, &mut id);
            let buffer = RenderBuffer {
                id,
                width,
                height,
            };

            buffer
        }
    }

    pub fn storage(&self, format: GLenum) {
        unsafe {
            gl::RenderbufferStorage(gl::RENDERBUFFER, format,
                                    self.width as i32, self.height as i32);
        }
    }
}

impl State for RenderBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        }
    }
}