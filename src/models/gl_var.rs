use gl::types::GLuint;
use glfw::{Glfw, Window, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct GlVar {
    pub glfw: Glfw,
    pub window: Window,
    pub events: Receiver<(f64, WindowEvent)>,
    pub shader_prgm_id: GLuint,
}

impl GlVar {
    pub fn set_shader_prgm_id(&mut self, id: GLuint) {
       self.shader_prgm_id = id;
    }
}