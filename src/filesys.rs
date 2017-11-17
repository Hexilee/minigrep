use std::io;
use std::io::Read;
use std::fs::File;
pub fn read_file(path: &str) -> Result<String, io::Error> {
    let mut result = String::new();
    File::open(path)?.read_to_string(&mut result)?;
    Ok(result)
}