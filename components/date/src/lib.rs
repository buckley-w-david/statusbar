use std::error::Error;

use chrono;

pub struct DateResource<'a> {
    pub format: &'a str,
}

impl resource::Resource for DateResource<'_> {
    fn fetch(&self) -> Result<String, Box<dyn Error>> {
        let now = chrono::offset::Local::now();
        Ok(format!("{}", now.format(self.format)))
    }
}
