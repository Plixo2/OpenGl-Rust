use std::{mem, ptr};
use std::ffi::c_void;

use gl::types::{GLboolean, GLfloat, GLint, GLsizei, GLsizeiptr};
use glam::{Vec3, Vec4};

use crate::alignment::{Attribute, Layout};
use crate::Position;

pub struct Mesh<V> {
    vertices: Vec<V>,
    indices: Vec<u32>,
    objects_count: i32,
    vertex_array_object: u32,
    vertex_buffer_object: u32,
    element_buffer_object: u32,
}

pub struct Model<V> {
    pub meshes: Vec<Mesh<V>>,
}

impl Model<Vertex> {
    pub fn from_tobj(models: Vec<tobj::Model>) -> Model<Vertex> {
        let mut meshes: Vec<Mesh<Vertex>> = Vec::new();
        for model in models.iter() {
            let mut vertices: Vec<Vertex> = Vec::new();
            let mut indices: Vec<u32> = Vec::new();

            for x in model.mesh.indices.iter() {
                indices.push(*x);
            }
            let len = model.mesh.positions.len();
            let mut i = 0;
            while i < len {
                vertices.push(Vertex {
                    position: Vec3::new(model.mesh.positions[i], model.mesh.positions[i + 1], model.mesh.positions[i + 2])
                });
                i += 3;
            }

            meshes.push(Mesh::from_lists(indices, vertices, Layout::new(vec![Position])));
        }
        Model {
            meshes
        }
    }

    pub fn render(&self) {
        unsafe {
            for x in self.meshes.iter() {
                gl::BindVertexArray(x.vertex_array_object);
                gl::DrawElements(gl::TRIANGLES,
                                 x.objects_count,
                                 gl::UNSIGNED_INT,
                                 ptr::null());
            }
        }
    }

    pub fn delete(&self) {
        for x in self.meshes.iter() {
            x.delete();
        }
    }
}

#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
}

impl<V> Mesh<V> {
    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array_object);
            gl::DrawElements(gl::TRIANGLES,
                             self.objects_count,
                             gl::UNSIGNED_INT,
                             ptr::null());
        }
    }
    pub fn delete(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vertex_array_object);
            gl::DeleteBuffers(1, &self.vertex_buffer_object);
            gl::DeleteBuffers(1, &self.element_buffer_object);
        }
    }

    pub(crate) fn from_lists(indices: Vec<u32>, vertices: Vec<V>, layout: Layout) -> Mesh<V> {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;
        unsafe {
            let vertex_size = layout.vertex_size();
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() as u32 * vertex_size) as GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() as u32 * Attribute::int_bytes()) as GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            let mut offset = 0;
            for (i, attrib) in layout.attributes.iter().enumerate() {
                gl::VertexAttribPointer(
                    i as u32,
                    attrib.size() as i32,
                    attrib.gl_type(),
                    gl::FALSE,
                    vertex_size as GLsizei,
                    offset as *const c_void,
                );
                offset += attrib.size_bytes();
                gl::EnableVertexAttribArray(i as u32);
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        Mesh {
            objects_count: indices.len() as i32,
            indices,
            vertices,
            vertex_array_object: vao,
            vertex_buffer_object: vbo,
            element_buffer_object: ebo,
        }
    }
}