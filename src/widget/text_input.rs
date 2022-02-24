use crate::Callback;
use crate::Event;
use crate::{buffer::Buffer, Cmd, InputBuffer, Widget};
use crossterm::event::{KeyEvent, MouseEvent};
use expanse::{
    geometry::Size,
    result::Layout,
    style::{Dimension, PositionType, Style},
};
use ito_canvas::unicode_canvas::{Border, Canvas};
use std::any::Any;
use std::fmt;

/// A one line text input
#[derive(Default, Debug)]
pub struct TextInput<MSG> {
    layout: Option<Layout>,
    input_buffer: InputBuffer,
    is_rounded: bool,
    has_border: bool,
    focused: bool,
    width: Option<f32>,
    height: Option<f32>,
    id: Option<String>,
    on_input: Vec<Callback<Event, MSG>>,
}

impl<MSG> TextInput<MSG> {
    /// creates a new text input with initial value
    pub fn new<S>(value: S) -> Self
    where
        S: ToString,
    {
        TextInput {
            layout: None,
            input_buffer: InputBuffer::new_with_value(value),
            is_rounded: false,
            has_border: true,
            focused: false,
            width: None,
            height: None,
            id: None,
            on_input: vec![],
        }
    }

    /// process the key event for this text input
    pub fn process_key(&mut self, key_event: KeyEvent) -> Vec<MSG> {
        self.input_buffer.process_key_event(key_event);
        self.on_input
            .iter_mut()
            .map(|cb| cb.emit(key_event.into()))
            .collect()
    }

    /// set the value of the buffer
    pub fn set_value<S: ToString>(&mut self, value: S) {
        self.input_buffer = InputBuffer::new_with_value(value);
    }

    /// returns a reference to the text value of this text input widget
    pub fn get_value(&self) -> &str {
        self.input_buffer.get_content()
    }

    /// set whether to use rounded corner when drawing the border of the text input
    pub fn set_rounded(&mut self, rounded: bool) {
        self.is_rounded = rounded;
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

    #[allow(dead_code)]
    fn inner_height(&self, layout: &Layout) -> usize {
        let ih = layout.size.height.round()
            - self.border_top()
            - self.border_bottom();
        if ih > 0.0 {
            ih as usize
        } else {
            0
        }
    }

    fn inner_width(&self, layout: &Layout) -> usize {
        let iw = layout.size.width.round()
            - self.border_left()
            - self.border_right();
        if iw > 0.0 {
            iw as usize
        } else {
            0
        }
    }

    pub fn on_input<F>(&mut self, f: F)
    where
        F: FnMut(Event) -> MSG + 'static,
    {
        self.on_input.push(f.into());
    }
}

impl<MSG> Widget<MSG> for TextInput<MSG>
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
                    Dimension::Points(3.0)
                },
            },
            min_size: Size {
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
                    Dimension::Percent(1.0)
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
    fn draw(&self, buf: &mut Buffer) -> Vec<Cmd> {
        let layout = self.layout.expect("must have a layout");
        let loc_x = layout.location.x;
        let loc_y = layout.location.y;
        let width = layout.size.width;
        let height = layout.size.height;

        let left = loc_x;
        let top = loc_y;
        let bottom = top + height - 1.0;
        let right = left + width - 1.0;

        if self.has_border {
            let border = Border {
                use_thick_border: self.focused,
                has_top: true,
                has_bottom: true,
                has_left: true,
                has_right: true,
                is_top_left_rounded: self.is_rounded,
                is_top_right_rounded: self.is_rounded,
                is_bottom_left_rounded: self.is_rounded,
                is_bottom_right_rounded: self.is_rounded,
            };
            let mut canvas = Canvas::new();
            canvas.draw_rect(
                (left as usize, top as usize),
                (right as usize, bottom as usize),
                border,
            );
            buf.write_canvas(canvas);
        }

        let inner_width = self.inner_width(&layout);
        for (t, ch) in self.get_value().chars().enumerate() {
            buf.set_symbol(
                (left + self.border_left() + t as f32) as usize,
                (top + self.border_top()) as usize,
                ch,
            );
        }

        let cursor_loc_x = self.input_buffer.get_cursor_location() as f32;
        if self.focused {
            vec![
                Cmd::ShowCursor,
                Cmd::MoveTo(
                    (left + cursor_loc_x + self.border_left()) as usize,
                    (top + self.border_top()) as usize,
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

    fn process_event(&mut self, event: Event) -> Vec<MSG> {
        let layout = self.layout.expect("must have a layout set");
        match event {
            Event::Key(ke) => {
                self.process_key(ke);
                vec![]
            }
            Event::Mouse(MouseEvent::Down(_btn, x, _y, _modifier)) => {
                let cursor_loc = x as i32 - layout.location.x.round() as i32;
                self.input_buffer.set_cursor_loc(cursor_loc as usize);
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
