use crate::{
    buffer::Buffer,
    cmd::Cmd,
    layout::LayoutTree,
    Widget,
};
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

    fn draw(
        &mut self,
        _buf: &mut Buffer,
        _layout_tree: &LayoutTree,
    ) -> Vec<Cmd> {
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
