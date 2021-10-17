use std::error::Error;

use async_process::Command;
use async_trait::async_trait;

#[async_trait]
pub trait SignalHandler: Sync {
    async fn signal(&self, sig: i32) -> Result<(), Box<dyn Error>>;
}

pub struct NoOpHandler;

#[async_trait]
impl SignalHandler for NoOpHandler {
    async fn signal(&self, _sig: i32) -> Result<(), Box<dyn Error>> {
        println!("no-op handler!");
        Ok(())
    }
}

pub struct ShHandler<'a> {
    pub code: &'a str,
}

#[async_trait]
impl SignalHandler for ShHandler<'_> {
    async fn signal(&self, _sig: i32) -> Result<(), Box<dyn Error>> {
        Command::new("sh")
            .arg("-c")
            .arg(self.code)
            .spawn()?
            .status()
            .await?;

        Ok(())
    }
}
