use std::error::Error;

use async_trait::async_trait;

pub struct NoResource ;

#[async_trait]
impl resource::Resource for NoResource {
    async fn fetch(&self) -> Result<String, Box<dyn Error>> {
        Ok("".to_string())
    }
}
