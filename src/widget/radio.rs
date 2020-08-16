use crate::Event;
use crate::{buffer::Buffer, symbol, Callback, Cmd, Widget};
use crossterm::event::MouseEvent;
use std::{any::Any, fmt};
use stretch::{
    geometry::Size,
    result::Layout,
    style::{Dimension, PositionType, Style},
};

/// Radio button widget
#[derive(Default, PartialEq)]
pub struct Radio {
    layout: Option<Layout>,
    label: String,
    is_checked: bool,
    id: Option<String>,
    on_input: Vec<Callback<Event>>,
}

impl Radio {
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
}

impl Widget for Radio {
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

    /// draw this button to the buffer, with the given computed layout
    fn draw(&self, buf: &mut Buffer) -> Vec<Cmd> {
        let layout = self.layout.expect("must have a layout");
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let (box_symbol, x_offset) = if self.is_checked {
            (symbol::RADIO_CHECKED, 0)
        } else {
            (symbol::RADIO_UNCHECKED, 0)
        };
        buf.set_symbol(loc_x, loc_y, box_symbol);

        for (t, ch) in self.label.chars().enumerate() {
            buf.set_symbol(loc_x + 3 + x_offset + t, loc_y, ch);
        }
        vec![]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn set_size(&mut self, _width: Option<f32>, _height: Option<f32>) {}

    fn process_event(&mut self, event: Event) {
        match event {
            Event::Mouse(MouseEvent::Down(_btn, _x, _y, _modifier)) => {
                self.is_checked = !self.is_checked;
            }
            _ => (),
        }
    }

    fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }

    fn get_id(&self) -> &Option<String> {
        &self.id
    }
}

impl fmt::Debug for Radio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Radio")
            .field("label", &self.label)
            .field("id", &self.id)
            .finish()
    }
}
