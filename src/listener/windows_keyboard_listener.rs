use common::*;
use futures::{select, FutureExt};
use std::sync::mpsc::{channel, Receiver};
use std::{
    error::Error,
    thread,
    time::{self, Instant},
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

use common::KeyboardListenerProvider;

pub struct WindowsKeyboardListener {
    callback: fn(),
}

impl KeyboardListenerProvider for WindowsKeyboardListener {
    fn new(callback: fn()) -> Result<WindowsKeyboardListener, Box<dyn Error>> {
        return Ok(WindowsKeyboardListener { callback });
    }

    fn start(&mut self) -> Result<&str, Box<dyn Error>> {
        println!("Listening for Ctrl+C...");
        let (tx, rx) = channel();

        thread::spawn(move || {
            let event_loop = EventLoop::new();
            event_loop.run(move |event, _, control_flow| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        if input.virtual_keycode == Some(VirtualKeyCode::C)
                            && input.modifiers.ctrl()
                        {
                            let _ = tx.send(());
                        }
                    }
                    _ => {}
                },
                _ => {}
            });
        });

        // TODO: move this loop outside (callback)
        loop {
            select! {
                recv(rx) -> _ => callback(),
            }
        }
        Ok("Done listening for Ctrl+C.")
    }
}
