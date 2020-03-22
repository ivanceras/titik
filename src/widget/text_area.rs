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
    AreaBuffer,
    Cmd,
    LayoutTree,
    Widget,
};
use crossterm::{
    cursor,
    event::{
        Event,
        KeyEvent,
        MouseEvent,
    },
    Command,
};
use std::any::Any;
use stretch::{
    geometry::Size,
    result::Layout,
    style::{
        Dimension,
        Style,
    },
};

#[derive(Default, Debug, PartialEq)]
pub struct TextArea {
    pub area_buffer: AreaBuffer,
    pub is_rounded: bool,
    focused: bool,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl TextArea {
    pub fn new<S>(value: S) -> Self
    where
        S: ToString,
    {
        TextArea {
            area_buffer: AreaBuffer::from(value.to_string()),
            is_rounded: false,
            ..Default::default()
        }
    }

    pub fn process_key(&mut self, key_event: KeyEvent) {
        self.area_buffer.process_key_event(key_event);
    }

    pub fn set_value<S: ToString>(&mut self, value: S) {
        self.area_buffer = AreaBuffer::from(value.to_string());
    }

    pub fn add_line<S: ToString>(&mut self, s: S) {
        self.area_buffer.add_line(s);
    }

    pub fn get_value(&self) -> String {
        self.area_buffer.to_string()
    }

    pub fn set_rounded(&mut self, rounded: bool) {
        self.is_rounded = rounded;
    }

}

impl<MSG> Widget<MSG> for TextArea {
    fn style(&self) -> Style {
        Style {
            size: Size {
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
                    //Dimension::Points((self.label.len() + 1) as f32)
                    //Dimension::Percent(0.95)
                    Dimension::Percent(1.0)
                },
                height: if let Some(height) = self.height {
                    Dimension::Points(height)
                } else {
                    Dimension::Points(
                        2.0 + self.area_buffer.content.len() as f32,
                    )
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
            buf.set_symbol(loc_x + i, loc_y, horizontal);
            buf.set_symbol(loc_x + i, loc_y + height - 1, horizontal);
        }
        for j in 0..height {
            buf.set_symbol(loc_x, loc_y + j, vertical);
            buf.set_symbol(loc_x + width - 1, loc_y + j, vertical);
        }
        let text_loc_y = loc_y + 1;
        for (j, line) in self.area_buffer.content.iter().enumerate() {
            for (i, ch) in line.iter().enumerate() {
                if loc_x + i < (width - 2) {
                    buf.set_symbol(loc_x + 1 + i, text_loc_y + j, ch);
                }
            }
        }

        buf.set_symbol(loc_x, loc_y, top_left);
        buf.set_symbol(loc_x, loc_y + height - 1, bottom_left);
        buf.set_symbol(loc_x + width - 1, loc_y, top_right);
        buf.set_symbol(loc_x + width - 1, loc_y + height - 1, bottom_right);
        let (cursor_loc_x, cursor_loc_y) =
            self.area_buffer.get_cursor_location();
        if self.focused {
            vec![
                Cmd::ShowCursor,
                Cmd::MoveTo(loc_x + cursor_loc_x + 1, loc_y + cursor_loc_y + 1),
            ]
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

    fn process_event(
        &mut self,
        event: Event,
        layout: &Layout,
    ) -> Vec<MSG> {
        match event {
            Event::Key(ke) => {
                self.process_key(ke);
                vec![]
            }
            Event::Mouse(MouseEvent::Down(_btn, x, y, modifier)) => {
                let cursor_loc_x =
                    x as i32 - layout.location.x.round() as i32;
                let mut cursor_loc_y =
                    y as i32 - layout.location.y.round() as i32;
                self.area_buffer
                    .set_cursor_loc_corrected(cursor_loc_x, cursor_loc_y - 1);
                //self.add_line(format!("cursor: {},{}", cursor_loc_x, cursor_loc_y));
                vec![]
            }
            _ => vec![],
        }
    }
}