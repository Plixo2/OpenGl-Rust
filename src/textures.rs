use std::os::raw::c_void;

use gl::types::GLenum;

pub struct Texture2D {
    pub id: u32,
    pub width: u32,
    pub height: u32,
}

impl Texture2D {
    pub fn tex_parameter(&self, parameter: GLenum, state: GLenum) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, parameter, state as i32);
        }
    }

    pub fn gen_mipmaps(&self) {
        unsafe {
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn put_data(&self, data: *const c_void, format: GLenum) {
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           format as i32,
                           self.width as i32,
                           self.height as i32,
                           0,
                           format,
                           gl::UNSIGNED_BYTE,
                           data);
        }
    }

    pub fn new(width: u32, height: u32) -> Texture2D {
        unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            Texture2D {
                id,
                width,
                height,
            }
        }
    }

    pub fn from_rgb(data: &[u8], width: u32, height: u32) -> Texture2D {
        unsafe {
            let tex = Texture2D::new(width, height);
            tex.bind();
            tex.tex_parameter(gl::TEXTURE_WRAP_S, gl::REPEAT);
            tex.tex_parameter(gl::TEXTURE_WRAP_T, gl::REPEAT);

            tex.tex_parameter(gl::TEXTURE_MIN_FILTER, gl::LINEAR);
            tex.tex_parameter(gl::TEXTURE_MAG_FILTER, gl::LINEAR);

            tex.put_data(&data[0] as *const u8 as *const c_void, gl::RGB);

            tex.gen_mipmaps();

            tex
        }
    }
}

impl State for Texture2D {
    fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

pub trait Renderable {
    fn render();
    fn dispose();
}

pub trait State {
    fn bind(&self);
    fn unbind(&self);
}
