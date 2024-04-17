use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn load_texture(path: &str) -> Result<Vec<u8>, String> {
    let mut file = File::open(path).map_err(|e| format!("Error opening file: {}", e))?;
    let data = read_bmp(&mut file)?;
    Ok(data)
}

fn read_bmp(file: &mut File) -> Result<Vec<u8>, String> {
    let mut header = [0; 54];
    file.read_exact(&mut header).map_err(|_| "Error reading header")?;
    
    if &header[0..2] != b"BM" {
        return Err("Not a BMP file".to_string());
    }
    
    let data_pos = u32::from_le_bytes([header[0x0A], header[0x0B], header[0x0C], header[0x0D]]) as u64;
    let image_size = u32::from_le_bytes([header[0x22], header[0x23], header[0x24], header[0x25]]) as usize;
    let width = i32::from_le_bytes([header[0x12], header[0x13], header[0x14], header[0x15]]);
    let height = i32::from_le_bytes([header[0x16], header[0x17], header[0x18], header[0x19]]);
    
    let tex_width = width.abs() as usize;
    let tex_height = height.abs() as usize;

    let image_size = if image_size == 0 { (tex_width * tex_height * 3) as usize } else { image_size };
    let data_pos = if data_pos == 0 { 54 } else { data_pos as u64 };

    file.seek(SeekFrom::Start(data_pos)).map_err(|_| "Error seeking to data")?;

    let mut data = vec![0; image_size];
    file.read_exact(&mut data).map_err(|_| "Error reading data")?;
    
    Ok(data)
}