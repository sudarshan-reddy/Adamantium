extern crate termios;
use termios::*;
use std::error::Error;

pub struct Term {
    stdin: i32,
    termios: Termios,
}

impl Term {
    pub fn new() -> Result<Term, Box<Error>> {
        let stdin = 0;
        let termios = Termios::from_fd(stdin)?;
        Ok(Term {
            stdin: stdin,
            termios: termios,
        })
    }

    pub fn enable_raw_mode(&mut self) -> Result<(), Box<Error>> {
        tcgetattr(self.stdin, &mut self.termios)?;
        self.termios.c_lflag &= !(ECHO | ICANON);
        tcsetattr(self.stdin, TCSAFLUSH, &self.termios)?;
        Ok(())
    }
}
