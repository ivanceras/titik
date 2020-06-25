use crossterm::cursor;
use std::io::Write;

/// creates a Cmd representation which translate to actual tty commands
#[derive(Debug)]
pub enum Cmd {
    /// Move the cursor to x,y location
    MoveTo(usize, usize),
    /// show the cursor
    ShowCursor,
}

impl Cmd {
    /// execute the command to the supplied writable buffer (ie: stdout)
    pub fn execute(&self, w: &mut dyn Write) -> crossterm::Result<()> {
        match self {
            Cmd::MoveTo(x, y) => {
                crossterm::queue!(w, cursor::MoveTo(*x as u16, *y as u16))
            }
            Cmd::ShowCursor => crossterm::queue!(w, cursor::Show),
        }
    }
}
