use std::error::Error;

use futures_lite::stream::StreamExt;
use async_trait::async_trait;

pub struct FileResource<'a> {
    pub file_path: &'a str,
}

#[async_trait]
impl resource::Resource for FileResource<'_> {
    async fn fetch(&self) -> Result<String, Box<dyn Error>> {
        Ok(async_fs::read_to_string(self.file_path).await?.trim().to_string())
    }
}

pub struct DirectoryResource<'a> {
    pub directory_path: &'a str,
}


#[async_trait]
impl resource::Resource for DirectoryResource<'_> {
    async fn fetch(&self) -> Result<String, Box<dyn Error>> {
        Ok(async_fs::read_dir(self.directory_path).await?.count().await.to_string())
    }
}
