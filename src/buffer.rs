use std::collections::HashMap;
use std::os::raw::c_void;

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

pub struct RenderTarget {
    frame_buffer: FrameBuffer,
    attachments: HashMap<u32, RenderAttachment>,
}

impl RenderTarget {
    pub fn new(width: u32, height: u32) -> RenderTarget {
        RenderTarget {
            frame_buffer: FrameBuffer::new(width, height),
            attachments: HashMap::new(),
        }
    }
    pub fn finish(&self) {
        self.frame_buffer.bind();
        self.frame_buffer.assert_status();
        self.frame_buffer.unbind();
    }
    pub fn bind(&self) {
        self.frame_buffer.bind();
    }
    pub fn unbind(&self) {
        self.frame_buffer.unbind();
    }

    pub fn new_texture(&mut self, target: GLenum) -> &Texture2D {
        self.frame_buffer.bind();
        let texture = Texture2D::new(self.frame_buffer.width, self.frame_buffer.height);
        texture.bind();
        texture.put_data(0 as *const c_void, gl::RGB);
        texture.tex_parameter(gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        texture.tex_parameter(gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        texture.unbind();

        self.attach_texture(texture, target);
        self.frame_buffer.unbind();
        self.get(target).unwrap().as_texture().unwrap()
    }

    pub fn new_buffer(&mut self, target: GLenum) -> &RenderBuffer {
        self.frame_buffer.bind();
        let buffer = RenderBuffer::new(self.frame_buffer.width, self.frame_buffer.height);
        buffer.bind();
        buffer.storage(gl::DEPTH24_STENCIL8);
        buffer.unbind();

        self.attach_buffer(buffer, target);
        self.frame_buffer.unbind();
        self.get(target).unwrap().as_buffer().unwrap()
    }

    pub fn attach_buffer(&mut self, buffer: RenderBuffer, attachment: GLenum) {
        self.frame_buffer.attach_buffer(&buffer, attachment, gl::RENDERBUFFER);
        self.attachments.insert(attachment, RenderAttachment::Buffer(buffer));
    }

    pub fn attach_texture(&mut self, texture: Texture2D, attachment: GLenum) {
        self.frame_buffer.attach_texture(&texture, attachment);
        self.attachments.insert(attachment, RenderAttachment::Texture2D(texture));
    }

    pub fn get(&mut self, target: u32) -> Option<&RenderAttachment> {
        self.attachments.get(&target)
    }
}

pub enum RenderAttachment {
    Buffer(RenderBuffer),
    Texture2D(Texture2D),
}

impl RenderAttachment {
    pub fn as_buffer(&self) -> Option<&RenderBuffer> {
        match self {
            RenderAttachment::Buffer(buffer) => Some(buffer),
            _ => None
        }
    }
    pub fn as_texture(&self) -> Option<&Texture2D> {
        match self {
            RenderAttachment::Texture2D(texture) => Some(texture)
            ,
            _ => None
        }
    }
}