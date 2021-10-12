use std::error::Error;
use std::fs;

use async_trait::async_trait;

pub struct FileResource<'a> {
    pub file_path: &'a str,
}

#[async_trait]
impl resource::Resource for FileResource<'_> {
    async fn fetch(&self) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(self.file_path)?.trim().to_string();
        Ok(content)
    }
}
