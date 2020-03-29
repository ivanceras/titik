use crate::{
    buffer::{
        Buffer,
    },
    symbol::{
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
    event::{
        Event,
        KeyEvent,
        MouseEvent,
    },
};
use std::{
    any::Any,
    fmt,
    marker::PhantomData,
};
use stretch::{
    geometry::Size,
    result::Layout,
    style::{
        Dimension,
        Style,
    },
};

#[derive(Debug, PartialEq)]
pub struct TextArea<MSG> {
    pub area_buffer: AreaBuffer,
    pub is_rounded: bool,
    focused: bool,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub scroll_top: usize,
    _phantom_msg: PhantomData<MSG>,
}

impl<MSG> TextArea<MSG> {
    pub fn new<S>(value: S) -> Self
    where
        S: ToString,
    {
        TextArea {
            area_buffer: AreaBuffer::from(value.to_string()),
            is_rounded: false,
            width: None,
            height: None,
            focused: false,
            scroll_top: 0,
            _phantom_msg: PhantomData,
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

    fn border_top(&self) -> f32 {
        1.0
    }

    fn border_bottom(&self) -> f32 {
        1.0
    }

    fn border_left(&self) -> f32 {
        1.0
    }
    fn border_right(&self) -> f32 {
        1.0
    }


    fn inner_height(&self, layout: &Layout) -> usize {
        let ih = layout.size.height.round() - self.border_top() - self.border_bottom();
        if ih > 0.0 {
            ih as usize
        }else{
            0
        }
    }

    fn inner_width(&self, layout: &Layout) -> usize {
        let iw = layout.size.width.round() - self.border_left() - self.border_right();
        if iw > 0.0 {
            iw as usize
        }else{
            0
        }
    }
}

impl<MSG> Widget<MSG> for TextArea<MSG>
where
    MSG: fmt::Debug + 'static,
{
    fn style(&self) -> Style {
        Style {
            size: Size {
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
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
        let inner_height = self.inner_height(&layout_tree.layout);

        let mut top_left_symbol = if self.is_rounded {
            rounded::TOP_LEFT
        } else {
            line::TOP_LEFT
        };

        let mut top_right_symbol = if self.is_rounded {
            rounded::TOP_RIGHT
        } else {
            line::TOP_RIGHT
        };
        let mut bottom_left_symbol = if self.is_rounded {
            rounded::BOTTOM_LEFT
        } else {
            line::BOTTOM_LEFT
        };
        let mut bottom_right_symbol = if self.is_rounded {
            rounded::BOTTOM_RIGHT
        } else {
            line::BOTTOM_RIGHT
        };

        let mut horizontal_symbol = line::HORIZONTAL;
        let mut vertical_symbol = line::VERTICAL;

        // Note: the rounded border is override with square thick line since there is no thick
        // rounded corner
        if self.focused {
            top_left_symbol = thick_line::TOP_LEFT;
            top_right_symbol = thick_line::TOP_RIGHT;
            bottom_left_symbol = thick_line::BOTTOM_LEFT;
            bottom_right_symbol = thick_line::BOTTOM_RIGHT;
            horizontal_symbol = thick_line::HORIZONTAL;
            vertical_symbol = thick_line::VERTICAL;
        }

        let bottom = loc_y + height - 1;
        let right = loc_x + width - 1;

        for i in 0..width {
            buf.set_symbol(loc_x + i, loc_y, horizontal_symbol);
            buf.set_symbol(loc_x + i, bottom, horizontal_symbol);
        }
        for j in 0..height {
            buf.set_symbol(loc_x, loc_y + j, vertical_symbol);
            buf.set_symbol(right, loc_y + j, vertical_symbol);
        }

        let scroller_height = height * height / self.area_buffer.content.len();
        if inner_height > 0 {
        let scroller_loc = self.scroll_top / inner_height;
            for j in 0..scroller_height {
                buf.set_symbol(
                    right,
                    loc_y + scroller_loc + j + 1,
                    '▇',
                );
            }
        }

        let inner_width = self.inner_width(&layout_tree.layout);

        // draw the text content
        let text_loc_y = loc_y + 1 - self.scroll_top;
        for (j, line) in self.area_buffer.content.iter().enumerate() {
            if j >= self.scroll_top && j < inner_height + self.scroll_top {
                for (i, ch) in line.iter().enumerate() {
                    if loc_x + i < inner_width {
                        buf.set_symbol(
                            loc_x + 1 + i,
                            text_loc_y + j,
                            ch,
                        );
                    }
                }
            }
        }

        buf.set_symbol(loc_x, loc_y, top_left_symbol);
        buf.set_symbol(loc_x, bottom, bottom_left_symbol);
        buf.set_symbol(right, loc_y, top_right_symbol);
        buf.set_symbol(right, bottom, bottom_right_symbol);
        let (cursor_loc_x, cursor_loc_y) =
            self.area_buffer.get_cursor_location();

        let abs_cursor_x = loc_x + cursor_loc_x + 1;
        let abs_cursor_y = loc_y + cursor_loc_y + 1 - self.scroll_top;

        let is_cursor_visible =
            abs_cursor_y > loc_y && abs_cursor_y < bottom;

        if self.focused && is_cursor_visible {
            vec![
                Cmd::ShowCursor,
                Cmd::MoveTo(
                    abs_cursor_x,
                    abs_cursor_y,
                ),
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

    fn process_event(&mut self, event: Event, layout: &Layout) -> Vec<MSG> {
        match event {
            Event::Key(ke) => {
                self.process_key(ke);
                vec![]
            }
            Event::Mouse(MouseEvent::Down(_btn, x, y, _modifier)) => {
                let mut x = x as i32 - layout.location.x.round() as i32;
                let mut y = y as i32 - layout.location.y.round() as i32 - 1;

                if y < 0 {
                    y = 0;
                }
                let rows = self.area_buffer.content.len() as i32;
                if y >= rows {
                    y = rows - 1;
                }
                if x < 0 {
                    x = 0;
                }
                let cursor_y = y as usize + self.scroll_top;
                if let Some(line) = self.area_buffer.content.get(cursor_y) {
                    if x > line.len() as i32 {
                        x = line.len() as i32;
                    }
                }
                let cursor_x = x as usize;

                self.area_buffer.set_cursor_loc(cursor_x, cursor_y);
                vec![]
            }
            Event::Mouse(MouseEvent::ScrollUp(_x, _y, _modifier)) => {
                if self.scroll_top > 0 {
                    self.scroll_top -= 1;
                }
                vec![]
            }
            Event::Mouse(MouseEvent::ScrollDown(_x, _y, _modifier)) => {
                // if the last content is aligned with the bottom
                let rows = self.area_buffer.content.len();
                let inner_height = self.inner_height(&layout);
                if rows - self.scroll_top > inner_height {
                    self.scroll_top += 1;
                }
                vec![]
            }
            _ => vec![],
        }
    }
}
