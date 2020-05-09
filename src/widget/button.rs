use crate::{
    buffer::{Buffer, Cell},
    symbol::{line, rounded},
    Cmd, LayoutTree, Widget,
};
use crossterm::event::{Event, MouseEvent};
use sauron_vdom::Callback;
use std::{any::Any, fmt, fmt::Debug};
use stretch::{
    geometry::Size,
    result::Layout,
    style::{Dimension, Style},
};

#[derive(PartialEq, Clone)]
pub struct Button<MSG>
where
    MSG: 'static,
{
    pub label: String,
    pub is_rounded: bool,
    pub width: Option<f32>,
    pub height: Option<f32>,
    focused: bool,
    pub on_click: Vec<Callback<Event, MSG>>,
	pub id: Option<String>,
}

impl<MSG> Default for Button<MSG> {
    fn default() -> Self {
        Button {
            label: String::new(),
            is_rounded: false,
            width: None,
            height: None,
            focused: false,
            on_click: vec![],
			id: None,
        }
    }
}

impl<MSG> Debug for Button<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Button")
            .field("label", &self.label)
			.field("id", &self.id)
            .finish()
    }
}

impl<MSG> Button<MSG>
where
    MSG: 'static,
{
    pub fn new<S>(label: S) -> Self
    where
        S: ToString,
    {
        Button {
            label: label.to_string(),
            is_rounded: true,
            ..Default::default()
        }
    }

    pub fn add_click_listener<F>(&mut self, func: F)
    where
        F: Fn(Event) -> MSG + 'static,
    {
        self.on_click.push(Callback::from(func));
    }

    pub fn set_label<S: ToString>(&mut self, label: S) {
        self.label = label.to_string();
    }

    pub fn set_rounded(&mut self, rounded: bool) {
        self.is_rounded = rounded;
    }
}

impl<MSG> Widget<MSG> for Button<MSG>
where
    MSG: 'static,
{
    fn style(&self) -> Style {
        Style {
            size: Size {
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
                    //Dimension::Points((self.label.len() + 1) as f32)
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
    fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let width = layout.size.width.round() as usize;
        let height = layout.size.height.round() as usize;

        let bottom = loc_y as i32 + height as i32 - 1;
        let right = loc_x as i32 + width as i32 - 1;

        for i in 0..width {
            buf.set_symbol(loc_x + i, loc_y, line::HORIZONTAL);
            buf.set_symbol(loc_x + i, bottom as usize, line::HORIZONTAL);
        }
        for j in 0..height {
            buf.set_symbol(loc_x, loc_y + j, line::VERTICAL);
            buf.set_symbol(right as usize, loc_y + j, line::VERTICAL);
        }
        for (t, ch) in self.label.chars().enumerate() {
            let mut cell = Cell::new(ch);
            if self.focused {
                cell.bold();
            }
            buf.set_cell(loc_x + 1 + t, loc_y + 1, cell);
        }

        let top_left_symbol = if self.is_rounded {
            rounded::TOP_LEFT
        } else {
            line::TOP_LEFT
        };
        let top_right_symbol = if self.is_rounded {
            rounded::TOP_RIGHT
        } else {
            line::TOP_RIGHT
        };
        let bottom_left_symbol = if self.is_rounded {
            rounded::BOTTOM_LEFT
        } else {
            line::BOTTOM_LEFT
        };
        let bottom_right_symbol = if self.is_rounded {
            rounded::BOTTOM_RIGHT
        } else {
            line::BOTTOM_RIGHT
        };
        buf.set_symbol(loc_x, loc_y, top_left_symbol);
        buf.set_symbol(loc_x, bottom as usize, bottom_left_symbol);
        buf.set_symbol(right as usize, loc_y, top_right_symbol);
        buf.set_symbol(right as usize, bottom as usize, bottom_right_symbol);
        vec![]
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

    fn process_event(&mut self, event: Event, _layout: &Layout) -> Vec<MSG> {
        match event {
            Event::Mouse(MouseEvent::Down(..)) => {
                self.on_click.iter().map(|cb| cb.emit(event)).collect()
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
