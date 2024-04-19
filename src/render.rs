use crate::glfw::{Context, Key, Action};
use std::hash::Hash;
use std::sync::mpsc::Receiver;
use std::collections::HashSet;

use crate::models::obj_data::{self, ObjData};
use crate::models::{gl_var::GlVar};
use crate::models::mat4::Mat4;
use crate::models::vec3::Vec3;
use crate::globals::*;

pub fn render_loop(glvar: &mut GlVar, vao: &u32, obj_data: &mut ObjData) {
    println!("Rendering...");

    let mut keys:HashSet<Key> = HashSet::new();

    let mut aspect_ratio = glvar.window.get_framebuffer_size().0 as f32 / glvar.window.get_framebuffer_size().1 as f32;
    
    let eye_distance = obj_data.longest_distance * 2.0; // Adjust as needed  
    let view = look_at(eye_distance);

    let mut projection = Mat4::perspective(FOV, aspect_ratio, NEAR, FAR);

    while !glvar.window.should_close() {

        process_events(&mut glvar.window, &glvar.events, &mut keys, obj_data);

        let model = Mat4::identity()
            .translate(-obj_data.center_x, -obj_data.center_y, -obj_data.center_z)
            .rotate_x(obj_data.orientation_x)
            .rotate_y(obj_data.orientation_y)
            .rotate_z(obj_data.orientation_z)
            .translate(obj_data.center_x, obj_data.center_y, obj_data.center_z)
            .scale(obj_data.scale_x, obj_data.scale_y, obj_data.scale_z)
            .translate(obj_data.position_x, obj_data.position_y, obj_data.position_z);
        
        let (width, height) = glvar.window.get_framebuffer_size();
        if height != 0 {
            aspect_ratio = width as f32 / height as f32;
        }
        projection = Mat4::perspective(FOV, aspect_ratio, NEAR, FAR);
            
        unsafe {
            gl::ClearColor(0.6, 0.6, 0.6 , 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            if obj_data.display_texture {
                gl::BindTexture(gl::TEXTURE_2D, glvar.texture_id);
            } else {
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
            gl::UseProgram(glvar.shader_prgm_id);

            let model_location = gl::GetUniformLocation(glvar.shader_prgm_id, "model\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, std::mem::transmute(model.data.as_ptr()));
            let view_location = gl::GetUniformLocation(glvar.shader_prgm_id, "view\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE, std::mem::transmute(view.data.as_ptr()));
            let projection_location = gl::GetUniformLocation(glvar.shader_prgm_id, "projection\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, std::mem::transmute(projection.data.as_ptr()));

            gl::BindVertexArray(*vao);
            gl::DrawElements(gl::TRIANGLES, obj_data.num_indices as i32, gl::UNSIGNED_SHORT, std::ptr::null());
        }
        glvar.window.swap_buffers();
        glvar.glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, keys: &mut HashSet<Key>, obj_data: &mut ObjData) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            // match viewport to window size if changed
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(key, _, Action::Press, _) => {
                if key == Key::T {
                    obj_data.display_texture = !obj_data.display_texture;
                }
                else if key == Key::Escape {
                    window.set_should_close(true);
                }
                keys.insert(key);
            },
            glfw::WindowEvent::Key(key, _, Action::Release, _) => {
                keys.remove(&key);
            }
            _ => {}
        }
    }
    if keys.contains(&Key::Up) {
        obj_data.orientation_x += TRANSFORM_SPEED;
    }
    if keys.contains(&Key::Down) {
        obj_data.orientation_x -= TRANSFORM_SPEED;
    }
    if keys.contains(&Key::Right) {
        obj_data.orientation_y += TRANSFORM_SPEED;
    }
    if keys.contains(&Key::Left) {
        obj_data.orientation_y -= TRANSFORM_SPEED;
    }
    if keys.contains(&Key::Z) {
        obj_data.orientation_z += TRANSFORM_SPEED;
    }
    if keys.contains(&Key::X) {
        obj_data.orientation_z -= TRANSFORM_SPEED;
    }
    if keys.contains(&Key::D) {
        obj_data.position_x -= TRANSFORM_SPEED;
    }
    if keys.contains(&Key::A) {
        obj_data.position_x += TRANSFORM_SPEED;
    }
    if keys.contains(&Key::W) {
        obj_data.position_y += TRANSFORM_SPEED;
    }
    if keys.contains(&Key::S) {
        obj_data.position_y -= TRANSFORM_SPEED;
    }
    if keys.contains(&Key::Q) {
        obj_data.position_z += TRANSFORM_SPEED;
    }
    if keys.contains(&Key::E) {
        obj_data.position_z -= TRANSFORM_SPEED;
    }
    if keys.contains(&Key::Kp4) {
        obj_data.scale_x += TRANSFORM_SPEED;
    }
    if keys.contains(&Key::Kp1) {
        obj_data.scale_x -= TRANSFORM_SPEED;
    }
    if keys.contains(&Key::Kp5) {
        obj_data.scale_y += TRANSFORM_SPEED;
    }
    if keys.contains(&Key::Kp2) {
        obj_data.scale_y -= TRANSFORM_SPEED;
    }
    if keys.contains(&Key::Kp6) {
        obj_data.scale_z += TRANSFORM_SPEED;
    }
    if keys.contains(&Key::Kp3) {
        obj_data.scale_z -= TRANSFORM_SPEED;
    }
}

fn look_at(eye_distance: f32) -> Mat4 {
    let eye = Vec3::new(0.0, 0.0, eye_distance);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
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