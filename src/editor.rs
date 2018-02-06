use std::io;
use std::io::prelude::*;
use std::error::Error;

pub fn editor_read_key() -> Result<u8, Box<Error>> {
    let mut c = [0];
    let stdin = io::stdin();
    stdin.lock().read(&mut c)?;
    Ok(c[0])
}

pub fn editor_process_key_press() -> Result<bool, Box<Error>> {
    let c = editor_read_key()?;
    match c {
        24 => return Ok(false),
        _ => println!("\r{} : {}", c, c as char),
    }
    Ok(true)
}
