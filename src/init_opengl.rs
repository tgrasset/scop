use gl::types::*;
use glfw::Context;

use std::{ffi::CString, io::{Error, ErrorKind}, ptr};

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

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    out vec3 our_color;
    void main() {
        gl_Position = vec4(aPos, 1.0);
        our_color = aColor;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    in vec3 our_color;
    void main() {
       FragColor = vec4(our_color, 1.0);
    }
"#;

pub fn compile_shaders() -> Result<GLuint, Error> {
    println!("Compiling shaders...");
    let mut shader_program_id: GLuint = 0;
    unsafe {
        let vertex_shader_id = gl::CreateShader(gl::VERTEX_SHADER);
        let mut c_str_source = CString::new(VERTEX_SHADER_SOURCE.as_bytes())?;
        gl::ShaderSource(
            vertex_shader_id,
            1,
            &c_str_source.as_ptr(),
            std::ptr::null());
        gl::CompileShader(vertex_shader_id);
        match check_shader_compilation(vertex_shader_id) {
            Ok(()) => {},
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        };

        let fragment_shader_id = gl::CreateShader(gl::FRAGMENT_SHADER);
        c_str_source = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes())?;
        gl::ShaderSource(
            fragment_shader_id,
            1,
            &c_str_source.as_ptr(),
            std::ptr::null());
        gl::CompileShader(fragment_shader_id);
        match check_shader_compilation(fragment_shader_id) {
            Ok(()) => {},
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        };

        println!("Linking shaders...");
        shader_program_id = gl::CreateProgram();
        gl::AttachShader(shader_program_id, vertex_shader_id);
        gl::AttachShader(shader_program_id, fragment_shader_id);
        gl::LinkProgram(shader_program_id);
        gl::DeleteShader(vertex_shader_id); 
        gl::DeleteShader(fragment_shader_id);
        match check_shader_program_link(shader_program_id) {
            Ok(()) => {},
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        };
    }
    Ok(shader_program_id)
}

unsafe fn check_shader_compilation(shader_id: GLuint) -> Result<(), String> {
    let mut success = gl::FALSE as GLint;
    gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        let mut info_log_len : GLint = 0;
        gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut info_log_len);
        let mut info_log: Vec<u8> = Vec::with_capacity(info_log_len as usize + 1);
        info_log.extend([b' '].iter().cycle().take(info_log_len as usize));
        let error: CString = CString::from_vec_unchecked(info_log);
        gl::GetShaderInfoLog(
            shader_id,
            info_log_len,
            ptr::null_mut(),
            error.as_ptr()as *mut GLchar);
        return Err(error.to_string_lossy().into_owned());
    }
    Ok(())
}

unsafe fn check_shader_program_link(id: GLuint) -> Result<(), String> {
    let mut success = gl::FALSE as GLint;
    gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        let mut info_log_len : GLint = 0;
        gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut info_log_len);
        let mut info_log: Vec<u8> = Vec::with_capacity(info_log_len as usize + 1);
        info_log.extend([b' '].iter().cycle().take(info_log_len as usize));
        let error: CString = CString::from_vec_unchecked(info_log);
        gl::GetProgramInfoLog(
            id,
            info_log_len,
            ptr::null_mut(),
            error.as_ptr()as *mut GLchar);
        return Err(error.to_string_lossy().into_owned());
    }
    Ok(())
}