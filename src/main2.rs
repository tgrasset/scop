extern crate gl;

use self::gl::types::*;

extern crate glfw;
use self::glfw::{Context, Key, Action};

use std::mem;
use std::{ffi::CString, sync::mpsc::Receiver, ptr, str, os::raw::c_void};


const WIN_WIDTH: u32 = 800;
const WIN_HEIGHT: u32 = 600;

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

fn main() {
    // Initializing glfw and window
    println!("Initializing glfw");
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = 
        glfw.create_window(WIN_WIDTH, WIN_HEIGHT, "scop",
        glfw::WindowMode::Windowed).expect("Failed to create window");

    // Make this window our current context, poll events (keys and frame buffer size)
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // OpenGL functions' addresses loaded at runtime so rust can use them
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let vao = unsafe {

        // vertex shader compilation
        println!("Compiling shaders.");
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        // compilation error ?
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // pour le null
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("Error: Vertex shader compilation failed\n{}", str::from_utf8(&info_log).unwrap());
        }

        // fragment shader compilation
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        // compilation error ?
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
        }

        // linking
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // linking error ?
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
        }
        gl::DeleteShader(vertex_shader); // no longer needed, program linked
        gl::DeleteShader(fragment_shader);

        // triangle coordinates
        let vertices: [f32; 18] = [
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
            0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
            0.0, 0.5, 0.0, 0.0, 0.0, 1.0
        ];

        // generate vertex array and vertex buffer objects, then bind them
        let (mut vbo, mut vao) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &vertices[0] as *const f32 as *const c_void,
                        gl::STATIC_DRAW);
        let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0,
                                 3,
                                 gl::FLOAT,
                                 gl::FALSE,
                                 stride,
                                 ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1,
                                3, 
                                gl::FLOAT,
                                gl::FALSE,
                                stride,
                                (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);
        gl::UseProgram(shader_program);
        vao
    };

    //render loop
    println!("Rendering...");
    while !window.should_close() {
        process_events(&mut window, &events);
        // rendering in the back buffer
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.swap_buffers();
        glfw.poll_events();
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
