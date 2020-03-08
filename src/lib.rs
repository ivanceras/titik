//! Titik is a crossplatform TUI widget library.
//! It uses crossterm as the underlying backend.
//!
pub use buffer::{
    Buffer,
    Cell,
};
pub use widget::{
    compute_layout,
    Box,
    Button,
    Control,
    Image,
};

mod buffer;
mod symbol;
mod widget;
