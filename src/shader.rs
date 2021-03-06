extern crate gl;


use std::ffi::{CStr, CString};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::ptr;

use gl::types::{self, GLchar, GLint};

use crate::gl::types::GLuint;

pub struct Shader {
    program: GLuint,
}

impl Shader {
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }

    pub fn load_mat4(&self, name: &str, mat: &glam::Mat4) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.program, c_str.as_ptr());

            gl::UniformMatrix4fv(location, 1, gl::FALSE, &mat.to_cols_array()[0]);
        }
    }

    pub fn from_toml(location: &Path) -> Shader {
        let toml = fs::read_to_string(location).unwrap();
        let (vertex, fragment, _config) = dsa_lib::compile_toml(toml.as_str()).unwrap();
        let vert_link = Shader::compile(gl::VERTEX_SHADER, vertex.as_str());
        let frag_link = Shader::compile(gl::FRAGMENT_SHADER, fragment.as_str());
        let handle = Shader::link(frag_link, vert_link);
        return Shader { program: handle };
    }

    pub fn load(location: &Path) -> Shader {
        if !location.exists() {
            panic!("Path {} does not exist", location.to_str().unwrap());
        }
        let name = location.file_name().unwrap().to_str().unwrap();
        let mut str = "".to_owned();
        str.push_str(name);

        let mut frag_path = str.clone();
        frag_path.push_str(".frag");
        let frag = location.join(Path::new(frag_path.as_str()));
        if !frag.exists() {
            panic!("Frag Shader does not exist");
        }

        let mut vert_path = str.clone();
        vert_path.push_str(".vert");
        let vert = location.join(Path::new(vert_path.as_str()));
        if !vert.exists() {
            panic!("Frag Shader does not exist");
        }

        let fragment_src = fs::read_to_string(frag).expect("Failed to read fragment shader");
        let vertex_src = fs::read_to_string(vert).expect("Failed to read vertex shader");

        // println!("Fragment Src: \n{}", fragment_src);
        // println!("Vertex Src: \n{}", vertex_src);

        let vert_link = Shader::compile(gl::VERTEX_SHADER, vertex_src.as_str());
        let frag_link = Shader::compile(gl::FRAGMENT_SHADER, fragment_src.as_str());

        let handle = Shader::link(frag_link, vert_link);

        return Shader { program: handle };
    }

    fn compile(type_: types::GLenum, src: &str) -> u32 {
        unsafe {
            let handle = gl::CreateShader(type_);
            let c_str = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(handle, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(handle);

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut status);
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(
                    handle,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    std::str::from_utf8(&buf)
                        .ok()
                        .expect("ShaderInfoLog not valid utf8")
                );
            }
            handle
        }
    }

    fn link(fragment_shader: u32, vertex_shader: u32) -> u32 {
        unsafe {
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            // check for linking errors
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut status);
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(
                    shader_program,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    std::str::from_utf8(&buf)
                        .ok()
                        .expect("ProgramInfoLog not valid utf8")
                );
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            shader_program
        }
    }
}
