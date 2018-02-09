use std::io::prelude::*;
use std::error::Error;
use std::{io, thread, time};

pub struct Editor {
    row: i32,
    col: i32,
}

impl Editor {
    pub fn new(row: i32, col: i32) -> Editor {
        Editor { row: row, col: col }
    }

    fn read_key(&self) -> Result<u8, Box<Error>> {
        let mut c = [0];
        let stdin = io::stdin();
        stdin.lock().read(&mut c)?;
        Ok(c[0])
    }

    pub fn process_key_press(&self) -> Result<bool, Box<Error>> {
        let c = self.read_key()?;
        match c {
            24 => return Ok(false),
            _ => println!("\r{} : {}", c, c as char),
        }
        Ok(true)
    }

    pub fn refresh_screen(&self) {
        print!("{}[2J", 27 as char);
        print!("{}[H", 27 as char);
        self.draw_rows();
        print!("{}[H", 27 as char);
    }

    fn draw_rows(&self) {
        for _ in 0..24 {
            print!("{}\r\n", "~")
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        println!("exiting adamantium...");
        thread::sleep(time::Duration::from_millis(1000));
        print!("{}[2J", 27 as char);
        print!("{}[H", 27 as char);
    }
}
