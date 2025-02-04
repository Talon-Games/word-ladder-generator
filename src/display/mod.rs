pub mod string_input;

use crossterm::{cursor, terminal, ExecutableCommand};
use std::io;

fn refresh_display(lines: i32) {
    for _ in 0..lines {
        io::stdout().execute(cursor::MoveUp(1)).unwrap();
        io::stdout()
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
    }
}
