extern crate adamantium;
extern crate termios;

use adamantium::term::Term;
use adamantium::editor::Editor;

fn main() {
    let mut t = Term::new().unwrap();
    let ws = t.get_term_win_size().unwrap();
    let editor = Editor::new(ws.row, ws.col);
    t.enable_raw_mode().unwrap();
    loop {
        editor.refresh_screen();
        let ok = editor.process_key_press().unwrap();
        if !ok {
            break;
        }
    }
}
