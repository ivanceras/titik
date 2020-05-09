use crossterm::cursor;
use std::io::Write;

#[derive(Debug)]
pub enum Cmd {
    MoveTo(usize, usize),
    ShowCursor,
}

impl Cmd {
    pub fn execute(&self, w: &mut dyn Write) -> crossterm::Result<()> {
        match self {
            Cmd::MoveTo(x, y) => {
                crossterm::queue!(w, cursor::MoveTo(*x as u16, *y as u16))
            }
            Cmd::ShowCursor => crossterm::queue!(w, cursor::Show),
        }
    }
}
