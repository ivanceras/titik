//! Titik is a crossplatform TUI widget library.
//! It uses crossterm as the underlying backend.
//!
//#![deny(warnings)]
pub use area_buffer::AreaBuffer;
pub use buffer::{
    Buffer,
    Cell,
};
pub use cmd::Cmd;
pub use crossterm;
pub use input_buffer::InputBuffer;
pub use sauron_vdom::Callback;
pub use layout::{
    compute_layout,
    find_layout,
    find_widget,
    find_widget_mut,
    set_focused_node,
    widget_hit_at,
    widget_node_idx_at,
    LayoutTree,
};
pub use stretch;
pub use widget::{
    Button,
    Checkbox,
    FlexBox,
    Image,
    Radio,
    SvgImage,
    TextArea,
    TextInput,
    Widget,
};
pub use renderer::Renderer;
pub use renderer::Dispatch;

mod area_buffer;
mod buffer;
mod cmd;
mod input_buffer;
mod layout;
#[allow(unused)]
mod symbol;
mod widget;
pub mod command;
pub mod renderer;
