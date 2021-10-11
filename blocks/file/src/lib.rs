use std::error::Error;
use std::fs;

pub struct FileResource<'a> {
    pub file_path: &'a str,
}

impl resource::Resource for FileResource<'_> {
    fn fetch(&self) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(self.file_path)?.trim().to_string();
        Ok(content)
    }
}
