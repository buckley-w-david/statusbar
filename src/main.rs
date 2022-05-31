mod blocks;
mod signal;

use std::cmp;
use std::time::Instant;
use std::thread;
use std::panic::catch_unwind;
use std::sync::Arc;

use async_channel::bounded;
use async_executor::Executor;
use async_io::Timer;
use futures_lite::future;

use libc::{SIGRTMAX, SIGRTMIN};

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

    let ex = Arc::new(Executor::new());
    for (i, &status_block) in blocks::BLOCKS.iter().enumerate() {
        tt.add_template(status_block.name, status_block.template)?;
        buffers.push((status_block.name, String::new()));

        let tx = tx.clone();
        ex.spawn(async move {
            loop {
                let start = Instant::now();

                let result = match status_block.resource.fetch().await {
                    Ok(content) => content,
                    Err(_) => blocks::ERROR.to_string(),
                };

                match tx.send((i, result)).await {
                    Err(_) => return (), // TODO: Handle the error for real somehow
                    _ => (),
                };

                let elapsed = start.elapsed();
                if elapsed < status_block.interval {
                    Timer::after(status_block.interval - elapsed).await;
                }
            }
        }).detach();
    }


    let ex1 = ex.clone();
    thread::Builder::new()
        .spawn(move || {
            let (sigmin, sigmax) = (SIGRTMIN(), SIGRTMAX());
            loop {
                let signals = (sigmin..cmp::min(sigmax, sigmin+blocks::BLOCKS.len() as i32)).collect::<Vec<_>>();
                let mut signals = signal_hook::iterator::Signals::new(&signals).unwrap(); // FIXME: unwrap
                for sig in signals.forever() {
                    let i = sig - sigmin;
                    ex1.spawn(async move {
                        blocks::BLOCKS[i as usize]
                            .signal_handler
                            .signal(i)
                            .await
                            .unwrap(); // FIXME: unwrap
                    }).detach();
                }
            }
        })
        .expect("cannot spawn signal handler thread");


    thread::Builder::new()
        .spawn(move || {
            loop {
                catch_unwind(|| async_io::block_on(ex.run(future::pending::<()>()))).ok();
            }
        })
        .expect("cannot spawn executor thread");

    async_io::block_on(async {
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
