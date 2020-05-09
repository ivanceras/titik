use crate::{buffer::Buffer, symbol, Cmd, LayoutTree, Widget};
use crossterm::event::{Event, MouseEvent};
use std::any::Any;
use stretch::{
    geometry::Size,
    result::Layout,
    style::{Dimension, Style},
};

#[derive(Default, Debug, PartialEq)]
pub struct Checkbox {
    pub label: String,
    pub is_checked: bool,
	pub id: Option<String>,
}

impl Checkbox {
    pub fn new<S>(label: S) -> Self
    where
        S: ToString,
    {
        Checkbox {
            label: label.to_string(),
            ..Default::default()
        }
    }

    pub fn set_label<S: ToString>(&mut self, label: S) {
        self.label = label.to_string();
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.is_checked = checked;
    }
}

impl<MSG> Widget<MSG> for Checkbox {
    fn style(&self) -> Style {
        Style {
            size: Size {
                width: Dimension::Points((self.label.len() + 3) as f32),
                height: Dimension::Points(1.0),
            },
            ..Default::default()
        }
    }

    /// draw this button to the buffer, with the given computed layout
    fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let box_symbol = if self.is_checked {
            symbol::BOX_CHECKED
        } else {
            symbol::BOX_UNCHECKED
        };
        buf.set_symbol(loc_x, loc_y, box_symbol);

        for (t, ch) in self.label.chars().enumerate() {
            buf.set_symbol(loc_x + 3 + t, loc_y, ch);
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

    fn process_event(&mut self, event: Event, _layout: &Layout) -> Vec<MSG> {
        match event {
            Event::Mouse(MouseEvent::Down(_btn, _x, _y, _modifier)) => {
                self.is_checked = !self.is_checked;
                vec![]
            }
            _ => vec![],
        }
    }
	fn set_id(&mut self, id: &str){
		self.id = Some(id.to_string());
	}

	fn get_id(&self) -> &Option<String> {
		&self.id
	}
}
