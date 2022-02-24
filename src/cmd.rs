use crossterm::cursor;
use std::io::Stdout;
use std::io::Write;

/// creates a Cmd representation which translate to actual tty commands
#[derive(Debug, Clone, Copy)]
pub enum Cmd {
    /// Move the cursor to x,y location
    MoveTo(usize, usize),
    /// show the cursor
    ShowCursor,
}

impl Cmd {
    /// queue the command to the supplied writable buffer (ie: stdout)
    pub fn queue(&self, w: &mut Stdout) -> crossterm::Result<()> {
        match self {
            Cmd::MoveTo(x, y) => {
                crossterm::queue!(w, cursor::MoveTo(*x as u16, *y as u16))
            }
            Cmd::ShowCursor => crossterm::queue!(w, cursor::Show),
        }
    }
}
