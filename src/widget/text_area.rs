use crate::crossterm::event::KeyEvent;
use crate::crossterm::event::KeyModifiers;
use crate::Event;
use crate::Value;
use crate::{
    buffer::Buffer, event::InputEvent, symbol, symbol::bar,
    text_buffer::AreaBuffer, Callback, Cmd, Widget,
};
use expanse::{
    geometry::Size,
    result::Layout,
    style::{Dimension, PositionType, Style},
};
use glam::*;
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

    fn unwrap_layout(&self) -> &Layout {
        self.layout.as_ref().expect("must have a layout")
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

    /// layout height excluding the borders
    fn inner_height(&self) -> f32 {
        self.layout_height() - self.border_top() - self.border_bottom()
    }

    /// layout width excluding the borders
    fn inner_width(&self) -> f32 {
        self.layout_width() - self.border_left() - self.border_right()
    }

    fn content_height(&self) -> f32 {
        self.area_buffer.height() as f32
    }

    fn content_width(&self) -> f32 {
        self.area_buffer.width() as f32
    }

    /// scroll height is the ratio of layout height to content height
    /// if content height is the same as the layout height (perfect fit)
    /// meaning scroll height is 1.0 * inner_height
    fn scroller_height(&self) -> f32 {
        let content_height = self.content_height();
        let inner_height = self.inner_height();
        let ratio_layout_content = inner_height / content_height;
        inner_height * ratio_layout_content
    }

    /// scroll_offset_y is the ratio of
    /// scroll_top to the content_height
    /// scroll_top / content_height * inner_height
    fn scroller_offset_y(&self) -> f32 {
        let inner_height = self.inner_height();
        self.scroll_top / self.content_height() * inner_height
    }

    fn scroller_width(&self) -> f32 {
        let content_width = self.content_width();
        let inner_width = self.inner_width();
        let ratio_layout_content = inner_width / content_width;
        inner_width * ratio_layout_content
    }

    fn scroller_offset_x(&self) -> f32 {
        let inner_width = self.inner_width();
        self.scroll_left / self.content_width() * inner_width
    }

    fn can_scroll_up(&self) -> bool {
        self.scroll_top > 0.0
    }

    fn can_scroll_left(&self) -> bool {
        self.scroll_left > 0.0
    }

    fn can_scroll_down(&self) -> bool {
        self.content_height() - self.scroll_top > self.inner_height()
    }
    fn can_scroll_right(&self) -> bool {
        self.content_width() - self.scroll_left > self.inner_width()
    }

    /// return the cursor location relative to the screen
    fn cursor_location(&self) -> Vec2 {
        let cursor_loc = self.area_buffer.get_cursor_location();
        let top_left = vec2(self.inner_left(), self.inner_top());
        let scroll_loc = vec2(self.scroll_left, self.scroll_top);
        top_left + cursor_loc - scroll_loc
    }

    fn top_left(&self) -> Vec2 {
        let layout = self.unwrap_layout();
        vec2(layout.location.x, layout.location.y)
    }

    fn top(&self) -> f32 {
        self.top_left().y
    }

    fn left(&self) -> f32 {
        self.top_left().x
    }

    fn layout_width(&self) -> f32 {
        let layout = self.unwrap_layout();
        layout.size.width
    }

    fn layout_height(&self) -> f32 {
        let layout = self.unwrap_layout();
        layout.size.height
    }

    fn bottom(&self) -> f32 {
        self.top() + self.layout_height() - 1.0
    }

    fn right(&self) -> f32 {
        self.left() + self.layout_width() - 1.0
    }

    /// inner bottom location excluding the border
    fn inner_bottom(&self) -> f32 {
        self.top() + self.layout_height() - self.border_bottom()
    }
    /// the inner right location of the textarea excluding the border
    fn inner_right(&self) -> f32 {
        self.left() + self.layout_width() - self.border_right()
    }
    /// the inner top location of the textarea excluding the border
    fn inner_top(&self) -> f32 {
        self.top() + self.border_top()
    }

    /// the inner left location of the textare excluding the border
    fn inner_left(&self) -> f32 {
        self.left() + self.border_left()
    }

    fn is_cursor_visible(&self) -> bool {
        let abs_cursor = self.cursor_location();
        (abs_cursor.y >= self.inner_top()
            && abs_cursor.y <= self.inner_bottom())
            && (abs_cursor.x >= self.inner_left()
                && abs_cursor.x <= self.inner_right())
    }

    fn draw_scrollers(&self, buf: &mut Buffer) {
        let bottom = self.inner_bottom();
        let right = self.inner_right();
        let top = self.inner_top();
        let left = self.inner_left();

        let scroller_width = self.scroller_width();
        let scroller_height = self.scroller_height();

        let inner_width = self.inner_width();
        let inner_height = self.inner_height();

        let content_width = self.content_width();
        let content_height = self.content_height();

        let scroller_offset_y = self.scroller_offset_y();
        let scroller_offset_x = self.scroller_offset_x();

        if inner_height > 0.0 {
            for j in 0..scroller_height as usize {
                buf.set_symbol(
                    right as usize,
                    (top + j as f32 + scroller_offset_y) as usize,
                    bar::SEVEN_EIGHTHS,
                );
            }
        }

        if inner_width > 0.0 {
            for i in 0..scroller_width as usize {
                buf.set_symbol(
                    (left + i as f32 + scroller_offset_x) as usize,
                    bottom as usize,
                    symbol::MIDDLE_BLOCK,
                );
            }
        }
    }

    fn draw_border(&self, buf: &mut Buffer) {
        let left = self.left();
        let top = self.top();
        let bottom = self.bottom();
        let right = self.right();

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
        canvas.draw_rect(
            (left as usize, top as usize),
            (right as usize, bottom as usize),
            border,
        );
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
        // draw the text content
        let text_loc_y = self.inner_top() - self.scroll_top;
        let text_loc_x = self.inner_left() - self.scroll_left;
        let bottom_scroll = self.inner_height() + self.scroll_top;
        let right_scroll = self.inner_width() + self.scroll_left;

        for (j, line) in self.area_buffer.content.iter().enumerate() {
            if (j as f32) >= self.scroll_top && (j as f32) < bottom_scroll {
                for (i, ch) in line.iter().enumerate() {
                    if (i as f32) >= self.scroll_left
                        && (i as f32) < right_scroll
                    {
                        buf.set_symbol(
                            (text_loc_x + i as f32) as usize,
                            (text_loc_y + j as f32) as usize,
                            ch,
                        );
                    }
                }
            }
        }

        self.draw_border(buf);
        self.draw_scrollers(buf);

        if self.focused && self.is_cursor_visible() {
            let abs_cursor = self.cursor_location();
            vec![
                Cmd::ShowCursor,
                Cmd::MoveTo(abs_cursor.x as usize, abs_cursor.y as usize),
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
            Event::Mouse(_me) => {
                let (x, y) = event
                    .extract_location()
                    .expect("must have a mouse location");
                let modifiers = event.modifiers();
                let scroll_speed_x = 2.0;
                let scroll_speed_y = 2.0;

                if event.is_mouse_click() {
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
                    let cursor_x = x + self.scroll_left;

                    self.area_buffer
                        .set_cursor_loc(cursor_x as usize, cursor_y as usize);
                    vec![]
                } else if event.is_scroll() {
                    let is_shift_key_pressed =
                        modifiers.unwrap().contains(KeyModifiers::SHIFT);

                    if event.is_scrollup() {
                        if is_shift_key_pressed {
                            if self.can_scroll_left() {
                                self.scroll_left -= scroll_speed_x;
                            }
                        } else {
                            if self.can_scroll_up() {
                                self.scroll_top -= scroll_speed_y;
                            }
                        }
                    }
                    if event.is_scrolldown() {
                        if is_shift_key_pressed {
                            if self.can_scroll_right() {
                                self.scroll_left += scroll_speed_x;
                            }
                        } else {
                            if self.can_scroll_down() {
                                self.scroll_top += scroll_speed_y;
                            }
                        }
                    }
                    vec![]
                } else {
                    vec![]
                }
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
