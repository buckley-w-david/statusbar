use x11rb::protocol::xproto::*;

use std::error::Error;
use async_trait::async_trait;

pub struct KeyIndicator<'a> {
    pub on: bool,
    pub toggle: bool,
    pub key: &'a str,
}

pub struct KeyboardIndicatorResource<'a> {
    pub caps_lock: KeyIndicator<'a>,
    pub num_lock: KeyIndicator<'a>,
}

#[async_trait]
impl resource::Resource for KeyboardIndicatorResource<'_> {
    async fn fetch(&self) -> Result<String, Box<dyn Error>> {
        let keyboard = {
            let (conn, _) = x11rb::connect(None)?;
            let x = conn.get_keyboard_control()?.reply()?; x
        };

        let caps_set = (keyboard.led_mask & 1) == 1;
        let num_set = (keyboard.led_mask & 2) == 1;

        let mut response = String::with_capacity(4);

        if self.caps_lock.on && self.caps_lock.toggle {
            let k = if caps_set { self.caps_lock.key.to_uppercase() } else { self.caps_lock.key.to_lowercase() };
            response.push_str(&k);
        } else if self.caps_lock.on && caps_set {
            response.push_str(self.caps_lock.key);
        }

        if self.num_lock.on && self.num_lock.toggle {
            let k = if num_set { self.num_lock.key.to_uppercase() } else { self.num_lock.key.to_lowercase() };
            response.push_str(&k)
        } else if self.num_lock.on && caps_set {
            response.push_str(self.num_lock.key);
        }

        Ok(response)
    }
}
