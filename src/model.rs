use std::{mem, ptr};
use std::ffi::c_void;
use std::path::Path;

use gl::types::{GLboolean, GLfloat, GLint, GLsizei, GLsizeiptr};
use glam::{Vec2, Vec3, Vec4};
use tobj::Material;

use crate::{Position, Texture2D, UV};
use crate::alignment::{Attribute, Layout};
use crate::alignment::Attribute::Color;
use crate::textures::State;

pub struct Mesh<V> {
    vertices: Vec<V>,
    indices: Vec<u32>,
    textures: Option<Texture2D>,
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
}

impl <T> Model<T> {
    pub fn render(&self) {
        unsafe {
            for x in self.meshes.iter() {
                x.render();
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
            if let Some(texture) = &self.textures {
                // println!("bound {}", texture.id);
                texture.bind();
            }
            gl::ActiveTexture(gl::TEXTURE0);

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

    pub fn from_lists(indices: Vec<u32>, vertices: Vec<V>, layout: Layout) -> Mesh<V> {
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
            textures: None,
        }
    }
}

#[repr(C)]
pub struct TexVertex {
    position: Vec3,
    color: Vec3,
    uv: Vec2,
}


impl Model<TexVertex> {
    pub fn from_textured_tobj(models: Vec<tobj::Model>, materials: Vec<Material>) -> Model<TexVertex> {
        let mut textures = Vec::new();
        for x in materials.iter() {
            let string = format!("res/model/sponza/{}", x.ambient_texture);
            let text_path = Path::new(string.as_str());
            if !text_path.exists() {
                panic!("path not existent");
            }
            println!("loading {:?}",text_path.to_str());
            if !text_path.is_file() {
                textures.push(Texture2D {
                    id: 0,
                    width: 0,
                    height: 0
                });
                continue;
            }
            let img = image::open(&Path::new(text_path.to_str().unwrap())).expect("Failed to load \
            texture");
            let img = img.flipv();
            let data = img.as_bytes();
            let texture = Texture2D::from_rgb(data, img.width(), img.height());
            textures.push(texture);
            // println!("Material {} , {}", x.ambient_texture, text_path.exists());
        }

        let mut meshes: Vec<Mesh<TexVertex>> = Vec::new();
        'model: for model in models.iter() {
            let index = model.mesh.material_id.expect("partial uv not supported");
            let mat = textures.get(index).expect("could not find material");

            let mut vertices: Vec<TexVertex> = Vec::new();
            let mut indices: Vec<u32> = Vec::new();

            for x in model.mesh.indices.iter() {
                indices.push(*x);
            }
            let len = model.mesh.positions.len();

            let mut i = 0;
            let mut texture_index = 0;

            println!("Verts {}", model.mesh.positions.len());
            println!("texts {}", model.mesh.texcoords.len());

            while i < len {
                let x1 = model.mesh.texcoords.get(texture_index);
                let y1 = model.mesh.texcoords.get(texture_index + 1);

                // if x1.is_none() || y1.is_none() {
                //     continue 'model;
                // }

                let val_x;
                if x1.is_none() {
                    val_x = 0.0f32;
                } else {
                    val_x = *x1.unwrap();
                }

                let val_y;
                if y1.is_none() {
                    val_y = 0.0f32;
                } else {
                    val_y = *y1.unwrap();
                }

                vertices.push(TexVertex {
                    position: Vec3::new(model.mesh.positions[i], model.mesh.positions[i + 1],
                                        model.mesh.positions[i + 2]),
                    color: Vec3::new(1.0, 1.0, 1.0),
                    uv: Vec2::new(val_x, val_y),
                });
                texture_index += 2;
                i += 3;
            }

            let mut mesh = Mesh::from_lists(indices, vertices, Layout::new(vec![Position, Color,
                                                                                UV]));
            mesh.textures = Some(Texture2D {
                id: mat.id,
                width: mat.width,
                height: mat.height
            });
            meshes.push(mesh);
        }
        Model {
            meshes
        }
    }
}
