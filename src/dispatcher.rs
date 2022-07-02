use std::collections::VecDeque;
use std::thread;

struct RenderQueue {
    commands: VecDeque<unsafe fn()>,
}

impl RenderQueue {
    pub fn dispatch(&mut self, command: unsafe fn()) {
        self.commands.push_back(command);
    }
    pub fn new() -> RenderQueue {
        RenderQueue {
            commands: VecDeque::new()
        }
    }
}

pub fn test () {
    let mut dispatcher: RenderQueue = RenderQueue::new();
    dispatcher.dispatch(move || unsafe {
        gl::Enable(gl::DEPTH_TEST);
    });
    
}

