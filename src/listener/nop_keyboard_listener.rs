use std::error::Error;

use super::keyboard_listener_provider;
use keyboard_listener_provider::KeyboardListenerProvider;

pub struct NopKeyboardListener {}

impl KeyboardListenerProvider for NopKeyboardListener {
    fn new(_callback: fn()) -> Result<NopKeyboardListener, Box<dyn Error>> {
        return Ok(NopKeyboardListener {});
    }

    fn start(&mut self) -> Result<&str, Box<dyn Error>> {
        panic!(
            "Attempting to set a keyboard listener, which hasn't yet been \
        implemented on this platform.",
        )
    }
    fn stop(&mut self) -> Result<&str, Box<dyn Error>> {
        panic!(
            "Attempting to stop a keyboard listener, which hasn't yet been \
                  implemented on this platform.",
        )
    }
}
