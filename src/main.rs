extern crate adamantium;
extern crate termios;

use std::io;
use std::io::prelude::*;
use adamantium::Term;

fn main() {
    let mut t = Term::new().unwrap();
    t.enable_raw_mode().unwrap();
    let stdin = io::stdin();
    let mut c = [0];
    while let Ok(_) = stdin.lock().read(&mut c) {
        if c == [113] {
            break;
        }
        println!("{}, {}", c[0], c[0] as char);
    }
}
