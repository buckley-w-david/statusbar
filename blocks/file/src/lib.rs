use std::error::Error;
use std::fs;

pub struct FileBlock<'a> {
    pub file_path : &'a str
}

impl block::Block for FileBlock<'_> {
    fn perform(&self) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(self.file_path)?.trim().to_string();
        Ok(content)
    }
}
