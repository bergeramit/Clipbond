use std::error::Error;

pub trait KeyboardListenerProvider: Sized {
    fn new(callback: fn()) -> Result<Self, Box<dyn Error>>;
    fn start(&mut self) -> Result<&str, Box<dyn Error>>;
    fn stop(&mut self) -> Result<&str, Box<dyn Error>>;
}
