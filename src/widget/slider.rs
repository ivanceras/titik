use crate::{
    buffer::Buffer,
    cmd::Cmd,
    layout::LayoutTree,
    symbol,
    Widget,
};
use ito_canvas::unicode_canvas::Canvas;
use std::{
    any::Any,
    fmt,
    marker::PhantomData,
};
use stretch::{
    geometry::Size,
    style::{
        Dimension,
        Style,
    },
};

/// A slider with value from 0.0 to 1.0
#[derive(Debug)]
pub struct Slider<MSG> {
    value: f32,
    width: Option<f32>,
    id: Option<String>,
    _phantom_data: PhantomData<MSG>,
}

impl<MSG> Default for Slider<MSG> {
    fn default() -> Self {
        Slider {
            value: 0.0,
            width: None,
            id: None,
            _phantom_data: PhantomData,
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
}

impl<MSG> Widget<MSG> for Slider<MSG>
where
    MSG: fmt::Debug + 'static,
{
    fn style(&self) -> Style {
        Style {
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

    fn draw(&mut self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let width = layout.size.width.round() as usize;
        let _height = layout.size.height.round() as usize;
        let mut canvas = Canvas::new();
        let right = loc_x + width - 2;
        canvas.draw_horizontal_line((loc_x + 1, loc_y), (right, loc_y), false);
        buf.write_canvas(canvas);
        let slider_loc = (self.value * width as f32) as usize;
        buf.set_symbol(loc_x + slider_loc, loc_y, symbol::MIDDLE_BLOCK);
        vec![]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn set_size(&mut self, width: Option<f32>, _height: Option<f32>) {
        self.width = width;
    }

    fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }

    fn get_id(&self) -> &Option<String> {
        &self.id
    }
}
