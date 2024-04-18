use crate::models::{gl_var::GlVar};
use crate::glfw::{Context, Key, Action};
use std::sync::mpsc::Receiver;

pub fn render_loop(glvar: &mut GlVar, vao: &u32) {
    println!("Rendering...");
    while !glvar.window.should_close() {
        process_events(&mut glvar.window, &glvar.events);
        unsafe {
            gl::ClearColor(0.0, 0.1, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::BindTexture(gl::TEXTURE_2D, glvar.texture_id);
            gl::UseProgram(glvar.shader_prgm_id);
            gl::BindVertexArray(*vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_SHORT, std::ptr::null());
        }
        glvar.window.swap_buffers();
        glvar.glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            // match viewport to window size if changed
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            // escape to close window
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}