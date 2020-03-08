use super::{
    Control,
    LayoutTree,
};
use crate::{
    buffer::{
        Buffer,
        Cell,
    },
    symbol::{
        bar,
        line,
        rounded,
    },
};
use stretch::{
    geometry::Size,
    node::{
        Node,
        Stretch,
    },
    number::Number,
    result::Layout,
    style::{
        Dimension,
        Style,
    },
};

#[derive(Default)]
pub struct TextInput {
    pub value: String,
    pub is_rounded: bool,
    pub is_focused: bool,
}

impl TextInput {
    pub fn new<S>(value: S) -> Self
    where
        S: ToString,
    {
        TextInput {
            value: value.to_string(),
            is_rounded: false,
            ..Default::default()
        }
    }

    pub fn set_label<S: ToString>(&mut self, value: S) {
        self.value = value.to_string();
    }

    pub fn style(&self) -> Style {
        Style {
            size: Size {
                width: Dimension::Points(20.0),
                height: Dimension::Points(3.0),
            },
            ..Default::default()
        }
    }

    pub fn set_rounded(&mut self, rounded: bool) {
        self.is_rounded = rounded;
    }

    pub fn set_focus(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    /// draw this button to the buffer, with the given computed layout
    pub fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let width = layout.size.width.round() as usize;
        let height = layout.size.height.round() as usize;
        for i in 0..width {
            buf.set_symbol(loc_x + i, loc_y + 1, line::HORIZONTAL);
            buf.set_symbol(loc_x + i, loc_y + height, line::HORIZONTAL);
        }
        for j in 0..height {
            buf.set_symbol(loc_x, loc_y + 1 + j, line::VERTICAL);
            buf.set_symbol(loc_x + width, loc_y + 1 + j, line::VERTICAL);
        }
        for (t, ch) in self.value.chars().enumerate() {
            buf.set_symbol(loc_x + 1 + t, loc_y + 2, ch);
        }

        let top_left = if self.is_rounded {
            rounded::TOP_LEFT
        } else {
            line::TOP_LEFT
        };
        let top_right = if self.is_rounded {
            rounded::TOP_RIGHT
        } else {
            line::TOP_RIGHT
        };
        let bottom_left = if self.is_rounded {
            rounded::BOTTOM_LEFT
        } else {
            line::BOTTOM_LEFT
        };
        let bottom_right = if self.is_rounded {
            rounded::BOTTOM_RIGHT
        } else {
            line::BOTTOM_RIGHT
        };
        buf.set_symbol(loc_x, loc_y + 1, top_left);
        buf.set_symbol(loc_x, loc_y + height, bottom_left);
        buf.set_symbol(loc_x + width, loc_y + 1, top_right);
        buf.set_symbol(loc_x + width, loc_y + height, bottom_right);
    }
}

impl From<TextInput> for Control {
    fn from(btn: TextInput) -> Self {
        Control::TextInput(btn)
    }
}
