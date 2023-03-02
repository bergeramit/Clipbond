use livesplit_hotkey::{Hook, Hotkey, KeyCode, Modifiers};
use std::{error::Error, thread};

use common::KeyboardListenerProvider;

use super::common;
pub struct OSXKeyboardListener {
    hook: Hook,
    hotkey: Hotkey,
    callback: fn(),
}

impl KeyboardListenerProvider for OSXKeyboardListener {
    fn new(callback: fn()) -> Result<OSXKeyboardListener, Box<dyn Error>> {
        return Ok(OSXKeyboardListener {
            hook: Hook::new().unwrap(),
            hotkey: Hotkey {
                key_code: KeyCode::KeyC,
                modifiers: Modifiers::META,
            },
            callback,
        });
    }

    fn start(&mut self) -> Result<&str, Box<dyn Error>> {
        let cb = self.callback;
        self.hook
            .register(self.hotkey, move || {
                thread::spawn(cb);
            })
            .map_err(|e| eprintln!("Failed to register hotkey: {}", e))
            .unwrap();
        Ok("Listening for Cmd+C...")
    }

    fn stop(&mut self) -> Result<&str, Box<dyn Error>> {
        self.hook.unregister(self.hotkey).unwrap();
        Ok("Done listening for Cmd+C.")
    }
}
