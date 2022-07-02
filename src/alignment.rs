use std::mem;

use gl::types::{GLenum, GLfloat, GLint};

use crate::alignment::Attribute::*;

pub enum Attribute {
    Color,
    Color4,
    Position,
    Normal,
    UV,
}

pub struct Layout {
    pub attributes: Vec<Attribute>,
}


impl Layout {
    pub fn new(attributes: Vec<Attribute>) -> Layout {
        Layout {
            attributes
        }
    }
    pub fn vertex_size(&self) -> u32 {
        let mut sum = 0;
        for x in self.attributes.iter() {
            sum += x.size_bytes();
        }
        sum
    }
}

impl Attribute {
    pub fn size(&self) -> u8 {
        match self {
            Color => 3,
            Color4 => 4,
            Position => 3,
            Normal => 3,
            UV => 2,
        }
    }
    pub fn size_bytes(&self) -> u32 {
        match self {
            Color => 3 * Attribute::float_bytes(),
            Color4 => 4 * Attribute::float_bytes(),
            Position => 3 * Attribute::float_bytes(),
            Normal => 3 * Attribute::float_bytes(),
            UV => 2 * Attribute::float_bytes(),
        }
    }
    pub fn gl_type(&self) -> GLenum {
        match self {
            Color => gl::FLOAT,
            Color4 => gl::FLOAT,
            Position => gl::FLOAT,
            Normal => gl::FLOAT,
            UV => gl::FLOAT,
        }
    }

    pub fn float_bytes() -> u32 {
        mem::size_of::<GLfloat>() as u32
    }
    pub fn int_bytes() -> u32 {
        mem::size_of::<GLfloat>() as u32
    }
}

