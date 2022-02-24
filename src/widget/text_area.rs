use crate::Event;
use crate::Value;
use crate::{
    area_buffer::AreaBuffer, buffer::Buffer, event::InputEvent, symbol,
    symbol::bar, Callback, Cmd, Widget,
};
use crossterm::event::{KeyEvent, KeyModifiers, MouseEvent};
use expanse::{
    geometry::Size,
    result::Layout,
    style::{Dimension, PositionType, Style},
};
use ito_canvas::unicode_canvas::{Border, Canvas};
use std::fmt;

/// A textarea is a 2 dimensional editor
/// where each line is separated by \n.
#[derive(Debug)]
pub struct TextArea<MSG> {
    layout: Option<Layout>,
    area_buffer: AreaBuffer,
    focused: bool,
    width: Option<f32>,
    height: Option<f32>,
    scroll_top: f32,
    scroll_left: f32,
    id: Option<String>,
    on_input: Vec<Callback<Event, MSG>>,
    has_border: bool,
    is_rounded_border: bool,
    is_thick_border: bool,
}

impl<MSG> TextArea<MSG> {
    /// create a new text area with initial value
    pub fn new<S>(value: S) -> Self
    where
        S: ToString,
    {
        TextArea {
            layout: None,
            area_buffer: AreaBuffer::from(value.to_string()),
            width: None,
            height: None,
            focused: false,
            scroll_top: 0.0,
            scroll_left: 0.0,
            id: None,
            on_input: vec![],
            has_border: true,
            is_rounded_border: false,
            is_thick_border: false,
        }
    }

    /// attach an listener to the input event of this textarea
    pub fn add_input_listener(&mut self, cb: Callback<Event, MSG>) {
        self.on_input.push(cb);
    }

    /// get the content of this text area, same as get_value
    pub fn get_content(&self) -> String {
        self.area_buffer.to_string()
    }

    /// process the keypress event
    pub fn process_key(&mut self, key_event: KeyEvent) {
        self.area_buffer.process_key_event(key_event);
    }

    /// set the value of this text area
    pub fn set_value<S: ToString>(&mut self, value: S) {
        self.area_buffer = AreaBuffer::from(value.to_string());
    }

    /// add a line to the last end of buffer of this text area
    pub fn add_line<S: ToString>(&mut self, s: S) {
        self.area_buffer.add_line(s);
    }

    /// return the string value of this text_area
    pub fn get_value(&self) -> String {
        self.area_buffer.to_string()
    }

    fn border_top(&self) -> f32 {
        if self.has_border {
            1.0
        } else {
            0.0
        }
    }

    fn border_bottom(&self) -> f32 {
        if self.has_border {
            1.0
        } else {
            0.0
        }
    }

    fn border_left(&self) -> f32 {
        if self.has_border {
            1.0
        } else {
            0.0
        }
    }

