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
pub use find_node::{
    find_widget,
    find_widget_by_id,
    find_widget_by_id_mut,
    find_widget_mut,
};
pub use input_buffer::InputBuffer;
pub use layout::{
    compute_layout,
    find_layout,
    set_focused_node,
    widget_hit_at,
    widget_node_idx_at,
    LayoutTree,
};
pub use renderer::Dispatch;
pub use sauron_vdom::Callback;
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

mod area_buffer;
mod buffer;
mod cmd;
pub mod command;
mod find_node;
mod input_buffer;
mod layout;
pub mod renderer;
#[allow(unused)]
mod symbol;
mod widget;
