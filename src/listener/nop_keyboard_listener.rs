use std::error::Error;

use super::common;
use common::KeyboardListenerProvider;

pub struct NopKeyboardListener {}

impl KeyboardListenerProvider for NopKeyboardListener {
    fn new(_callback: fn()) -> Result<NopKeyboardListener, Box<dyn Error>> {
        return Ok(NopKeyboardListener {});
    }

    fn start(&mut self) -> Result<&str, Box<dyn Error>> {
        println!(
            "Attempting to set a keyboard listener, which hasn't yet been \
                  implemented on this platform."
        );
        Ok("")
    }
    fn stop(&mut self) -> Result<&str, Box<dyn Error>> {
        println!(
            "Attempting to stop a keyboard listener, which hasn't yet been \
                  implemented on this platform."
        );
        Ok("")
    }
}
