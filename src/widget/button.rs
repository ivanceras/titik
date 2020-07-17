use crate::Event;
use crate::{
    buffer::{Buffer, Cell},
    mt_dom::Callback,
    Cmd, LayoutTree, Widget,
};
use crossterm::event::MouseEvent;
use ito_canvas::unicode_canvas::{Border, Canvas};
use std::{any::Any, fmt, fmt::Debug};
use stretch::{
    geometry::Size,
    style::{Dimension, Style},
};

/// A button widget
#[derive(PartialEq, Clone)]
pub struct Button<MSG>
where
    MSG: 'static,
{
    label: String,
    is_rounded: bool,
    width: Option<f32>,
    height: Option<f32>,
    focused: bool,
    on_click: Vec<Callback<Event, MSG>>,
    id: Option<String>,
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
    /// create a new button with label
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

    /// set the label of the button
    pub fn set_label<S: ToString>(&mut self, label: S) {
        self.label = label.to_string();
    }

    /// set to use a rounded border
    pub fn set_rounded(&mut self, rounded: bool) {
        self.is_rounded = rounded;
    }

    /// add to the click listener of this button
    pub fn add_click_listener(&mut self, cb: Callback<Event, MSG>) {
        self.on_click.push(cb);
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
        if event.is_mouse_click() {
            self.on_click
                .iter()
                .map(|cb| cb.emit(event.clone()))
                .collect()
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
