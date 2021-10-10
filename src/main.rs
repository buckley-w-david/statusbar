mod blocks;

use std::{thread, time};

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::wrapper::ConnectionExt as _;

use serde::Serialize;

use tinytemplate::TinyTemplate;

#[derive(Serialize)]
pub struct BlockOutput<'a> {
    pub left : &'a str,
    pub content : String,
    pub right : &'a str
}

// FIXME: There sure are a lot of `unwrap`s in here, might want to do something about that
fn build_status(blocks: &'static [&'static blocks::StatusBlock], tt: &TinyTemplate) -> String {
    blocks
        .into_iter()
        .map(|b| {
            let content = match b.block.perform() {
                Ok(content) => content,
                Err(_) => blocks::ERROR.to_string(), // TODO: logging?
            };

            let out = BlockOutput {
                left: blocks::LEFT,
                content: content,
                right: blocks::RIGHT,
            };
            tt.render(b.name, &out).unwrap()
        } )
        .collect::<Vec<String>>()
        .join(blocks::SEPARATOR)
}

// TODO: loop and schedule block invocations
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None).unwrap();

    let screen = &conn.setup().roots[screen_num];

    let mut tt = TinyTemplate::new();
    for &status_block in blocks::BLOCKS {
        tt.add_template(status_block.name, status_block.template).unwrap();
    };

    loop {
        let status = build_status(blocks::BLOCKS, &tt);
        println!("{}", status);
        conn.change_property8(
            PropMode::REPLACE,
            screen.root,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            status.as_bytes(),
            )
            .unwrap();
        conn.flush().unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}
