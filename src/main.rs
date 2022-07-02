extern crate core;
extern crate gl;
extern crate glfw;

use std::{mem, ptr, thread};
use std::collections::VecDeque;
use std::fmt::format;
use std::os::raw::c_void;
use std::path::Path;
use std::sync::{Arc, mpsc, Mutex};
use std::time::{Duration, Instant};

use glam::*;
use glfw::{Action, Context, CursorMode, Glfw, Key, MouseButton, OpenGlProfileHint, SwapInterval, WindowHint};
use glfw::Key::U;
use imgui::{CollapsingHeader, ColorPicker, Condition, Context as ImContext, Image, Slider, TextureId, Window, WindowFlags};

use crate::alignment::Attribute::{Color, Position, UV};
use crate::alignment::Layout;
use crate::buffer::{FrameBuffer, RenderBuffer};
use crate::glfw_bind::ImguiGLFW;
use crate::model::{Mesh, Model, Vertex};
use crate::textures::{State, Texture2D};

// use bindings as gl;

mod shader;
mod model;
mod alignment;
mod glfw_bind;
mod textures;
mod dispatcher;
mod buffer;


/*const VERTICES: Vec<Vertex> = vec![
    Vertex { position: Vec3::new(-1.0, -1.0, -1.0) },
    Vertex { position: Vec3::new(1.0, -1.0, -1.0) },
    Vertex { position: Vec3::new(1.0, 1.0, -1.0) },
    Vertex { position: Vec3::new(-1.0, 1.0, -1.0) },
    Vertex { position: Vec3::new(-1.0, -1.0, 1.0) },
    Vertex { position: Vec3::new(1.0, -1.0, 1.0) },
    Vertex { position: Vec3::new(1.0, 1.0, 1.0) },
    Vertex { position: Vec3::new(-1.0, 1.0, 1.0) },
];
const INDICES: Vec<u32> = vec![
    0, 1, 3, 3, 1, 2,
    1, 5, 2, 2, 5, 6,
    5, 4, 6, 6, 4, 7,
    4, 0, 7, 7, 0, 3,
    3, 2, 7, 7, 2, 6,
    4, 5, 0, 0, 5, 1,
];
*/

/*
#[repr(C)]
struct TexVertex {
    position: Vec3,
    color: Vec3,
    uv: Vec2,
}


impl TexVertex {
    fn new(a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32) -> TexVertex {
        TexVertex {
            position: Vec3::new(a1, a2, a3),
            color: Vec3::new(a4, a5, a6),
            uv: Vec2::new(a7, a8),
        }
    }
}

pub struct WindowContainer {
    window: glfw::Window,
    glfw: Glfw,
    cursor_x: f64,
    cursor_y: f64,
    delta_x: f64,
    delta_y: f64,
}

pub struct Camera {
    yaw: f64,
    pitch: f64,
    position: Vec3,
    up: Vec3,
    fov: f32,
    speed: f32,
}

impl Camera {
    fn front(&self) -> Vec3 {
        Vec3 {
            x: (self.yaw.to_radians().cos() * self.pitch.to_radians().cos()) as f32,
            y: self.pitch.to_radians().sin() as f32,
            z: (self.yaw.to_radians().sin() * self.pitch.to_radians().cos()) as f32,
        }.normalize()
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(4, 5));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::Floating(false));
    glfw.window_hint(WindowHint::Resizable(true));

    // glfw.window_hint(WindowHint::Decorated(false));
    let (window, events) = glfw.create_window(1200, 720, "Rust OpenGL", glfw::WindowMode::Windowed)
                               .expect("Failed to create GLFW window.");

    let mut handle = WindowContainer {
        window,
        glfw,
        cursor_x: 0.0,
        cursor_y: 0.0,
        delta_x: 0.0,
        delta_y: 0.0,
    };

    handle.window.set_all_polling(true);
    handle.window.make_current();

    gl::load_with(|s| handle.window.get_proc_address(s) as *const _);

    // loading a specific function pointer
    gl::Viewport::load_with(|s| handle.window.get_proc_address(s) as *const _);

    if handle.glfw.supports_raw_motion() {
        handle.window.set_raw_mouse_motion(true);
    }

    let shader = shader::Shader::load(Path::new("res/shader/flat"));

    let model_obj: Model<TexVertex>;

    let obj_file = "res/model/teapot.obj";
    let (models, materials) = tobj::load_obj(
        &obj_file,
        &tobj::LoadOptions {
            triangulate: true,
            ignore_points: true,
            single_index: false,
            ignore_lines: true,
        },
    ).unwrap();

    let set_model: Model<Vertex> = Model::from_tobj(models);
*/
/*
{
    let materials = materials.unwrap();
    for x in materials.iter() {
        let string = format!("res/model/sponza/{}", x.ambient_texture);
        let textPath = Path::new(string.as_str());
        // println!("Material {} , {}", x.ambient_texture, textPath.exists());
    }

    let mut meshes: Vec<Mesh<TexVertex>> = Vec::new();
    for model in models.iter() {
        let index = model.mesh.material_id.expect("partial uv not supported");
        // println!("material set {}", index);
        let mat = materials.get(index).expect("could not find material");

        let mut vertices: Vec<TexVertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for x in model.mesh.indices.iter() {
            indices.push(*x);
        }
        let len = model.mesh.positions.len();
        let mut i = 0;
        while i < len {
            vertices.push(TexVertex {
                position: Vec3::new(model.mesh.positions[i], model.mesh.positions[i + 1],
                                    model.mesh.positions[i + 2]),
                color: Vec3::new(1.0, 1.0, 1.0),
                uv: Vec2::new(0.0, 0.0),
            });

            i += 3;
        }

        meshes.push(Mesh::from_lists(indices, vertices, Layout::new(vec![Position, Color, UV])));
    }
    model_obj = Model {
        meshes
    }
}
*/

