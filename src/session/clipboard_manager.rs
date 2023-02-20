use cli_clipboard::{ClipboardProvider};

pub struct ClipboardManager {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    context: cli_clipboard::linux_clipboard::LinuxClipboardContext,

    #[cfg(target_os = "windows")]
    context: cli_clipboard::windows_clipboard::WindowsClipboardContext,
}

impl ClipboardManager {
    pub fn new() -> Self {
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        return ClipboardManager {
            context: cli_clipboard::linux_clipboard::LinuxClipboardContext::new().unwrap()
        };

        #[cfg(target_os = "windows")]
        return ClipboardManager {
            context: cli_clipboard::windows_clipboard::WindowsClipboardContext::new().unwrap()
        };
    }

    pub fn set_content(&mut self, content: String) {
        self.context.set_contents(content).unwrap();
    }

    pub fn get_content(&mut self) -> String {
        self.context.get_contents().unwrap()
    }
}