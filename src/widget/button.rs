use crate::{
    buffer::{
        Buffer,
        Cell,
    },
    Cmd,
    LayoutTree,
    Widget,
};
use crossterm::event::{
    Event,
    MouseEvent,
};
use ito_canvas::unicode_canvas::{
    Border,
    Canvas,
};
use sauron_vdom::Callback;
use std::{
    any::Any,
    fmt,
    fmt::Debug,
};
use stretch::{
    geometry::Size,
    result::Layout,
    style::{
        Dimension,
        Style,
    },
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
    pub on_click: Vec<Callback<sauron_vdom::Event, MSG>>,
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
                    Dimension::Points((self.label.len() + 2) as f32)
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
                    Dimension::Points(5.0)
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
    fn draw(&mut self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let width = layout.size.width.round() as usize;
        let height = layout.size.height.round() as usize;

        let left = loc_x;
        let top = loc_y;
        let bottom = top + height - 1;
        let right = left + width - 1;

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
        canvas.draw_rect((left, top), (right, bottom), border);
        buf.write_canvas(canvas);

        for (t, ch) in self.label.chars().enumerate() {
            let mut cell = Cell::new(ch);
            if self.focused {
                cell.bold();
            }
            buf.set_cell(loc_x + 1 + t, loc_y + 1, cell);
        }

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

    fn process_event(&mut self, event: Event) -> Vec<MSG> {
        eprintln!("button events..");
        match event {
            Event::Mouse(MouseEvent::Down(_btn, x, y, _modifier)) => {
                eprintln!("mouse is clicked");
                let s_event: sauron_vdom::Event =
                    sauron_vdom::event::MouseEvent::click(x as i32, y as i32)
                        .into();
                self.on_click
                    .iter()
                    .map(|cb| cb.emit(s_event.clone()))
                    .collect()
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
