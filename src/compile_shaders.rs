use gl::types::*;

use std::{ffi::CString, io::{Error, ErrorKind}, ptr};


pub fn compile_shaders() -> Result<GLuint, Error> {
    println!("Compiling shaders...");
    unsafe {
        let vertex_shader_id = gl::CreateShader(gl::VERTEX_SHADER);
        let mut c_str_source = read_shader_code("./src/shader_code/vertex_shader.glsl")?;
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
        c_str_source = read_shader_code("./src/shader_code/fragment_shader.glsl")?;
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
        let shader_program_id = gl::CreateProgram();
        gl::AttachShader(shader_program_id, vertex_shader_id);
        gl::AttachShader(shader_program_id, fragment_shader_id);
        gl::LinkProgram(shader_program_id);
        gl::DeleteShader(vertex_shader_id); 
        gl::DeleteShader(fragment_shader_id);
        match check_shader_program_link(shader_program_id) {
            Ok(()) => {},
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        };
        Ok(shader_program_id)
    }
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

fn read_shader_code(file_path: &str) -> Result<CString, Error> {
    let content = std::fs::read_to_string(file_path)?;
    let c_str = CString::new(content)?;
    Ok(c_str)
}