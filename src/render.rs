use crate::glfw::{Context, Key, Action};
use std::sync::mpsc::Receiver;

use crate::models::obj_data::{self, ObjData};
use crate::models::{gl_var::GlVar};
use crate::models::mat4::Mat4;
use crate::models::vec3::Vec3;
use crate::globals::*;

pub fn render_loop(glvar: &mut GlVar, vao: &u32, obj_data: &ObjData) {
    println!("Rendering...");
    //modif here
    let mut aspect_ratio = glvar.window.get_framebuffer_size().0 as f32 / glvar.window.get_framebuffer_size().1 as f32;
    let eye = Vec3::new(0.0, 0.0, 3.0);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let view = look_at(eye, target, up);

    let mut model = Mat4::identity();

    let mut projection = Mat4::perspective(FOV, aspect_ratio, NEAR, FAR);
    // end of modif
    while !glvar.window.should_close() {
        process_events(&mut glvar.window, &glvar.events);
        let (width, height) = glvar.window.get_framebuffer_size();
        if height != 0 {
            aspect_ratio = width as f32 / height as f32;
        }
        projection = Mat4::perspective(FOV, aspect_ratio, NEAR, FAR);
        model = model.rotate_y(0.01);
        unsafe {
            gl::ClearColor(0.0, 0.1, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::BindTexture(gl::TEXTURE_2D, glvar.texture_id);
            gl::UseProgram(glvar.shader_prgm_id);
            //modif here
            let model_location = gl::GetUniformLocation(glvar.shader_prgm_id, "model\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, std::mem::transmute(model.data.as_ptr()));
            let view_location = gl::GetUniformLocation(glvar.shader_prgm_id, "view\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE, std::mem::transmute(view.data.as_ptr()));
            let projection_location = gl::GetUniformLocation(glvar.shader_prgm_id, "projection\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, std::mem::transmute(projection.data.as_ptr()));
            // end of modif
            gl::BindVertexArray(*vao);
            gl::DrawElements(gl::TRIANGLES, obj_data.num_indices as i32, gl::UNSIGNED_SHORT, std::ptr::null());
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

fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    let forward = (target.sub(eye)).normalize();
    let right = up.cross(forward).normalize();
    let up = forward.cross(right);

    let mut result = Mat4::identity();
    result.data[0][0] = right.x;
    result.data[1][0] = right.y;
    result.data[2][0] = right.z;
    result.data[0][1] = up.x;
    result.data[1][1] = up.y;
    result.data[2][1] = up.z;
    result.data[0][2] = -forward.x;
    result.data[1][2] = -forward.y;
    result.data[2][2] = -forward.z;
    result.data[3][0] = -right.dot(eye);
    result.data[3][1] = -up.dot(eye);
    result.data[3][2] = forward.dot(eye);
    result
}