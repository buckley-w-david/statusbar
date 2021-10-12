use std::str;
use std::error::Error;

use async_trait::async_trait;
use async_process::Command;

pub struct ShResource<'a> {
    pub code: &'a str,
}

#[async_trait]
impl resource::Resource for ShResource<'_> {
    async fn fetch(&self) -> Result<String, Box<dyn Error>> {
        let out = Command::new("sh")
            .arg("-c")
            .arg(self.code)
            .output().await?;

        Ok(str::from_utf8(&out.stdout)?.to_string())
    }
}
