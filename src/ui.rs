use std::io;
use std::io::Write;
use std::io::{Error, ErrorKind};
pub fn get_user_input(buffer: &mut String) -> Result<usize, Error> {
    buffer.clear();//read_line concatanates to the buffer, so clear it in case it has been used before
    let ret = io::stdin().read_line(buffer);
    *buffer = buffer.trim().to_string();
    return ret;
}

pub fn get_user_selection() -> Result<char, Error> {
    let mut buffer : String = String::new();
    match get_user_input(&mut buffer) {
        Ok(_) => {
            if buffer.len() != 1 {
                Err(Error::new(ErrorKind::Other, "Expected single character"))
            } else {
                Ok(buffer.as_bytes()[0] as char)
            }
        },
        Err(e) => Err(e),
    }
    //make sure the line only has 1 character
}

pub fn clear_screen() {
    let cmd : [u8; 12] = [0x1bu8, 0x5bu8, 0x33u8, 0x3bu8, 0x4au8, 0x1bu8, 0x5bu8, 0x48u8, 0x1bu8, 0x5bu8, 0x32u8, 0x4au8];
    io::stdout().write(&cmd).unwrap();
}

pub fn display(text : String) {
    clear_screen();
    io::stdout().write(text.as_bytes()).unwrap();
}
