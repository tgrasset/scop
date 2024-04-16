use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
use std::str::SplitWhitespace;
use crate::models::obj_data::{ObjData, Vertex, Face};
use crate::models::vec3::Vec3;

pub fn parse_obj_file(file_path: &str) -> Result<ObjData, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut n_vertices: u32 = 0;
    let mut n_vertices: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.trim().split_whitespace();

        match parts.next() {
            Some("v") => {
                add_vertex(&mut vertices, &mut parts)?;
            }
            Some("f") => {
                add_face(&mut faces, &mut parts)?;
            }
            _ => {}
        }
    }
    Ok(ObjData { vertices, faces })
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
    vertices.push(Vertex {position, rgb: None});

    Ok(())
}

fn add_face(faces: &mut Vec<Face>, parts: &mut SplitWhitespace) -> Result<(), Error> {
    let indices: Result<Vec<usize>, _> = parts
        .map(|s| s.parse::<usize>())
        .collect();
    let indices = match indices {
        Ok(indices) => indices,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    };
    if indices.len() < 3 {
        return Err(Error::new(io::ErrorKind::InvalidData, "A face must have at least 3 indices"));
    }
    faces.push(Face { indices });

    Ok(())
}