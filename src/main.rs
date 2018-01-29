extern crate termios;

use std::io;
use std::io::prelude::*;
use std::process;
use termios::*;

fn main() {
    enable_raw_mode();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let op = line.map(|l| {
            if l == "q" {
                process::exit(0);
            }
        });
        if op.is_err() {
            process::exit(0);
        };
    }
}

fn enable_raw_mode() {
    let stdin = 0;
    let mut termios = Termios::from_fd(stdin).unwrap();
    tcgetattr(stdin, &mut termios).unwrap();
    termios.c_lflag &= !ECHO;
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}
