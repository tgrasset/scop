use std::env;

use globals::{WIN_HEIGHT, WIN_WIDTH};

extern crate gl;
extern crate glfw;

mod obj_parser;
mod models;
mod globals;
mod init_opengl;
mod compile_shaders;
mod texture_loader;
mod render;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err("Error: Wrong number of arguments\nUsage: ./scop path/to/object/file path/to/bmp/texture".to_string());
    }

    let mut objdata = match obj_parser::parse_obj_file(&args[1]) {
        Ok(data) => data,
        Err(err) => return Err(format!("Error while parsing obj file: {}", err)),
    };

    let mut glvar = match init_opengl::init_window(WIN_WIDTH, WIN_HEIGHT) {
        Ok(vars) => vars,
        Err(err) => return Err(format!("Error while initializing window: {}", err)),
    };

    let (vao, vbo, ebo) = unsafe { init_opengl::send_data_to_opengl(&objdata) };

    match compile_shaders::compile_shaders() {
        Ok(shader_prgm_id) => glvar.set_shader_prgm_id(shader_prgm_id),
        Err(err) => return Err(format!("Error while compiling shaders: {}", err)),
    }

    match texture_loader::load_texture(&args[2]) {
        Ok(texture_id) => glvar.set_texture_id(texture_id),
        Err(err) => return Err(format!("Error while loading texture: {}", err)),
    };

    render::render_loop(&mut glvar, &vao, &mut objdata);

    // Clean up OpenGL resources
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
        gl::DeleteProgram(glvar.shader_prgm_id);
    }
    Ok(())
}
