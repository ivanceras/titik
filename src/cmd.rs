use crossterm::cursor;
use std::io::Write;

#[derive(Debug)]
pub enum Cmd {
    MoveTo(usize, usize),
}

impl Cmd {
    pub fn execute<W: Write>(&self, w: &mut W) -> crossterm::Result<()> {
        match self {
            Cmd::MoveTo(x, y) => {
                crossterm::queue!(w, cursor::MoveTo(*x as u16, *y as u16))
            }
        }
    }
}
