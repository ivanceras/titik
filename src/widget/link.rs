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
pub struct Link<MSG> {
    layout: Option<Layout>,
    label: String,
    //TODO: show the uri when hovered
    uri: String,
    is_rounded: bool,
    has_border: bool,
    width: Option<f32>,
    height: Option<f32>,
    on_click: Vec<Callback<Event, MSG>>,
    id: Option<String>,
}

impl<MSG> Link<MSG> {
    /// creates a new text input with initial label
    pub fn new<S>(uri: S, label: S) -> Self
    where
        S: ToString,
    {
        Link {
            layout: None,
            label: label.to_string(),
            uri: uri.to_string(),
            is_rounded: false,
            has_border: false,
            width: None,
            height: None,
            on_click: vec![],
            id: None,
        }
    }

    /// set the label of this link
    pub fn set_label<S: ToString>(&mut self, label: S) {
        self.label = label.to_string();
    }

    /// set if there is an border in of this link
    pub fn set_border(&mut self, has_border: bool) {
        self.has_border = has_border;
    }

    /// set the uri of the link
    pub fn set_uri<S: ToString>(&mut self, uri: S) {
        self.uri = uri.to_string();
    }

    /// returns a reference to the text label of this text input widget
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// set whether to use rounded corner when drawing the border of the text input
    pub fn set_rounded(&mut self, rounded: bool) {
        self.is_rounded = rounded;
    }
}

impl<MSG> Widget<MSG> for Link<MSG>
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

    fn border_style(&self) -> Border {
        Border {
            use_thick_border: false,
            has_top: self.has_border,
            has_bottom: self.has_border,
            has_left: self.has_border,
            has_right: self.has_border,
            is_top_left_rounded: self.is_rounded,
            is_top_right_rounded: self.is_rounded,
            is_bottom_left_rounded: self.is_rounded,
            is_bottom_right_rounded: self.is_rounded,
        }
    }

    /// draw this button to the buffer, with the given computed layout
    fn draw(&self, buf: &mut Buffer) -> Vec<Cmd> {
        self.draw_border(buf);

        for (t, ch) in self.get_label().chars().enumerate() {
            buf.set_symbol(
                self.inner_left() as usize + t,
                self.inner_top() as usize,
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
