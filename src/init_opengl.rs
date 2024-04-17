use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint, GLvoid};
use glfw::Context;

use std::{io::{Error, ErrorKind}, mem::size_of, os::raw::c_void};

use crate::models::{gl_var::GlVar, obj_data::{self, ObjData}};

pub fn init_window(width: u32, height: u32) -> Result<GlVar, Error> {

    println!("Initializing glfw...");
    let mut glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
        Ok(glfw) => glfw,
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    };
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) =
        glfw.create_window(
            width,
            height,
            "scop",
            glfw::WindowMode::Windowed)
            .expect("Failed to create window");
    // Make this window our current context, poll events (keys and frame buffer size)
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // OpenGL functions' addresses loaded at runtime so rust can use them
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    
    Ok(GlVar {glfw: glfw, window: window, events: events, shader_prgm_id: 0})
}

pub unsafe fn send_data_to_opengl(obj_data: &ObjData, texture_data: &Vec<u8>) -> GLuint {

    let (mut VBO, mut VAO) = (0, 0);
    gl::GenVertexArrays(1, &mut VAO);
    gl::GenBuffers(1, &mut VBO);
    gl::BindVertexArray(VAO);
    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::BufferData(gl::ARRAY_BUFFER,
                    (obj_data.vertices_raw.len() * size_of::<GLfloat>()) as GLsizeiptr,
                    obj_data.vertices_raw.as_ptr() as *const GLvoid,
                    gl::STATIC_DRAW);
    gl::VertexAttribPointer(
        0,
        3, 
        gl::FLOAT,
        gl::FALSE,
        6 * size_of::<GLfloat>() as GLsizei,
        std::ptr::null());
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        6 * size_of::<GLfloat>() as GLsizei,
        (3 * size_of::<GLfloat>()) as *const c_void);
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::EnableVertexAttribArray(1);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        obj_data.indices_buffer_size as GLsizeiptr,
        obj_data.indices.as_ptr() as *const c_void,
        gl::STATIC_DRAW);
    VAO
}