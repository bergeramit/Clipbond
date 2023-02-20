#[cfg(test)]
mod tests {
    use clipbond::session::clipboard_manager::{ClipboardManager};

    #[test]
    fn test_clipboard_manager_set_and_get() {
        let random_value = String::from("testing value");
        let mut manager = ClipboardManager::new();
        manager.set_content(random_value.clone());
        let clipboard_value = manager.get_content();

        assert_eq!(clipboard_value, random_value);
    }
}