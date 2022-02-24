use crate::Callback;
use crate::Event;
use crate::{buffer::Buffer, cmd::Cmd, symbol, Widget};
use crossterm::event::MouseEvent;
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

    fn draw(&self, buf: &mut Buffer) -> Vec<Cmd> {
        let layout = self.layout.expect("must have a layout");
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let width = layout.size.width.round() as usize;
        let _height = layout.size.height.round() as usize;
        let mut canvas = Canvas::new();
        let right = loc_x + width - 2;
        canvas.draw_horizontal_line(
            (loc_x + 1, loc_y),
            (right, loc_y),
            self.use_thick_track,
        );
        buf.write_canvas(canvas);
        let slider_loc = (self.value * width as f32) as usize;
        buf.set_symbol(loc_x + slider_loc, loc_y, symbol::MIDDLE_BLOCK);
        vec![]
    }

    fn set_size(&mut self, width: Option<f32>, _height: Option<f32>) {
        self.width = width;
    }

    fn process_event(&mut self, event: Event) -> Vec<MSG> {
        let layout = self.layout.expect("must have a layout");
        match event {
            Event::Mouse(MouseEvent::Down(_btn, x, _y, _modifier)) => {
                let cursor_loc = x as i32 - layout.location.x.round() as i32;
                let width = layout.size.width;
                let value = cursor_loc as f32 / width;
                self.value = value;
                vec![]
            }
            Event::Mouse(MouseEvent::Drag(_btn, x, _y, _modifier)) => {
                let cursor_loc = x as i32 - layout.location.x.round() as i32;
                let width = layout.size.width;
                let value = cursor_loc as f32 / width;
                self.value = value;
                vec![]
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
