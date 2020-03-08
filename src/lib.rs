//! Titik is a crossplatform TUI widget library.
//! It uses crossterm as the underlying backend.
//!
pub use buffer::{
    Buffer,
    Cell,
};
pub use control::{
    compute_layout,
    Box,
    Button,
    Checkbox,
    Control,
    Image,
    Radio,
    TextInput,
};
pub use crossterm;
pub use stretch;

mod buffer;
mod control;
mod symbol;

pub mod command {
    use crossterm::{
        cursor,
        event::EnableMouseCapture,
        style,
        terminal,
        terminal::ClearType,
    };
    use std::{
        io,
        io::Write,
    };

    pub fn reset_top<W: Write>(w: &mut W) -> crossterm::Result<()> {
        crossterm::queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1)
        )
    }

    pub fn init<W: Write>(w: &mut W) -> crossterm::Result<()> {
        crossterm::execute!(w, terminal::EnterAlternateScreen)?;
        crossterm::execute!(w, EnableMouseCapture)?;
        terminal::enable_raw_mode()
    }

    pub fn finalize<W: Write>(w: &mut W) -> crossterm::Result<()> {
        crossterm::execute!(
            w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;
        terminal::disable_raw_mode()
    }
}
