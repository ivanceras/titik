//! command to the terminal, such as moving the cursor and clearing the screen
//!

use crate::crossterm::{
    self, cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute, queue, style, terminal,
    terminal::ClearType,
};
use std::io::Stdout;
use std::io::Write;

pub(crate) fn reset_top(w: &mut Stdout) -> crossterm::Result<()> {
    queue!(
        w,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(1, 1)
    )
}

pub(crate) fn init(w: &mut Stdout) -> crossterm::Result<()> {
    execute!(w, terminal::EnterAlternateScreen, EnableMouseCapture)?;
    terminal::enable_raw_mode()
}

pub(crate) fn finalize(w: &mut Stdout) -> crossterm::Result<()> {
    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal::disable_raw_mode()
}
