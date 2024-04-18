use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
use std::mem::size_of;
use std::str::SplitWhitespace;
use gl::types::GLushort;

use crate::models::obj_data::{ObjData, Vertex, Face};
use crate::models::vec3::Vec3;

pub fn parse_obj_file(file_path: &str) -> Result<ObjData, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut num_vertices: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.trim().split_whitespace();

        match parts.next() {
            Some("v") => {
                add_vertex(&mut vertices, &mut parts)?;
                num_vertices += 1;
            }
            Some("f") => {
                add_face(&mut faces, &mut parts)?;
            }
            _ => {}
        }
    }
    let indices = get_indices_array_from_faces(&faces);
    let num_indices = indices.len();
    let vertices_raw = get_vertices_array(&vertices);
    let vertex_buffer_size = vertices_raw.len() * size_of::<f32>();
    let indices_buffer_size = indices.len() * size_of::<u16>();

    Ok(ObjData { vertices, num_vertices, vertices_raw, vertex_buffer_size, indices, num_indices, indices_buffer_size })
}

fn add_vertex(vertices: &mut Vec<Vertex>, parts: &mut SplitWhitespace ) -> Result<(), Error> {
    let x = parts.next().ok_or_else(|| Error::new(io::ErrorKind::InvalidData, "Invalid vertex format"))?;
    let y = parts.next().ok_or_else(|| Error::new(io::ErrorKind::InvalidData, "Invalid vertex format"))?;
    let z = parts.next().ok_or_else(|| Error::new(io::ErrorKind::InvalidData, "Invalid vertex format"))?;

    if parts.next().is_some() {
        return Err(Error::new(io::ErrorKind::InvalidData, "A vertex must have exactly 3 coordinates"));
    }
    
    let xfloat = match x.parse::<f32>() {
        Ok(value) => value,
        Err(e) => return Err(Error::new(io::ErrorKind::InvalidData, e)),
    };
    let yfloat = match y.parse::<f32>() {
        Ok(value) => value,
        Err(e) => return Err(Error::new(io::ErrorKind::InvalidData, e)),
    };
    let zfloat = match z.parse::<f32>() {
        Ok(value) => value,
        Err(e) => return Err(Error::new(io::ErrorKind::InvalidData, e)),
    };
    
    let position = Vec3::new(xfloat, yfloat, zfloat);
    let mut text_x: f32 = 0.0;
    let mut text_y: f32 = 0.0;
    if position.x == 0.5 && position.y == 0.5 {
        text_x = 1.0;
        text_y = 1.0;
    }
    else if position.x == 0.5 && position.y == -0.5 {
        text_x = 1.0;
        text_y = 0.0;
    }
    else if position.x == -0.5 && position.y == -0.5 {
        text_x = 0.0;
        text_y = 0.0;
    }
    else {
        text_x = 0.0;
        text_y = 1.0;
    }
    vertices.push(Vertex {position, rgb: None, text_x, text_y});

    Ok(())
}

fn add_face(faces: &mut Vec<Face>, parts: &mut SplitWhitespace) -> Result<(), Error> {
    let mut indices: Vec<GLushort> = Vec::new();

    for (i, s) in parts.enumerate() {
        let index = match s.parse::<GLushort>() {
            Ok(value) => {
                if value == 0 {
                    return Err(Error::new(io::ErrorKind::InvalidData, "Invalid vertex index (0)"));
                }
                value
            },
            Err(e) => return Err(Error::new(io::ErrorKind::InvalidData, e)),
        };
        indices.push(index);
    }
    if indices.len() < 3 {
        return Err(Error::new(io::ErrorKind::InvalidData, "A face must have at least 3 indices"))
    }
    else if indices.len() == 3 {
        faces.push(Face {indices});
    }
    else {
        for i in 2..indices.len() {
            let face = Face {
                indices: vec![indices[0], indices[i -1], indices[i]],
            };
            faces.push(face);
        }
    }
    Ok(())






    // let indices: Result<Vec<GLushort>, _> = parts
    //     .map(|s| s.parse::<GLushort>())
    //     .collect();
    // let indices = match indices {
    //     Ok(indices) => indices,
    //     Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    // };
    // if indices.len() < 3 {
    //     return Err(Error::new(io::ErrorKind::InvalidData, "A face must have at least 3 indices"));
    // }
    // faces.push(Face { indices });

    // Ok(())
}

fn get_indices_array_from_faces (faces: &Vec<Face>) -> Vec<GLushort> {
    let mut res = Vec::new();
    for face in faces {
        for &index in &face.indices {
            res.push(index - 1); // indices start from 1 in obj file, they need to start from 0 in openGL buffer
        }
    }
    res
}

fn get_vertices_array(vertices: &Vec<Vertex>) -> Vec<f32> {
    let mut vertices_raw = Vec::with_capacity(vertices.len() * 8);

    for vertex in vertices {
        vertices_raw.push(vertex.position.x);
        vertices_raw.push(vertex.position.y);
        vertices_raw.push(vertex.position.z);
        
        if let Some(rgb) = &vertex.rgb {
            vertices_raw.push(rgb.x);
            vertices_raw.push(rgb.y);
            vertices_raw.push(rgb.z);
        } else { 
            vertices_raw.push(0.0); // default to black
            vertices_raw.push(0.0);
            vertices_raw.push(0.0);
        }
        vertices_raw.push(vertex.text_x);
        vertices_raw.push(vertex.text_y);
    }

    vertices_raw
}