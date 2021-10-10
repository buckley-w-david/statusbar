use std::error::Error;

use chrono;

pub struct DateBlock<'a> {
    pub format: &'a str,
}

impl block::Block for DateBlock<'_> {
    fn perform(&self) -> Result<String, Box<dyn Error>> {
        let now = chrono::offset::Local::now();
        Ok(format!("{}", now.format(self.format)))
    }
}
