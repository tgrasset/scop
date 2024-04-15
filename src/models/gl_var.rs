use glfw::{Glfw, Window, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct GlVar {
    pub glfw: Glfw,
    pub window: Window,
    pub events: Receiver<(f64, WindowEvent)>,
}