/*    let vertices = vec![
        // positions                            // colors              // texture coords
        TexVertex::new(0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0), // top right
        TexVertex::new(0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0), // bottom right
        TexVertex::new(-0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0), // bottom let
        TexVertex::new(-0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0), // top left
    ];

    let indices = vec![
        0, 1, 3, // first triangle
        1, 2, 3,  // second triangle
    ];
    let model_mesh = Mesh::from_lists(indices, vertices, Layout::new(vec![Position, Color, UV]));

    let img = image::open(&Path::new("res/textures/uvs.png")).expect("Failed to load \
        texture");
    let img = img.flipv();
    let data = img.as_bytes();
    let texture = Texture2D::from_rgb(data, img.width(), img.height());

    let start = Instant::now();
    let (mut width, mut height) = handle.window.get_framebuffer_size();

    let mut delta_time: f32;
    let mut last_frame: f32 = 0.0;

    let mut cam = Camera {
        pitch: 0.0,
        yaw: 0.0,
        fov: 55.0,
        position: Vec3::splat(0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        speed: 2.0,
    };

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    }

    let mut imgui = ImContext::create();
    let mut imgui_glfw = ImguiGLFW::new(&mut imgui, &mut handle.window);

    let mut framebuffer: FrameBuffer;
    let mut texture_colorbuffer: Texture2D;
    unsafe {
        framebuffer = FrameBuffer::new(width as u32, height as u32);
        framebuffer.bind();

        texture_colorbuffer = Texture2D::new(width as u32, height as u32);
        texture_colorbuffer.bind();
        texture_colorbuffer.put_data(0 as *const c_void, gl::RGB);
        texture_colorbuffer.tex_parameter(gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        texture_colorbuffer.tex_parameter(gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        texture_colorbuffer.unbind();

        framebuffer.attach_texture(&texture_colorbuffer, gl::COLOR_ATTACHMENT0);
        let render_buff = RenderBuffer::new(width as u32, height as u32);
        render_buff.bind();
        render_buff.storage(gl::DEPTH24_STENCIL8);
        render_buff.unbind();

        framebuffer.attach_buffer(&render_buff, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER);

        framebuffer.assert_status();
        framebuffer.unbind();
    }

    while !handle.window.should_close() {
        handle.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut cam, &mut handle, &event);
            imgui_glfw.handle_event(&mut imgui, &event);
            if let glfw::WindowEvent::FramebufferSize(f_width, f_height) = event {
                unsafe {
                    gl::Viewport(0, 0, f_width, f_height);
                }
                width = f_width;
                height = f_height;
            }
        }

        let current_frame = handle.glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        handle_keys(&handle, &mut cam, &delta_time);
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.bind();
            let elapsed = start.elapsed();
            let time = elapsed.as_millis() as f32;

            let projection = Mat4::perspective_rh_gl(cam.fov.to_radians(), width as f32 / height as f32, 0.1, 1000.);
            let view = Mat4::look_at_lh(cam.position, cam.position + cam.front(), cam.up);
            let model = Mat4::from_scale_rotation_translation(Vec3::splat(1.0),
                                                              Quat::from_euler(EulerRot::XYZ, 0.0, 0.0,
                                                                               time * 0.00),
                                                              Vec3::new(0.0, 0.0, -1.0));
            // let cam = Mat4::orthographic_rh_gl(-1.0,1.0,-1.0,1.0,0.01,100.0);
            shader.load_mat4("projection", &projection);
            shader.load_mat4("view", &view);
            shader.load_mat4("model", &model);

            framebuffer.bind();
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // we're not using the stencil
            // buffer now
            gl::Enable(gl::DEPTH_TEST);

            // texture.bind();
            // gl::Enable(gl::CULL_FACE);
            // gl::CullFace(gl::BACK);
            // model_mesh.render();
            // model_obj.render();

            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

            texture.unbind();
            set_model.render();

            shader.unbind();

            framebuffer.unbind();

            let style = imgui.style_mut();
            style.frame_border_size = 0.0;
            style.child_border_size = 0.0;
            style.popup_border_size = 0.0;
            style.window_border_size = 0.0;
            style.tab_border_size = 0.0;
            style.frame_padding = [0.0; 2];
            style.window_padding = [0.0; 2];

            let ui = imgui_glfw.frame(&mut handle.window, &mut imgui);

            let aspect = width as f32 / height as f32;
            let reduced_height = height as f32 - (1.0 / aspect) * 200.0;

            let sub = Image::new(
                TextureId::new(texture.id as usize),
                [(texture.width / 4) as f32, (texture.height / 4) as f32]);
            let img = sub.uv0([0.0, 1.0]);
            let img = img.uv1([1.0, 0.0]);
            img.build(&ui);

            let canvas = Window::new("Buffer")
                .position([200.0, 0.0], Condition::Appearing)
                .size([(width - 200) as f32, reduced_height],
                      Condition::Appearing);

            let canvas = canvas.flags(WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE |
                WindowFlags::NO_COLLAPSE | WindowFlags::NO_SCROLLBAR | WindowFlags::NO_TITLE_BAR);

            canvas.build(&ui, || {
                let img = Image::new(
                    TextureId::new(texture_colorbuffer.id as usize),
                    [(width - 200) as f32, reduced_height]);
                let img = img.uv0([0.0, 1.0]);
                let img = img.uv1([1.0, 0.0]);
                img.build(&ui);
            });

            let w = Window::new("Settings")
                .position([0.0, 0.0], Condition::Appearing)
                .size([200.0, 300.0], Condition::Appearing)
                .resizable(false)
                .movable(false).collapsible(false)
                ;

            w.build(&ui, || {
                let mut x = cam.position.to_array();
                ui.input_float3("input float3", &mut x)
                  .build();
                ui.spacing();
                if CollapsingHeader::new("Camera").build(&ui) {
                    Slider::new("FOV", 10f32, 90f32).build(&ui, &mut cam.fov);
                    Slider::new("Speed", 0.1f32, 60f32).build(&ui, &mut cam.speed);
                }
                cam.position.x = x[0];
                cam.position.y = x[1];
                cam.position.z = x[2];
                ui.spacing();
                ui.text(format!("{:.2}ms", delta_time * 1000.0));
                ui.text(format!("{:.0} fps", 1.0 / (delta_time)));
            });

            imgui_glfw.draw(ui, &mut handle.window);
        }
        handle.window.render_context().swap_buffers();
        handle.glfw.set_swap_interval(SwapInterval::Sync(1));
    }
    shader.delete();
    model_mesh.delete();
    set_model.delete();
}

fn handle_window_event(
    camera: &mut Camera,
    handle: &mut WindowContainer,
    event: &glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            handle.window.set_should_close(true);
            handle.window.set_cursor_mode(CursorMode::Normal);
        }
        glfw::WindowEvent::MouseButton(btn, state, _) => {
            let index = match btn {
                MouseButton::Button1 => 1,
                MouseButton::Button2 => 2,
                MouseButton::Button3 => 3,
                MouseButton::Button4 => 4,
                MouseButton::Button5 => 5,
                MouseButton::Button6 => 6,
                MouseButton::Button7 => 7,
                MouseButton::Button8 => 8,
            };

            handle.window.set_cursor_mode(if *state != Action::Press && index == 2 {
                CursorMode::Normal
            } else {
                CursorMode::Disabled
            });
        }
        glfw::WindowEvent::CursorPos(x, y) => {
            let delta_x = handle.cursor_x - x;
            let delta_y = handle.cursor_y - y;

            handle.delta_x = delta_x;
            handle.delta_y = delta_y;

            handle.cursor_x = *x;
            handle.cursor_y = *y;

            if handle.window.get_mouse_button(MouseButton::Button2) == Action::Press {
                camera.yaw -= delta_x * 0.1;
                camera.pitch -= delta_y * 0.1;
            }
        }

        _ => {}
    }
}

fn handle_keys(handle: &WindowContainer, camera: &mut Camera, delta_time: &f32) {
    let camera_speed = camera.speed * delta_time;
    let camera_front = camera.front();
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
*/

