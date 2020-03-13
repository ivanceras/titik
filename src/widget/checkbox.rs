use crate::{
    buffer::{
        Buffer,
        Cell,
    },
    symbol,
    symbol::{
        bar,
        line,
        rounded,
    },
    LayoutTree,
    Widget,
};
use std::any::Any;
use stretch::{
    geometry::Size,
    style::{
        Dimension,
        Style,
    },
};

#[derive(Default, Debug, PartialEq)]
pub struct Checkbox {
    pub label: String,
    pub is_checked: bool,
}

impl Checkbox {
    pub fn new<S>(label: S) -> Self
    where
        S: ToString,
    {
        Checkbox {
            label: label.to_string(),
            ..Default::default()
        }
    }

    pub fn set_label<S: ToString>(&mut self, label: S) {
        self.label = label.to_string();
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.is_checked = checked;
    }
}

impl Widget for Checkbox {
    fn style(&self) -> Style {
        Style {
            size: Size {
                width: Dimension::Points((self.label.len() + 3) as f32),
                height: Dimension::Points(1.0),
            },
            ..Default::default()
        }
    }

    /// draw this button to the buffer, with the given computed layout
    fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let box_symbol = if self.is_checked {
            symbol::BOX_CHECKED
        } else {
            symbol::BOX_UNCHECKED
        };
        buf.set_symbol(loc_x + 1, loc_y + 1, box_symbol);

        for (t, ch) in self.label.chars().enumerate() {
            buf.set_symbol(loc_x + 4 + t, loc_y + 1, ch);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
