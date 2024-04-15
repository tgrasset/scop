use glfw::Context;

use std::io::{Error, ErrorKind};

use crate::models::gl_var::GlVar;

pub fn init_window(width: u32, height: u32) -> Result<GlVar, Error> {

    println!("Initializing glfw...");
    let mut glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
        Ok(glfw) => glfw,
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    };
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) =
        glfw.create_window(width, height, "scop", glfw::WindowMode::Windowed)
            .expect("Failed to create window");
    // Make this window our current context, poll events (keys and frame buffer size)
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // OpenGL functions' addresses loaded at runtime so rust can use them
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    
    Ok(GlVar {glfw: glfw, window: window, events: events})
}