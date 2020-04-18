use crate::{
    buffer::Buffer,
    symbol::{bar, line, rounded, thick_line},
    AreaBuffer, Cmd, LayoutTree, Widget,
};
use crossterm::event::{Event, KeyEvent, KeyModifiers, MouseEvent};
use std::{any::Any, fmt, marker::PhantomData};
use stretch::{
    geometry::Size,
    result::Layout,
    style::{Dimension, Style},
};

//TODO: make the widget scroll to the cursor location when cursor is not visible
#[derive(Debug, PartialEq)]
pub struct TextArea<MSG> {
    pub area_buffer: AreaBuffer,
    pub is_rounded: bool,
    focused: bool,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub scroll_top: f32,
    pub scroll_left: f32,
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
            scroll_top: 0.0,
            scroll_left: 0.0,
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

    fn inner_height(&self, layout: &Layout) -> f32 {
        let ih = layout.size.height.round()
            - self.border_top()
            - self.border_bottom();
        if ih > 0.0 {
            ih
        } else {
            0.0
        }
    }

    fn inner_width(&self, layout: &Layout) -> f32 {
        let iw = layout.size.width.round()
            - self.border_left()
            - self.border_right();
        if iw > 0.0 {
            iw
        } else {
            0.0
        }
    }

    fn get_symbols(&self) -> (&str, &str, &str, &str, &str, &str) {
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

        (
            top_left_symbol,
            top_right_symbol,
            bottom_left_symbol,
            bottom_right_symbol,
            horizontal_symbol,
            vertical_symbol,
        )
    }

    fn content_height(&self) -> f32 {
        self.area_buffer.height() as f32
    }

    fn content_width(&self) -> f32 {
        self.area_buffer.width() as f32
    }

    fn scroller_height(&self, layout: &Layout) -> f32 {
        let content_height =
            self.content_height() + self.border_top() + self.border_bottom();

        let height = layout.size.height.round();
        let scroller_height =
            (height as f32 * height as f32 / content_height).round();
        scroller_height
    }

    fn scroller_width(&self, layout: &Layout) -> f32 {
        let content_width =
            self.content_width() + self.border_left() + self.border_right();

        let width = layout.size.width.round();
        let scroller_width =
            (width as f32 * width as f32 / content_width).round();
        scroller_width
    }

