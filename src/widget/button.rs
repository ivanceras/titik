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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Button {
    pub label: String,
    pub is_rounded: bool,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl Button {
    pub fn new<S>(label: S) -> Self
    where
        S: ToString,
    {
        Button {
            label: label.to_string(),
            is_rounded: true,
            ..Default::default()
        }
    }

    pub fn set_label<S: ToString>(&mut self, label: S) {
        self.label = label.to_string();
    }

    pub fn set_size(&mut self, width: Option<f32>, height: Option<f32>) {
        self.width = width;
        self.height = height;
    }

    pub fn set_rounded(&mut self, rounded: bool) {
        self.is_rounded = rounded;
    }
}

impl Widget for Button {
    fn style(&self) -> Style {
        Style {
            size: Size {
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
                    //Dimension::Points((self.label.len() + 1) as f32)
                    Dimension::Percent(0.95)
                },
                height: if let Some(height) = self.height {
                    Dimension::Points(height)
                } else {
                    Dimension::Points(3.0)
                },
            },
            ..Default::default()
        }
    }

    /// draw this button to the buffer, with the given computed layout
    fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) {
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
        for (t, ch) in self.label.chars().enumerate() {
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}
