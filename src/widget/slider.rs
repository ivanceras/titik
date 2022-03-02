use crate::Callback;
use crate::Event;
use crate::{buffer::Buffer, cmd::Cmd, symbol, Widget};
use expanse::result::Layout;
use expanse::{
    geometry::Size,
    style::{Dimension, PositionType, Style},
};
use ito_canvas::unicode_canvas::Canvas;
use std::fmt;

/// A slider with value from 0.0 to 1.0
#[derive(Debug)]
pub struct Slider<MSG> {
    layout: Option<Layout>,
    value: f32,
    width: Option<f32>,
    id: Option<String>,
    use_thick_track: bool,
    on_input: Vec<Callback<Event, MSG>>,
}

impl<MSG> Default for Slider<MSG> {
    fn default() -> Self {
        Slider {
            layout: None,
            value: 0.0,
            width: None,
            id: None,
            use_thick_track: false,
            on_input: vec![],
        }
    }
}

impl<MSG> Slider<MSG> {
    /// create a new slider with value
    pub fn new(value: f32) -> Self {
        Slider {
            value,
            ..Default::default()
        }
    }

    /// set the value of this slider
    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    /// set the use thick track, default is false
    pub fn use_thick_track(&mut self, use_thick: bool) {
        self.use_thick_track = use_thick;
    }
}

impl<MSG> Widget<MSG> for Slider<MSG>
where
    MSG: fmt::Debug + 'static,
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
                width: Dimension::Percent(1.0),
                height: Dimension::Points(1.0),
            },
            min_size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Points(1.0),
            },
            ..Default::default()
        }
    }

    fn has_border(&self) -> bool {
        false
    }

    fn draw(&self, buf: &mut Buffer) -> Vec<Cmd> {
        let mut canvas = Canvas::new();
        canvas.draw_horizontal_line(
            (self.left() as usize + 1, self.top() as usize),
            (self.right() as usize - 2, self.top() as usize),
            self.use_thick_track,
        );
        buf.write_canvas(canvas);
        let slider_loc = (self.value * self.layout_width()) as usize;
        buf.set_symbol(
            self.left() as usize + slider_loc,
            self.top() as usize,
            symbol::MIDDLE_BLOCK,
        );
        vec![]
    }

    fn set_size(&mut self, width: Option<f32>, _height: Option<f32>) {
        self.width = width;
    }

    fn process_event(&mut self, event: Event) -> Vec<MSG> {
        let layout = self.layout.expect("must have a layout");
        if event.is_mouse_click() || event.is_mouse_drag() {
            let (x, _y) =
                event.extract_location().expect("must have a location");
            self.value = (x as f32 - self.left()) / self.layout_width();
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
