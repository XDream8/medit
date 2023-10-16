use std::fs;
use std::io::Result;
use std::path::Path;

#[inline]
pub fn cat(path: &Path) -> Result<String> {
    let file_bytes: Vec<u8> = fs::read(path)?;
    let buffer: String = String::from_utf8(file_bytes).unwrap_or(String::new());

    Ok(buffer)
}
