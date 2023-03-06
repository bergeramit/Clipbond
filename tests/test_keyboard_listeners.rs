#[cfg(test)]
mod tests {
    use clipbond::listener::KeyboardListener;
    use clipbond::listener::KeyboardListenerProvider;
    use enigo::*;

    static mut INDICATOR: bool = false;

    fn callback() {
        unsafe {
            INDICATOR = !INDICATOR;
        }
        println!("callback was called");
    }

    fn press_key(modifier: Key) {
        let mut enigo = Enigo::new();
        enigo.key_down(modifier);
        enigo.key_click(Key::Layout('c'));
        enigo.key_up(modifier);
    }

    #[test]
    fn test_osx_keyboard_listener_active() {
        unsafe {
            INDICATOR = false;
        }
        let mut listener = KeyboardListener::new(callback).unwrap();
        listener.start().unwrap();
        press_key(Key::Meta);
        listener.stop().unwrap();
        unsafe {
            assert_eq!(INDICATOR, true);
        }
    }

    #[test]
    fn test_osx_keyboard_listener_inactive() {
        unsafe {
            INDICATOR = false;
        }
        let mut listener = KeyboardListener::new(callback).unwrap();
        listener.start().unwrap();
        press_key(Key::Meta);
        listener.stop().unwrap();
        unsafe {
            assert_eq!(INDICATOR, true);
        }

        press_key(Key::Meta);
        unsafe {
            assert_eq!(INDICATOR, true);
        }
    }
}
