use std::io::stdout;
use crossterm::{
    ExecutableCommand,
    cursor,
    terminal::{
        Clear,
        ClearType
    }
};

pub fn clear() {
    stdout()
        .execute(Clear(ClearType::All)).unwrap()
        .execute(cursor::MoveTo(0, 0)).unwrap();
}
