use crate::crossterm::event::KeyEvent;
use crate::symbol;
use crate::text_buffer::InputBuffer;
use crate::Callback;
use crate::Event;
use crate::{
    buffer::{Buffer, Cell},
    Cmd, Widget,
};
use expanse::{
    geometry::{Rect, Size},
    result::Layout,
    style::{Dimension, PositionType, Style},
};
use ito_canvas::unicode_canvas::{Border, Canvas};

use std::{fmt, fmt::Debug};

/// A button widget
#[derive(PartialEq, Clone)]
pub struct SearchBox<MSG> {
    layout: Option<Layout>,
    input_buffer: InputBuffer,
    is_rounded: bool,
    width: Option<f32>,
    height: Option<f32>,
    focused: bool,
    id: Option<String>,
    on_input: Vec<Callback<Event, MSG>>,
}

impl<MSG> Default for SearchBox<MSG> {
    fn default() -> Self {
        SearchBox {
            layout: None,
            input_buffer: InputBuffer::new_with_value(""),
            is_rounded: false,
            width: None,
            height: None,
            focused: false,
            on_input: vec![],
            id: None,
        }
    }
}

impl<MSG> Debug for SearchBox<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SearchBox").field("id", &self.id).finish()
    }
}

impl<MSG> SearchBox<MSG> {
    /// create a new button with label
    pub fn new<S>(label: S) -> Self
    where
        S: ToString,
    {
        SearchBox {
            is_rounded: true,
            ..Default::default()
        }
    }

    /// set the label of the button
    pub fn set_search_term<S: ToString>(&mut self, text: S) {
        self.input_buffer.set_content(text);
    }

    pub fn get_search_term(&self) -> &str {
        self.input_buffer.get_content()
    }

    /// set to use a rounded border
    pub fn set_rounded(&mut self, rounded: bool) {
        self.is_rounded = rounded;
    }

    /// process the key event for this text input
    pub fn process_key(&mut self, key_event: KeyEvent) -> Vec<MSG> {
        self.input_buffer.process_key_event(key_event);
        self.on_input
            .iter_mut()
            .map(|cb| cb.emit(key_event.into()))
            .collect()
    }

    pub fn on_input<F>(&mut self, f: F)
    where
        F: FnMut(Event) -> MSG + 'static,
    {
        self.on_input.push(f.into());
    }
}

impl<MSG> Widget<MSG> for SearchBox<MSG> {
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
                    Dimension::Auto
                },
            },
            min_size: Size {
                height: Dimension::Points(3.0),
                ..Default::default()
            },
            border: Rect {
                top: Dimension::Points(self.border_top()),
                bottom: Dimension::Points(self.border_bottom()),
                start: Dimension::Points(self.border_left()),
                end: Dimension::Points(self.border_right()),
            },
            ..Default::default()
        }
    }

    fn border_style(&self) -> Border {
        Border {
            use_thick_border: false,
            has_top: true,
            has_bottom: true,
            has_left: true,
            has_right: true,
            is_top_left_rounded: self.is_rounded,
            is_top_right_rounded: self.is_rounded,
            is_bottom_left_rounded: self.is_rounded,
            is_bottom_right_rounded: self.is_rounded,
        }
    }

    fn has_border(&self) -> bool {
        true
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

        let text_start = left + 4.0;

        self.draw_border(buf);

        for (t, ch) in self.get_search_term().chars().enumerate() {
            let mut cell = Cell::new(ch);
            if self.focused {
                cell.bold();
            }
            buf.set_cell(text_start as usize + t, (loc_y + 1.0) as usize, cell);
        }

        let cursor_loc_x = self.input_buffer.get_cursor_location() as f32;
        if self.focused {
            vec![
                Cmd::ShowCursor,
                Cmd::MoveTo(
                    (text_start + cursor_loc_x) as usize,
                    (top + self.border_top()) as usize,
                ),
            ]
        } else {
            vec![]
        }
    }

    fn draw_border(&self, buf: &mut Buffer) {
        let left = self.left();
        let top = self.top();
        let bottom = self.bottom();
        let right = self.right();

        let border = self.border_style();
        let mut canvas = Canvas::new();
        canvas.draw_rect(
            (left as usize, top as usize),
            (right as usize, bottom as usize),
            border,
        );

        // draw the separator and search icon button
        canvas.draw_vertical_line(
            ((left + 3.0) as usize, top as usize),
            ((left + 3.0) as usize, bottom as usize),
            false,
        );
        buf.set_cell(
            (left + 1.0) as usize,
            (top + 1.0) as usize,
            Cell::new(symbol::SEARCH_ICON),
        );
        buf.write_canvas(canvas);
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
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
            Event::Mouse(_me) => {
                let (x, _y) =
                    event.extract_location().expect("must have a location");
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
