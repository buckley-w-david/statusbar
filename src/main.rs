mod blocks;

use std::{thread, time::Instant};

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::wrapper::ConnectionExt as _;

use serde::Serialize;

use tinytemplate::TinyTemplate;

#[derive(Serialize)]
pub struct BlockOutput<'a> {
    pub left: &'a str,
    pub content: String,
    pub right: &'a str,
}

fn build_status(blocks: &[&blocks::StatusBlock], tt: &TinyTemplate) -> String {
    blocks
        .into_iter()
        .map(|b| {
            let content = match b.resource.fetch() {
                Ok(content) => content,
                Err(_) => blocks::ERROR.to_string(), // TODO: logging?
            };

            let out = BlockOutput {
                left: blocks::LEFT,
                content: content,
                right: blocks::RIGHT,
            };
            match tt.render(b.name, &out) {
                Ok(s) => s,
                Err(_) => blocks::ERROR.to_string(), // TODO: logging?
            }
        })
        .collect::<Vec<String>>()
        .join(blocks::SEPARATOR)
}

// TODO: Multithreading? Sounds like a lot of work
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None)?;

    let screen = &conn.setup().roots[screen_num];

    let mut tt = TinyTemplate::new();
    for &status_block in blocks::BLOCKS {
        tt.add_template(status_block.name, status_block.template)?;
    }

    let mut old_status: String = String::new();
    loop {
        let start = Instant::now();
        let new_status = build_status(blocks::BLOCKS, &tt);
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

        let elapsed = start.elapsed();
        if elapsed < blocks::INTERVAL {
            thread::sleep(blocks::INTERVAL - elapsed);
        }
    }
}
