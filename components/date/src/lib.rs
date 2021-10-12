use std::error::Error;
use async_trait::async_trait;

use chrono;

pub struct DateResource<'a> {
    pub format: &'a str,
}

#[async_trait]
impl resource::Resource for DateResource<'_> {
    async fn fetch(&self) -> Result<String, Box<dyn Error>> {
        let now = chrono::offset::Local::now();
        Ok(format!("{}", now.format(self.format)))
    }
}
