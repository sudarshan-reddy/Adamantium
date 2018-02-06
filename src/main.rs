extern crate adamantium;
extern crate termios;

use adamantium::term::Term;
use adamantium::editor;

fn main() {
    let mut t = Term::new().unwrap();
    t.enable_raw_mode().unwrap();
    loop {
        let ok = editor::editor_process_key_press().unwrap();
        if !ok {
            break;
        }
    }
}
