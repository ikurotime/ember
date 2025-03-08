use std::{fs::read_to_string, io::Error};

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Buffer, Error> {
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for line in contents.lines() {
            lines.push(line.to_string());
        }
        Ok(Buffer { lines })
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
