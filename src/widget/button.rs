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
use std::marker::PhantomData;
use std::{fmt, fmt::Debug};

/// A button widget
#[derive(PartialEq, Clone)]
pub struct Button<MSG> {
    layout: Option<Layout>,
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
            layout: None,
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

impl<MSG> Button<MSG> {
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

    pub fn on_click<F>(&mut self, f: F)
    where
        F: FnMut(Event) -> MSG + 'static,
    {
        self.on_click.push(f.into());
    }

    fn border_top(&self) -> f32 {
        1.0
    }

    fn border_bottom(&self) -> f32 {
        1.0
    }

    fn border_left(&self) -> f32 {
        1.0
    }

    fn border_right(&self) -> f32 {
        1.0
    }
}

impl<MSG> Widget<MSG> for Button<MSG> {
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

    /// draw this button to the buffer, with the given computed layout
    fn draw(&self, buf: &mut Buffer) -> Vec<Cmd> {
        let layout = self.layout.expect("must have a layout");
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

    fn set_size(&mut self, width: Option<f32>, height: Option<f32>) {
        self.width = width;
        self.height = height;
    }

    fn process_event(&mut self, event: Event) -> Vec<MSG> {
        if event.is_mouse_click() {
            self.on_click
                .iter_mut()
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
