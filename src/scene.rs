use std::path::Path;

use glam::{EulerRot, Mat4, Quat, Vec3};
use glfw::{Action, Key, MouseButton};

use crate::buffer::RenderTarget;
use crate::camera::Camera;
use crate::model::{Model, TexVertex, Vertex};
use crate::rendering::{shader_mode, WindowContainer};
use crate::RenderPath;
use crate::shader::Shader;

pub struct Scene {
    pub target: RenderTarget,
    model: Model<TexVertex>,
    shader: Shader,
    pub debug_camera: Camera,
}

impl Scene {
    pub fn new(window: &WindowContainer) -> Scene {
        let mut target = RenderTarget::new(window.width, window.height);
        target.new_texture(gl::COLOR_ATTACHMENT0);
        target.new_buffer(gl::DEPTH_STENCIL_ATTACHMENT);

        let obj_file = "res/model/sponza/sponza.obj";
        let (models, materials) = tobj::load_obj(
            &obj_file,
            &tobj::LoadOptions {
                triangulate: true,
                ignore_points: true,
                single_index: true,
                ignore_lines: true,
            },
        ).unwrap();

        let model: Model<TexVertex> = Model::from_textured_tobj(models,materials.unwrap());

        let shader = Shader::from_toml(Path::new("res/shader/textured.toml"));



        Scene {
            target,
            model,
            shader,
            debug_camera: Camera::new(55.0, 60.0),
        }
    }
    pub fn render(&mut self, renderer: &mut RenderPath) {
        self.target.bind();

        if renderer.window.window.get_mouse_button(MouseButton::Button2) == Action::Press {
            self.debug_camera.yaw -= renderer.window.delta_x * 0.1;
            self.debug_camera.pitch -= renderer.window.delta_y * 0.1;
        }

        unsafe {
            gl::ClearColor(0.1, 0.3, 0.51, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.shader.bind();

        Scene::handle_keys(&renderer.window, &mut self.debug_camera, 0.05);

        let projection = renderer.projection(&self.debug_camera);
        let view = self.debug_camera.matrix();
        let model = Mat4::from_scale_rotation_translation(Vec3::splat(1.0),
                                                          Quat::from_euler(
                                                              EulerRot::XYZ, 0.0, 0.0,
                                                              (renderer.window.glfw.get_time() * 0.00) as f32),
                                                          Vec3::new(0.0, 0.0, -1.0));
        self.shader.load_mat4("projection", &projection);
        self.shader.load_mat4("view", &view);
        self.shader.load_mat4("model", &model);

        shader_mode::fill();
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
        }

        self.model.render();

        self.shader.unbind();

        self.target.unbind();
    }

    pub fn handle_keys(handle: &WindowContainer, camera: &mut Camera, delta_time: f32) {
        let mut camera_speed = camera.speed * delta_time;
        let camera_front = camera.front();
        if handle.window.get_key(Key::LeftControl) == Action::Press {
            camera_speed *= 4.0;
        }

        if handle.window.get_key(Key::W) == Action::Press {
            camera.position -= camera_speed * camera_front;
        }
        if handle.window.get_key(Key::S) == Action::Press {
            camera.position -= -(camera_speed * camera_front);
        }
        if handle.window.get_key(Key::A) == Action::Press {
            camera.position -= -(camera_front.cross(camera.up).normalize() * camera_speed);
        }
        if handle.window.get_key(Key::D) == Action::Press {
            camera.position -= camera_front.cross(camera.up).normalize() * camera_speed;
        }
        if handle.window.get_key(Key::Space) == Action::Press {
            camera.position += camera.up * camera_speed;
        }
        if handle.window.get_key(Key::LeftShift) == Action::Press {
            camera.position -= camera.up * camera_speed;
        }
    }
}

