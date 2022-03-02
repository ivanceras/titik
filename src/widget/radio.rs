use crate::Event;
use crate::{buffer::Buffer, symbol, Callback, Cmd, Widget};
use expanse::{
    geometry::Size,
    result::Layout,
    style::{Dimension, PositionType, Style},
};
use std::fmt;

/// Radio button widget
#[derive(Default, PartialEq)]
pub struct Radio<MSG> {
    layout: Option<Layout>,
    label: String,
    is_checked: bool,
    id: Option<String>,
    on_input: Vec<Callback<Event, MSG>>,
}

impl<MSG> Radio<MSG> {
    /// create a new radio button with label
    pub fn new<S>(label: S) -> Self
    where
        S: ToString,
    {
        Radio {
            layout: None,
            label: label.to_string(),
            is_checked: false,
            id: None,
            on_input: vec![],
        }
    }

    /// set the label value
    pub fn set_label<S: ToString>(&mut self, label: S) {
        self.label = label.to_string();
    }

    /// set the checked status
    pub fn set_checked(&mut self, checked: bool) {
        self.is_checked = checked;
    }

    pub fn on_input<F>(&mut self, f: F)
    where
        F: FnMut(Event) -> MSG + 'static,
    {
        self.on_input.push(f.into());
    }
}

impl<MSG: 'static> Widget<MSG> for Radio<MSG> {
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
                width: Dimension::Points((self.label.len() + 3) as f32),
                height: Dimension::Points(1.0),
            },
            min_size: Size {
                height: Dimension::Points(1.0),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn has_border(&self) -> bool {
        false
    }

    /// draw this button to the buffer, with the given computed layout
    fn draw(&self, buf: &mut Buffer) -> Vec<Cmd> {
        let (box_symbol, x_offset) = if self.is_checked {
            (symbol::RADIO_CHECKED, 0)
        } else {
            (symbol::RADIO_UNCHECKED, 0)
        };
        buf.set_symbol(
            self.left() as usize,
            self.inner_top() as usize,
            box_symbol,
        );

        for (t, ch) in self.label.chars().enumerate() {
            buf.set_symbol(
                self.left() as usize + 3 + x_offset + t,
                self.inner_top() as usize,
                ch,
            );
        }
        vec![]
    }

    fn set_size(&mut self, _width: Option<f32>, _height: Option<f32>) {}

    fn process_event(&mut self, event: Event) -> Vec<MSG> {
        if event.is_mouse_click() {
            self.is_checked = !self.is_checked;
            vec![]
        } else {
            vec![]
        }
    }

    fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }

    fn get_id(&self) -> &Option<String> {
        &self.id
    }
}

impl<MSG> fmt::Debug for Radio<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Radio")
            .field("label", &self.label)
            .field("id", &self.id)
            .finish()
    }
}
