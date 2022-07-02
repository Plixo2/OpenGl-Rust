extern crate core;
extern crate gl;
extern crate glfw;

use std::{ptr, thread};
use std::sync::{Arc, mpsc, Mutex};
use std::time::{Duration, Instant};
use crate::alignment::Attribute::{ Position, UV};
use crate::rendering::RenderPath;
use crate::textures::{Texture2D};

mod shader;
mod model;
mod alignment;
mod glfw_bind;
mod textures;
mod dispatcher;
mod buffer;
mod rendering;
mod camera;
mod scene;
mod computation;




fn main() {
    let mut path = RenderPath::new(1200, 720);
    start(&mut path);
}

fn start(renderer: &mut RenderPath) {
    renderer.start();
    let mut frames = 0;
    let mut fps_timer = Instant::now();
    while renderer.running() {
        let i = fps_timer.elapsed().as_millis();
        if i >= 1000 {
            renderer.window.fps = frames;
            fps_timer = Instant::now();
            frames = 0;
        }
        frames += 1;


        renderer.window.glfw.poll_events();
        let mut events = Vec::new();
        for x in glfw::flush_messages(&renderer.window.event_receiver) {
            events.push(x.1);
        }
        for event in events.iter() {
            renderer.handle_event(&event);
            renderer.ui.handle_event(&mut renderer.ui_context, &event);
        }

        renderer.render();
    }
    renderer.dispose();
}
