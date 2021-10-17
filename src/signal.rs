use std::error::Error;

use async_process::Command;
use async_trait::async_trait;

// TODO: When signal_hook unseals signal_hook::iterator::exfiltrator::Exfiltrator implement an
//       Exfiltrator that extracts siginfo_t.si_value(), this will allow better compatability
//       with statuscmd patch for dwm since it makes use of the si_value
//
// Alternative: use the underlying crate signal_hook_registry
//
// Example: https://git.meli.delivery/meli/meli/src/branch/master/src/bin.rs
// This is an example of doing so, I would do something similar, except instead of only using it
// for one of the signals I'm interested in (SIGALRM in the example), I would use it for all
// signals
//
// Very annoying that I would have to do without the nice abstractions in signal_hook


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
