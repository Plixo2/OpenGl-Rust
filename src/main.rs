extern crate gl;
extern crate glfw;

use std::ffi::CString;
use std::path::Path;
use std::{mem, ptr};

use glfw::{Action, Context, Key, MouseButton, OpenGlProfileHint, SwapInterval, WindowHint};

// use bindings as gl;

use crate::gl::types::{GLboolean, GLchar, GLfloat, GLint, GLsizei, GLsizeiptr};

mod bindings;
mod shader;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::ContextVersionMajor(4));
    glfw.window_hint(WindowHint::ContextVersionMinor(5));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::Floating(false));
    glfw.window_hint(WindowHint::Resizable(true));

    // glfw.window_hint(WindowHint::Decorated(false));
    let (mut window, events) = glfw
        .create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_all_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // loading a specific function pointer
    gl::Viewport::load_with(|s| window.get_proc_address(s) as *const _);

    let shader = shader::Shader::load(Path::new("res/shader/flat"));
    let mut VAO = 0;
    let mut VBO = 0;
    let mut EBO = 0;
    unsafe {
        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let vertices: [GLfloat; 12] = [
            0.5, 0.5, 0.0, // top right
            0.5, -0.5, 0.0, // bottom right
            -0.5, -0.5, 0.0, // bottom left
            -0.5, 0.5, 0.0, // top left
        ];
        let indices: [GLint; 6] = [
            0, 1, 3, // first Triangle
            1, 2, 3, // second Triangle
        ];

        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut EBO);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        let float_size = mem::size_of::<GLfloat>();
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * float_size) as GLsizeiptr,
            mem::transmute(&vertices[0]),
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * mem::size_of::<GLint>()) as GLsizeiptr,
            mem::transmute(&indices[0]),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            (3 * float_size) as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to glVertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::BindVertexArray(0);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader.bind();
            gl::BindVertexArray(VAO);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            shader.unbind();
        }
        window.render_context().swap_buffers();
    }
    shader.delete();
    unsafe {
        gl::DeleteVertexArrays(1, &VAO);
        gl::DeleteBuffers(1, &VBO);
        gl::DeleteBuffers(1, &EBO);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
        },
        glfw::WindowEvent::MouseButton(btn, Action::Press, _) => {
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
            println!("Pressed {}", index);
        }
        _ => {}
    }
}
