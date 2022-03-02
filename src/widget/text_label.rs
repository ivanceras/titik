use crate::Event;
use crate::{buffer::Buffer, Callback, Cmd, Widget};
use expanse::{
    geometry::Size,
    result::Layout,
    style::{Dimension, PositionType, Style},
};
use ito_canvas::unicode_canvas::{Border, Canvas};
use std::fmt;

/// A one line text input
#[derive(Debug)]
pub struct TextLabel<MSG> {
    layout: Option<Layout>,
    value: String,
    is_rounded: bool,
    has_border: bool,
    width: Option<f32>,
    height: Option<f32>,
    on_click: Vec<Callback<Event, MSG>>,
    id: Option<String>,
}

impl<MSG> TextLabel<MSG> {
    /// creates a new text input with initial value
    pub fn new<S>(value: S) -> Self
    where
        S: ToString,
    {
        TextLabel {
            layout: None,
            value: value.to_string(),
            is_rounded: false,
            has_border: false,
            width: None,
            height: None,
            on_click: vec![],
            id: None,
        }
    }

    /// set the value of the buffer
    pub fn set_value<S: ToString>(&mut self, value: S) {
        self.value = value.to_string();
    }

    /// returns a reference to the text value of this text input widget
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// set whether to use rounded corner when drawing the border of the text input
    pub fn set_rounded(&mut self, rounded: bool) {
        self.is_rounded = rounded;
    }
}

impl<MSG> Widget<MSG> for TextLabel<MSG>
where
    MSG: fmt::Debug,
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

    fn has_border(&self) -> bool {
        self.has_border
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
                use_thick_border: false,
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

        for (t, ch) in self.get_value().chars().enumerate() {
            buf.set_symbol(
                (left + self.border_left() + t as f32) as usize,
                (top + self.border_top()) as usize,
                ch,
            );
        }

        vec![]
    }

    fn set_size(&mut self, width: Option<f32>, height: Option<f32>) {
        self.width = width;
        self.height = height;
    }

    fn process_event(&mut self, _event: Event) -> Vec<MSG> {
        vec![]
    }

    fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }

    fn get_id(&self) -> &Option<String> {
        &self.id
    }
}
