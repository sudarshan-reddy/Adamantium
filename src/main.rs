extern crate adamantium;
extern crate termios;

use adamantium::term::Term;
use adamantium::editor;

fn main() {
    let mut t = Term::new().unwrap();
    t.enable_raw_mode().unwrap();
    loop {
        editor::refresh_screen();
        let ok = editor::process_key_press().unwrap();
        if !ok {
            break;
        }
    }
}
