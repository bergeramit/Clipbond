use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub struct ClipboardManager {
    context: ClipboardContext,
}

impl ClipboardManager {
    pub fn new() -> Self {
        return Self {
            context: ClipboardContext::new().unwrap(),
        };
    }

    pub fn set_content(&mut self, content: String) {
        self.context.set_contents(content).unwrap();
    }

    pub fn get_content(&mut self) -> String {
        self.context.get_contents().unwrap()
    }
}
