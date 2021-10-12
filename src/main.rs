mod blocks;

use std::time::{Instant, Duration};

use smol::channel::bounded;
use smol::{io, Async};

use std::os::unix::io::AsRawFd;

use timerfd::{SetTimeFlags, TimerFd, TimerState};


use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::wrapper::ConnectionExt as _;

use serde::Serialize;

use tinytemplate::TinyTemplate;

#[derive(Serialize)]
pub struct BlockOutput<'a> {
    pub left: &'a str,
    pub content: &'a str,
    pub right: &'a str,
}


/// Converts a [`nix::Error`] into [`std::io::Error`].
fn io_err(err: nix::Error) -> io::Error {
    match err {
        nix::Error::Sys(code) => code.into(),
        err => io::Error::new(io::ErrorKind::Other, Box::new(err)),
    }
}

/// Sleeps using an OS timer.
async fn sleep(dur: Duration) -> io::Result<()> {
    // Create an OS timer.
    let mut timer = TimerFd::new()?;
    timer.set_state(TimerState::Oneshot(dur), SetTimeFlags::Default);

    // When the OS timer fires, a 64-bit integer can be read from it.
    Async::new(timer)?
        .read_with(|t| nix::unistd::read(t.as_raw_fd(), &mut [0u8; 8]).map_err(io_err))
        .await?;
    Ok(())
}

fn build_status(blocks: &Vec<(&str, String)>, tt: &TinyTemplate) -> String {
    blocks
        .into_iter()
        .map(|b| {
            let out = BlockOutput {
                left: blocks::LEFT,
                content: &b.1,
                right: blocks::RIGHT,
            };
            match tt.render(b.0, &out) {
                Ok(s) => s,
                Err(_) => blocks::ERROR.to_string(), // TODO: logging?
            }
        })
        .collect::<Vec<String>>()
        .join(blocks::SEPARATOR)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None)?;

    let screen = &conn.setup().roots[screen_num];

    let (tx, rx) = bounded::<(usize, String)>(1);
    let mut tt = TinyTemplate::new();
    let mut buffers: Vec<(&str, String)> = Vec::new();

    for (i, &status_block) in blocks::BLOCKS.iter().enumerate() {
        tt.add_template(status_block.name, status_block.template)?;
        buffers.push((status_block.name, String::new()));

        let tx = tx.clone();
        smol::spawn(async move {
            loop {
                let start = Instant::now();

                let result = match status_block.resource.fetch().await {
                    Ok(content) => content,
                    Err(_) => blocks::ERROR.to_string(),
                };

                // For now, don't do anything if it failed
                match tx.send((i, result)).await {
                    Err(_) => return (), // TODO: Handle the error for real somehow
                    _ => (),
                };

                let elapsed = start.elapsed();
                if elapsed < status_block.interval {
                    match sleep(status_block.interval - elapsed).await {
                        Err(_) => return (), // TODO: Handle the error for real somehow
                        _ => (),
                    };
                }
            }
        }).detach();
    }

    smol::block_on(async {
        let mut old_status: String = String::new();
        loop {
            let (index, content) = rx.recv().await?;
            buffers[index].1 = content;
            let new_status = build_status(&buffers, &tt);
            // Only set the root WM_NAME if the status text has changed
            if old_status != new_status {
                conn.change_property8(
                    PropMode::REPLACE,
                    screen.root,
                    AtomEnum::WM_NAME,
                    AtomEnum::STRING,
                    new_status.as_bytes(),
                )?;
                conn.flush()?;
                old_status = new_status;
            }
        }
    })
}
