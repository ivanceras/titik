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
    pub scroll: usize,
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
            scroll: 0,
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

        // scroller
        // TODO: calculate max scroll
        let scroller_height = height * height / self.area_buffer.content.len();
        let scroller_loc = self.scroll / height;
        for j in 0..scroller_height {
            buf.set_symbol(
                loc_x + width - 1,
                loc_y + scroller_loc + j + 1,
                '▇',
            );
        }

        // draw the text content
        let text_loc_y = loc_y + 1;
        for (j, line) in self.area_buffer.content.iter().enumerate() {
            if j >= self.scroll && j < height - 2 + self.scroll {
                for (i, ch) in line.iter().enumerate() {
                    if loc_x + i < (width - 2) {
                        buf.set_symbol(
                            loc_x + 1 + i,
                            text_loc_y - self.scroll + j,
                            ch,
                        );
                    }
                }
            }
        }

        buf.set_symbol(loc_x, loc_y, top_left);
        buf.set_symbol(loc_x, loc_y + height - 1, bottom_left);
        buf.set_symbol(loc_x + width - 1, loc_y, top_right);
        buf.set_symbol(loc_x + width - 1, loc_y + height - 1, bottom_right);
        let (cursor_loc_x, cursor_loc_y) =
            self.area_buffer.get_cursor_location();

        let abs_cursor_y = loc_y + cursor_loc_y + 1 - self.scroll;
        let is_cursor_visible =
            abs_cursor_y > loc_y && abs_cursor_y < loc_y + height - 1;
        if self.focused && is_cursor_visible {
            vec![
                Cmd::ShowCursor,
                Cmd::MoveTo(
                    loc_x + cursor_loc_x + 1,
                    loc_y + cursor_loc_y + 1 - self.scroll,
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
            Event::Mouse(MouseEvent::Down(_btn, mut x, mut y, modifier)) => {
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
                let cursor_y = y as usize + self.scroll;
                if let Some(line) = self.area_buffer.content.get(cursor_y) {
                    if x > line.len() as i32 {
                        x = line.len() as i32;
                    }
                }
                let cursor_x = x as usize;

                self.area_buffer.set_cursor_loc(cursor_x, cursor_y);
                vec![]
            }
            Event::Mouse(MouseEvent::ScrollUp(x, y, modifier)) => {
                if self.scroll > 0 {
                    self.scroll -= 1;
                }
                vec![]
            }
            Event::Mouse(MouseEvent::ScrollDown(x, y, modifier)) => {
                if self.scroll < self.area_buffer.content.len() {
                    self.scroll += 1;
                }
                vec![]
            }
            _ => vec![],
        }
    }
}
