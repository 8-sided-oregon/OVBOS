use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};

const KEYBOARD_PORT: PortGeneric<u8, ReadWriteAccess> = Port::new(0x60);

pub fn get_key_char() -> Option<char> {
    let scancode: u8 = unsafe { KEYBOARD_PORT.read() };

    return match scancode {
        0x02 => Some('1'),
        0x03 => Some('2'),
        0x04 => Some('3'),
        0x05 => Some('4'),
        0x06 => Some('5'),
        0x07 => Some('6'),
        0x08 => Some('7'),
        0x09 => Some('8'),
        0x0a => Some('9'),
        0x0b => Some('0'),
        _ => Some("?".parse().unwrap()),
    };
}

pub fn get_keycode() -> u8 {
    return unsafe { KEYBOARD_PORT.read() };
}