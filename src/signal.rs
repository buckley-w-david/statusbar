use std::error::Error;
use std::process::Command;
use std::thread;

pub trait SignalHandler: Sync {
    fn signal(&'static self, sig: i32) -> Result<(), Box<dyn Error>>;
}

pub struct NoOpHandler;

impl SignalHandler for NoOpHandler {
    fn signal(&self, _sig: i32) -> Result<(), Box<dyn Error>> {
        println!("no-op handler!");
        Ok(())
    }
}

pub struct ShHandler<'a> {
    pub code: &'a str,
}

impl SignalHandler for ShHandler<'static> {
    fn signal(&'static self, _sig: i32) -> Result<(), Box<dyn Error>> {
        println!("sh handler!");
        thread::Builder::new()
            .spawn(move || {
                let mut c = Command::new("sh")
                    .arg("-c")
                    .arg(self.code)
                    .spawn()
                    .expect("failed to execute sh handler");
                c.wait().expect("failed to wait for process");
            })
            .expect("cannot spawn executor thread");

        Ok(())
    }
}