    fn cursor_location(&self, layout: &Layout) -> (f32, f32) {
        let (cursor_loc_x, cursor_loc_y) =
            self.area_buffer.get_cursor_location();

        let loc_x = layout.location.x.round();
        let loc_y = layout.location.y.round();

        let abs_cursor_x = loc_x + cursor_loc_x as f32 + 1.0;
        let abs_cursor_y = loc_y + cursor_loc_y as f32 + 1.0 - self.scroll_top;
        (abs_cursor_x, abs_cursor_y)
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
                    Dimension::Points(self.content_height())
                },
            },
            ..Default::default()
        }
    }

    /// draw this button to the buffer, with the given computed layout
    fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round();
        let loc_y = layout.location.y.round();
        let width = layout.size.width.round();
        let height = layout.size.height.round();

        let (
            top_left_symbol,
            top_right_symbol,
            bottom_left_symbol,
            bottom_right_symbol,
            horizontal_symbol,
            vertical_symbol,
        ) = self.get_symbols();

        let bottom = loc_y + height - 1.0;
        let right = loc_x + width - 1.0;

        for i in 0..width as usize {
            buf.set_symbol(
                loc_x as usize + i,
                loc_y as usize,
                horizontal_symbol,
            );
            buf.set_symbol(
                loc_x as usize + i,
                bottom as usize,
                horizontal_symbol,
            );
        }
        for j in 0..height as usize {
            buf.set_symbol(loc_x as usize, loc_y as usize + j, vertical_symbol);
            buf.set_symbol(right as usize, loc_y as usize + j, vertical_symbol);
        }

        let scroller_height = self.scroller_height(&layout) as usize;
        let inner_height = self.inner_height(&layout_tree.layout);

        if inner_height > 0.0 {
            let scroller_top_loc = self.scroll_top / inner_height;
            for j in 0..scroller_height {
                buf.set_symbol(
                    right as usize,
                    loc_y as usize + scroller_top_loc as usize + j + 1,
                    bar::SEVEN_EIGHTHS,
                );
            }
        }

        let scroller_width = self.scroller_width(&layout) as usize;
        let inner_width = self.inner_width(&layout_tree.layout);

        if inner_width > 0.0 {
            let scroller_left_loc = self.scroll_left / inner_width;
            for i in 0..scroller_width {
                buf.set_symbol(
                    loc_x as usize + scroller_left_loc as usize + i + 1,
                    bottom as usize,
                    '▮',
                );
            }
        }

        // draw the text content
        let text_loc_y = loc_y - self.scroll_top;
        let text_loc_x = loc_x - self.scroll_left;
        let bottom_scroll = inner_height + self.scroll_top;
        let right_scroll = inner_width + self.scroll_left;

        for (j, line) in self.area_buffer.content.iter().enumerate() {
            if (j as f32) >= self.scroll_top && (j as f32) < bottom_scroll {
                for (i, ch) in line.iter().enumerate() {
                    if (i as f32) >= self.scroll_left
                        && (i as f32) < right_scroll
                    {
                        buf.set_symbol(
                            (text_loc_x + i as f32 + 1.0) as usize,
                            (text_loc_y + j as f32 + 1.0) as usize,
                            ch,
                        );
                    }
                }
            }
        }

        buf.set_symbol(loc_x as usize, loc_y as usize, top_left_symbol);
        buf.set_symbol(loc_x as usize, bottom as usize, bottom_left_symbol);
        buf.set_symbol(right as usize, loc_y as usize, top_right_symbol);
        buf.set_symbol(right as usize, bottom as usize, bottom_right_symbol);

        let (abs_cursor_x, abs_cursor_y) = self.cursor_location(&layout);

        let is_cursor_visible = abs_cursor_y > loc_y && abs_cursor_y < bottom;

        if self.focused && is_cursor_visible {
            vec![
                Cmd::ShowCursor,
                Cmd::MoveTo(abs_cursor_x as usize, abs_cursor_y as usize),
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
                let mut x = x as f32 - layout.location.x.round();
                let mut y = y as f32 - layout.location.y.round() - 1.0;

                if y < 0.0 {
                    y = 0.0;
                }
                let rows = self.content_height();
                if y >= rows {
                    y = rows - 1.0;
                }
                if x < 0.0 {
                    x = 0.0;
                }
                let cursor_y = y + self.scroll_top;
                if let Some(line) =
                    self.area_buffer.content.get(cursor_y as usize)
                {
                    if x > line.len() as f32 {
                        x = line.len() as f32;
                    }
                }
                let cursor_x = x;

                self.area_buffer
                    .set_cursor_loc(cursor_x as usize, cursor_y as usize);
                vec![]
            }
            Event::Mouse(MouseEvent::ScrollUp(_x, _y, modifier)) => {
                if modifier.contains(KeyModifiers::SHIFT) {
                    if self.scroll_left > 0.0 {
                        self.scroll_left -= 1.0;
                    }
                } else {
                    if self.scroll_top > 0.0 {
                        self.scroll_top -= 1.0;
                    }
                }
                vec![]
            }
            Event::Mouse(MouseEvent::ScrollDown(_x, _y, modifier)) => {
                if modifier.contains(KeyModifiers::SHIFT) {
                    self.scroll_left += 1.0;
                } else {
                    if self.content_height() - self.scroll_top
                        > self.inner_height(&layout)
                    {
                        self.scroll_top += 1.0;
                    }
                }
                vec![]
            }
            _ => vec![],
        }
    }
}
