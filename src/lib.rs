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
        let mut raw = self.termios;
        tcgetattr(self.stdin, &mut raw)?;
        raw.c_lflag &= !(ICRNL | IXON);
        raw.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
        raw.c_oflag &= !(OPOST);
        raw.c_cc[VMIN] = 0;
        raw.c_cc[VTIME] = 1;
        tcsetattr(self.stdin, TCSAFLUSH, &raw)?;
        Ok(())
    }

    pub fn disable_raw_mode(&self) -> Result<(), Box<Error>> {
        tcsetattr(self.stdin, TCSAFLUSH, &self.termios)?;
        Ok(())
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        Term::disable_raw_mode(&self).unwrap();
    }
}