    fn border_right(&self) -> f32 {
        if self.has_border {
            1.0
        } else {
            0.0
        }
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

    fn content_height(&self) -> f32 {
        self.area_buffer.height() as f32
    }

    fn content_width(&self) -> f32 {
        self.area_buffer.width() as f32
    }

    fn scroller_height(&self, _layout: &Layout) -> f32 {
        1.0
    }

    fn scroller_width(&self, _layout: &Layout) -> f32 {
        3.0
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

    fn draw_scrollers(&self, buf: &mut Buffer) {
        let layout = self.layout.expect("must have a layout");
        let loc_x = layout.location.x.round();
        let loc_y = layout.location.y.round();
        let width = layout.size.width.round();
        let height = layout.size.height.round();

        let bottom = loc_y + height - 1.0;
        let right = loc_x + width - 1.0;

        let scroller_width = self.scroller_width(&layout) as usize;
        let scroller_height = self.scroller_height(&layout) as usize;

        let inner_width = self.inner_width(&layout);
        let inner_height = self.inner_height(&layout);

        if inner_height > 0.0 {
            for j in 0..scroller_height {
                buf.set_symbol(
                    right as usize,
                    bottom as usize - j - 1,
                    bar::SEVEN_EIGHTHS,
                );
            }
        }

        if inner_width > 0.0 {
            for i in 0..scroller_width {
                buf.set_symbol(
                    right as usize - i - 1,
                    bottom as usize,
                    symbol::MIDDLE_BLOCK,
                );
            }
        }
    }

    fn draw_border(&self, buf: &mut Buffer) {
        let layout = self.layout.expect("must have a layout");
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let width = layout.size.width.round() as usize;
        let height = layout.size.height.round() as usize;

        let left = loc_x;
        let top = loc_y;
        let bottom = top + height - 1;
        let right = left + width - 1;

        let border = Border {
            use_thick_border: self.focused,
            has_top: true,
            has_bottom: true,
            has_left: true,
            has_right: true,
            is_top_left_rounded: false,
            is_top_right_rounded: false,
            is_bottom_left_rounded: false,
            is_bottom_right_rounded: false,
        };
        let mut canvas = Canvas::new();
        canvas.draw_rect((left, top), (right, bottom), border);
        buf.write_canvas(canvas);
    }
}

impl<MSG> Widget<MSG> for TextArea<MSG>
where
    MSG: fmt::Debug + 'static,
{
    fn layout(&self) -> Option<&Layout> {
        self.layout.as_ref()
    }
    fn set_layout(&mut self, layout: Layout) {
        self.layout = Some(layout);
    }
    fn style(&self) -> Style {
        Style {
            position_type: PositionType::Relative,
            size: Size {
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
                    Dimension::Percent(1.0)
                },
                height: if let Some(height) = self.height {
                    Dimension::Points(height)
                } else {
                    Dimension::Percent(1.0)
                },
            },
            ..Default::default()
        }
    }

    /// draw this button to the buffer, with the given computed layout
    fn draw(&self, buf: &mut Buffer) -> Vec<Cmd> {
        let layout = self.layout.expect("must have a layout");
        let loc_x = layout.location.x.round();
        let loc_y = layout.location.y.round();
        let height = layout.size.height.round();

        let bottom = loc_y + height - 1.0;

        // draw the text content
        let text_loc_y = loc_y - self.scroll_top;
        let text_loc_x = loc_x - self.scroll_left;
        let bottom_scroll = self.inner_height(&layout) + self.scroll_top;
        let right_scroll = self.inner_width(&layout) + self.scroll_left;

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

        let (abs_cursor_x, abs_cursor_y) = self.cursor_location(&layout);

        let is_cursor_visible = abs_cursor_y > loc_y && abs_cursor_y < bottom;

        self.draw_border(buf);
        self.draw_scrollers(buf);

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

    fn set_size(&mut self, width: Option<f32>, height: Option<f32>) {
        self.width = width;
        self.height = height;
    }

    fn process_event(&mut self, event: Event) -> Vec<MSG> {
        let layout = self.layout.expect("must have a layout");
        match event {
            Event::Key(ke) => {
                self.process_key(ke);
                let s_event: Event = Event::from(InputEvent::from(
                    Value::from(self.get_content()),
                ));
                self.on_input
                    .iter_mut()
                    .map(|cb| cb.emit(s_event.clone()))
                    .collect()
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
                        self.scroll_left -= 4.0;
                    }
                } else {
                    if self.scroll_top > 0.0 {
                        self.scroll_top -= 4.0;
                    }
                }
                vec![]
            }
            Event::Mouse(MouseEvent::ScrollDown(_x, _y, modifier)) => {
                if modifier.contains(KeyModifiers::SHIFT) {
                    self.scroll_left += 4.0;
                } else {
                    if self.content_height() - self.scroll_top
                        > self.inner_height(&layout)
                    {
                        self.scroll_top += 4.0;
                    }
                }
                vec![]
            }
            _ => vec![],
        }
    }

    fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }

    fn get_id(&self) -> &Option<String> {
        &self.id
    }
}
