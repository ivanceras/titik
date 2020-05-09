use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    style, terminal,
    terminal::ClearType,
};
use std::io::Write;

pub fn move_to(w: &mut dyn Write) -> crossterm::Result<()> {
    crossterm::execute!(w, cursor::Hide, cursor::MoveTo(1, 1))
}

pub fn reset_top(w: &mut dyn Write) -> crossterm::Result<()> {
    crossterm::queue!(
        w,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(1, 1)
    )
}

pub fn init(w: &mut dyn Write) -> crossterm::Result<()> {
    crossterm::execute!(w, terminal::EnterAlternateScreen, EnableMouseCapture)?;
    terminal::enable_raw_mode()
}

pub fn finalize(w: &mut dyn Write) -> crossterm::Result<()> {
    crossterm::execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal::disable_raw_mode()
}
