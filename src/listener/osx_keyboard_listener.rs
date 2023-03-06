use livesplit_hotkey::{Hook, Hotkey, KeyCode, Modifiers};
use std::{error::Error, thread};

use keyboard_listener_provider::KeyboardListenerProvider;

use super::keyboard_listener_provider;

pub struct OSXKeyboardListener {
    hook: Hook,
    hotkey: Hotkey,
    callback: fn(),
}

impl KeyboardListenerProvider for OSXKeyboardListener {
    fn new(callback: fn()) -> Result<OSXKeyboardListener, Box<dyn Error>> {
        let listener = Self::default();
        return Ok(OSXKeyboardListener {
            hook: listener.hook,
            hotkey: listener.hotkey,
            callback,
        });
    }

    fn start(&mut self) -> Result<&str, Box<dyn Error>> {
        let cb = self.callback;
        self.hook
            .register(self.hotkey, move || {
                thread::spawn(cb);
            })
            .map_err(|e| println!("ERROR: Failed to register hotkey: {}", e))
            .unwrap();
        Ok("Listening for Cmd+C...")
    }

    fn stop(&mut self) -> Result<&str, Box<dyn Error>> {
        self.hook.unregister(self.hotkey).unwrap();
        Ok("Done listening for Cmd+C.")
    }
}

impl Default for OSXKeyboardListener {
    fn default() -> Self {
        Self {
            hook: Hook::new().unwrap(),
            hotkey: Hotkey {
                key_code: KeyCode::KeyC,
                modifiers: Modifiers::META,
            },
            callback: || {},
        }
    }
}
