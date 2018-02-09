use termios::*;
use std::error::Error;
use std::fmt;
use libc::{c_ulong, c_ushort, ioctl, STDOUT_FILENO};

const TIOCGWINSZ: c_ulong = 0x40087468;

pub struct Term {
    stdin: i32,
    termios: Termios,
}

pub struct Dimensions {
    pub row: i32,
    pub col: i32,
}

#[repr(C)]
struct WinSize {
    row: c_ushort,
    col: c_ushort,
    xpixel: c_ushort,
    ypixel: c_ushort,
}

#[derive(Debug, Clone)]
pub struct TermErr {
    desc: String,
}

impl TermErr {
    pub fn new(msg: String) -> Self {
        TermErr { desc: msg }
    }
}

impl fmt::Display for TermErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "term error")
    }
}

impl Error for TermErr {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
    fn cause(&self) -> Option<&Error> {
        None
    }
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
        raw.c_lflag &= !(ICRNL | IXON | ECHO | ICANON | IEXTEN | ISIG);
        raw.c_oflag &= !(OPOST);
        raw.c_cc[VMIN] = 0;
        raw.c_cc[VTIME] = 3;
        tcsetattr(self.stdin, TCSAFLUSH, &raw)?;
        Ok(())
    }

    pub fn disable_raw_mode(&self) -> Result<(), Box<Error>> {
        tcsetattr(self.stdin, TCSAFLUSH, &self.termios)?;
        Ok(())
    }

    pub fn get_term_win_size(&self) -> Result<Dimensions, TermErr> {
        let w = WinSize {
            row: 0,
            col: 0,
            xpixel: 0,
            ypixel: 0,
        };
        let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) };

        match r {
            0 => Ok(Dimensions {
                row: w.row as i32,
                col: w.col as i32,
            }),
            _ => Err(TermErr::new(String::from("resource unavailable"))),
        }
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        Term::disable_raw_mode(&self).unwrap();
    }
}
