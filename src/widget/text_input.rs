use crate::{
    buffer::{
        Buffer,
        Cell,
    },
    symbol::{
        bar,
        line,
        rounded,
        thick_line,
    },
    Cmd,
    InputBuffer,
    LayoutTree,
    Widget,
};
use crossterm::{
    cursor,
    event::KeyEvent,
    Command,
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
pub struct TextInput {
    pub input_buffer: InputBuffer,
    pub is_rounded: bool,
    focused: bool,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl TextInput {
    pub fn new<S>(value: S) -> Self
    where
        S: ToString,
    {
        TextInput {
            input_buffer: InputBuffer::new_with_value(value),
            is_rounded: false,
            ..Default::default()
        }
    }

    pub fn process_key(&mut self, key_event: KeyEvent) {
        self.input_buffer.process_key_event(key_event);
    }

    pub fn set_value<S: ToString>(&mut self, value: S) {
        self.input_buffer.set_value(value);
    }

    pub fn get_value(&self) -> &str {
        self.input_buffer.get_content()
    }

    pub fn set_rounded(&mut self, rounded: bool) {
        self.is_rounded = rounded;
    }
}

impl Widget for TextInput {
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
    fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let width = layout.size.width.round() as usize;
        let height = layout.size.height.round() as usize;

        let mut top_left = if self.is_rounded {
            rounded::TOP_LEFT
        } else {
            line::TOP_LEFT
        };

        let mut top_right = if self.is_rounded {
            rounded::TOP_RIGHT
        } else {
            line::TOP_RIGHT
        };
        let mut bottom_left = if self.is_rounded {
            rounded::BOTTOM_LEFT
        } else {
            line::BOTTOM_LEFT
        };
        let mut bottom_right = if self.is_rounded {
            rounded::BOTTOM_RIGHT
        } else {
            line::BOTTOM_RIGHT
        };

        let mut horizontal = line::HORIZONTAL;
        let mut vertical = line::VERTICAL;

        // Note: the rounded border is override with square thick line since there is no thick
        // rounded corner
        if self.focused {
            top_left = thick_line::TOP_LEFT;
            top_right = thick_line::TOP_RIGHT;
            bottom_left = thick_line::BOTTOM_LEFT;
            bottom_right = thick_line::BOTTOM_RIGHT;
            horizontal = thick_line::HORIZONTAL;
            vertical = thick_line::VERTICAL;
        }

        for i in 0..width {
            buf.set_symbol(loc_x + i, loc_y + 1, horizontal);
            buf.set_symbol(loc_x + i, loc_y + height, horizontal);
        }
        for j in 0..height {
            buf.set_symbol(loc_x, loc_y + 1 + j, vertical);
            buf.set_symbol(loc_x + width, loc_y + 1 + j, vertical);
        }
        let text_loc_y = loc_y + 2;
        for (t, ch) in self.get_value().chars().enumerate() {
            buf.set_symbol(loc_x + 1 + t, text_loc_y, ch);
        }

        buf.set_symbol(loc_x, loc_y + 1, top_left);
        buf.set_symbol(loc_x, loc_y + height, bottom_left);
        buf.set_symbol(loc_x + width, loc_y + 1, top_right);
        buf.set_symbol(loc_x + width, loc_y + height, bottom_right);
        let cursor_loc_x = loc_x + self.input_buffer.get_cursor_location();
        if self.focused {
            vec![Cmd::MoveTo(cursor_loc_x + 3, loc_y + 1)]
        } else {
            vec![]
        }
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn set_size(&mut self, width: Option<f32>, height: Option<f32>) {
        self.width = width;
        self.height = height;
    }
}
