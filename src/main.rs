extern crate adamantium;
extern crate termios;

use std::io;
use std::io::prelude::*;
use adamantium::Term;

fn main() {
    let mut t = Term::new().unwrap();
    t.enable_raw_mode().unwrap();
    let stdin = io::stdin();
    loop {
        let mut c = [0];
        stdin.lock().read(&mut c).unwrap();
        if c == [113] {
            break;
        }
        println!("\r{}, {}", c[0], c[0] as char);
    }
}
