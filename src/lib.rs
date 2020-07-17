//! [![Build Status](https://api.travis-ci.com/ivanceras/titik.svg?branch=master)](https://travis-ci.com/github/ivanceras/titik)
//! [![Latest Version](https://img.shields.io/crates/v/titik.svg)](https://crates.io/crates/titik)

//!
//! Titik is a crossplatform TUI widget library with the goal of being able to interact
//! intuitively on these widgets.
//!
//! ![Screenshot](https://ivanceras.github.io/screenshots/sauron-titik.gif)
//!
//! It uses [`crossterm`](https://crates.io/crates/crossterm) as the underlying backend.
//!
//! To run the demo use the following command:
//! ```sh
//! cargo run --example demo 2>/dev/null
//! ```
//! Note: `2>/dev/null` is sending the debugging log from `eprintln` into the `/dev/null` device
//!
//! Without doing so, will result a flicker in your screen caused by debugging info and tui mixed
//! in one terminal output.
//!
//! Alternatively, you can pipe the debugging log from `eprintln` into a file say `/tmp/debug.log`
//! by doing so:
//! ```sh
//! cargo run --example demo 2>/tmp/debug.log
//! ```
//!
//! You can then open a new terminal and tail the `/tmp/debug.log` file
//! ```sh
//! tail -f /tmp/debug.log
//! ```
//!
//!
//#![deny(warnings)]
//#![deny(
//    missing_docs,
//    missing_copy_implementations,
//    trivial_casts,
//    trivial_numeric_casts,
//    unstable_features,
//    unused_import_braces
//)]
pub use buffer::{Buffer, Cell};
pub use cmd::Cmd;
pub use crossterm;
pub use event::Event;
pub use find_node::{
    find_widget, find_widget_by_id, find_widget_by_id_mut, find_widget_mut,
};
pub use input_buffer::InputBuffer;
pub use layout::LayoutTree;
pub use mt_dom::{self, Callback};
pub use renderer::{Dispatch, Renderer};
pub use stretch;
pub use value::Value;
pub use widget::{
    Button, Checkbox, FlexBox, GroupBox, Image, ListBox, Radio, Slider,
    SvgImage, TabBox, TextArea, TextInput, Widget,
};

mod area_buffer;
mod buffer;
mod cmd;
pub mod command;
pub mod event;
mod find_node;
mod input_buffer;
pub(crate) mod layout;
pub mod renderer;
#[allow(unused)]
mod symbol;
mod value;
mod widget;
