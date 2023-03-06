mod keyboard_listener_provider;

pub use keyboard_listener_provider::KeyboardListenerProvider;

#[cfg(target_os = "windows")]
pub mod windows_keyboard_listener;

#[cfg(target_os = "macos")]
pub mod osx_keyboard_listener;

pub mod nop_keyboard_listener;

#[cfg(target_os = "windows")]
pub type KeyboardListener = windows_keyboard_listener::WindowsKeyboardListener;

#[cfg(target_os = "macos")]
pub type KeyboardListener = osx_keyboard_listener::OSXKeyboardListener;

#[cfg(not(any(target_os = "macos", target_os = "windows",)))]
pub type KeyboardListener = nop_keyboard_listener::NopKeyboardListener;
