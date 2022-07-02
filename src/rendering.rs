use std::sync::mpsc::Receiver;

use glam::Mat4;
use glfw::*;
use imgui::{CollapsingHeader, Condition, Image, Slider, TextureId, WindowFlags};

use crate::buffer::{RenderBuffer, RenderTarget};
use crate::camera::Camera;
use crate::glfw_bind::ImguiGLFW;
use crate::scene::Scene;

pub struct WindowContainer {
    pub(crate) window: Window,
    pub glfw: Glfw,
    pub event_receiver: Receiver<(f64, WindowEvent)>,
    cursor_x: f64,
    cursor_y: f64,
    pub(crate) delta_x: f64,
    pub(crate) delta_y: f64,
    pub(crate) width: u32,
    pub(crate) height: u32,
    delta_time: f32,
    last_frame: f32,
    pub fps: u32,
}

pub struct RenderPath {
    running: bool,
    pub ui: ImguiGLFW,
    pub ui_context: imgui::Context,
    pub window: WindowContainer,
    scene: Option<Scene>,
}

impl RenderPath {
    pub fn new(width: u32, height: u32) -> RenderPath {
        let mut glfw = init(FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(WindowHint::ContextVersion(4, 5));
        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::Floating(false));
        glfw.window_hint(WindowHint::Resizable(true));

        let (mut window, events) = glfw.create_window(
            width, height,
            "Rust OpenGL", WindowMode::Windowed)
                                       .expect("Failed to create GLFW window.");

        window.set_all_polling(true);
        window.make_current();

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        gl::Viewport::load_with(|s| window.get_proc_address(s) as *const _);

        if glfw.supports_raw_motion() {
            window.set_raw_mouse_motion(true);
        }

        let mut im_gui = imgui::Context::create();
        let im_gui_glfw = ImguiGLFW::new(&mut im_gui, &mut window);

        let mut handle = WindowContainer {
            window,
            glfw,
            cursor_x: 0.0,
            cursor_y: 0.0,
            delta_x: 0.0,
            delta_y: 0.0,
            width,
            height,
            event_receiver: events,
            delta_time: 0.0,
            last_frame: 0.0,
            fps: 0,
        };

        let path = RenderPath {
            running: true,
            ui: im_gui_glfw,
            ui_context: im_gui,
            window: handle,
            scene: None,
        };

        path
    }
    pub fn start(&mut self) {
        {
            let mut target = RenderTarget::new(self.window.width, self.window.height);
            target.new_texture(gl::COLOR_ATTACHMENT0);
            target.new_buffer(gl::DEPTH_STENCIL_ATTACHMENT);

            let scene = Scene::new(&self.window);
            self.scene = Some(scene);
        }
    }

    pub fn projection(&mut self, camera: &Camera) -> Mat4 {
        Mat4::perspective_rh_gl(
            camera.fov.to_radians(),
            self.window.width as f32 / self.window.height as f32,
            camera.near_plane,
            camera.far_plane)
    }

    pub fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let mut scene = self.scene.take().unwrap();
        scene.render(self);

        self.scene = Some(scene);

        // let mut s = self.scene.unwrap();
        // s.render(self);

        self.ui();

        self.window.window.render_context().swap_buffers();
        // self.window.glfw.set_swap_interval(SwapInterval::Sync(1));

        self.window.delta_x = 0.0;
        self.window.delta_y = 0.0;
    }


    pub fn handle_event(&mut self, event: &WindowEvent) {
        if self.window.window.should_close() {
            self.running = false;
        }
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                self.window.window.set_should_close(true);
            }
            WindowEvent::CursorPos(x, y) => {
                let delta_x = self.window.cursor_x - x;
                let delta_y = self.window.cursor_y - y;

                self.window.delta_x = delta_x;
                self.window.delta_y = delta_y;

                self.window.cursor_x = *x;
                self.window.cursor_y = *y;
            }
            _ => {}
        }
    }

    pub fn ui(&mut self) {
        let style = self.ui_context.style_mut();
        style.frame_border_size = 0.0;
        style.child_border_size = 0.0;
        style.popup_border_size = 0.0;
        style.window_border_size = 0.0;
        style.tab_border_size = 0.0;
        style.frame_padding = [0.0; 2];
        style.window_padding = [0.0; 2];

        let ui = self.ui.frame(&mut self.window.window, &mut self.ui_context);

        let aspect = self.window.width as f32 / self.window.height as f32;
        let reduced_height = self.window.height as f32 - (1.0 / aspect) * 200.0;

        let canvas = imgui::Window::new("Buffer")
            .position([200.0, 0.0], Condition::Appearing)
            .size([(self.window.width - 200) as f32, reduced_height],
                  Condition::Appearing);

        let canvas = canvas.flags(WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE |
            WindowFlags::NO_COLLAPSE | WindowFlags::NO_SCROLLBAR | WindowFlags::NO_TITLE_BAR);

        let texture =
            self.scene.as_mut().unwrap().target.get(gl::COLOR_ATTACHMENT0)
                .unwrap().as_texture()
                .unwrap();

        canvas.build(&ui, || {
            let img = Image::new(
                TextureId::new(texture.id as usize),
                [(self.window.width - 200) as f32, reduced_height]);
            let img = img.uv0([0.0, 1.0]);
            let img = img.uv1([1.0, 0.0]);
            img.build(&ui);
        });

        let w = imgui::Window::new("Settings")
            .position([0.0, 0.0], Condition::Appearing)
            .size([200.0, 300.0], Condition::Appearing)
            .resizable(false)
            .movable(false).collapsible(false)
            ;

        w.build(&ui, || {
            let mut x = self.scene.as_mut().unwrap().debug_camera.position.to_array();
            ui.input_float3("input float3", &mut x)
              .build();
            ui.spacing();
            if CollapsingHeader::new("Camera").build(&ui) {
                Slider::new("FOV", 10f32, 90f32).build(&ui, &mut self.scene.as_mut().unwrap()
                                                                     .debug_camera.fov);
                Slider::new("Speed", 0.1f32, 60f32).build(&ui, &mut self.scene.as_mut().unwrap()
                                                                        .debug_camera.speed);
            }
            self.scene.as_mut().unwrap().debug_camera.position.x = x[0];
            self.scene.as_mut().unwrap().debug_camera.position.y = x[1];
            self.scene.as_mut().unwrap().debug_camera.position.z = x[2];
            ui.spacing();
            let current_frame = self.window.glfw.get_time() as f32;
            self.window.delta_time = current_frame - self.window.last_frame;
            self.window.last_frame = current_frame;

            ui.text(format!("{:.2}ms", self.window.delta_time * 1000.0));
            ui.text(format!("{:.0} fps", self.window.fps));
        });

        self.ui.draw(ui, &mut self.window.window);
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn dispose(&mut self) {}
}

pub mod shader_mode {
    pub fn line() {
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
    }

    pub fn fill() {
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }
    }

    pub fn point() {
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT);
        }
    }
}