struct Lock {
    delta: u64,
}

impl Lock {
    pub fn delta(&self) -> u64 {
        self.delta
    }
}

#[derive(Copy, Clone)]
struct Command<T> where T: Fn() {
    function: T,
}

impl<T> Command<T> where T: Fn() {
    pub fn new(function: T) -> Command<T> {
        Command {
            function
        }
    }
    pub fn execute(&self) {
        (self.function)();
    }
}

fn main() {

    // let LOCK = Arc::new(Mutex::new(0));
    // let compute_lock = Arc::new(Mutex::new(Lock { delta: 1 }));
    // let render_lock = Arc::new(Mutex::new(Lock { delta: 1 }));
    // let both_lock = Arc::new(Mutex::new((compute_lock, render_lock)));
    // let render_buffer: Arc<Mutex<Vec<Command>>> = Arc::new(Mutex::new(Vec::new()));
    let intermediate_buffer: Arc<Mutex<Vec<Command<_>>>> = Arc::new(Mutex::new(Vec::new()));
    // let write_lock = Arc::new(Mutex::new(false));

    // let (tx, rx) = mpsc::channel();

    {
        let intermediate_buffer = Arc::clone(&intermediate_buffer);
        // let write_lock = Arc::clone(&write_lock);
        thread::spawn(move || {
            loop {
                let mut commands = Vec::new();
                commands.push(Command::new(move || {
                    thread::sleep(Duration::from_millis(100));
                }));
                 thread::sleep(Duration::from_millis(100));

                {
                    let now = Instant::now();
                    while !intermediate_buffer.lock().unwrap().is_empty() {
                        //wait
                    }
                    let elapsed = now.elapsed();
                    // println!("Waited {} for render thread transfer" , elapsed.as_micros());
                    let mut buffer = intermediate_buffer.lock().unwrap();
                    for x in commands {
                        buffer.push(x);
                    }
                    let elapsed = now.elapsed();
                    // println!("Waited {} for command transfer" , elapsed.as_micros());
                }
                // println!("thread 1");
            }
        });
    }

    // let write_lock = Arc::clone(&write_lock);
    let intermediate_buffer = Arc::clone(&intermediate_buffer);
    let mut runns = 0;
    let mut now = Instant::now();
    loop {
        let mut render_buffer = Vec::new();

        {
            let now = Instant::now();
            let mut guard = intermediate_buffer.lock().unwrap();
            let elapsed = now.elapsed();
            // println!("Waited {} for compute thread transfer" , elapsed.as_micros());
            for x in guard.iter() {
                render_buffer.push(*x);
            }
            let elapsed = now.elapsed();
            // println!("Waited {} for command transfer" , elapsed.as_micros());

            guard.clear();
        }
        if render_buffer.len() > 0 {
            let elapsed = now.elapsed();
            runns += 1;
            if elapsed.as_millis() >= 1000 {
                println!("{} FPS", runns);
                now = Instant::now();
                runns = 0;
            }
            for command in render_buffer {
                command.execute();
                // if command.delay > 0 {
                //     thread::sleep(Duration::from_millis(command.delay as u64));
                // }
            }
        }
        {
            let now = Instant::now();
            while now.elapsed().as_micros() < 1 {
                //wait
            }
        }
    }
}
