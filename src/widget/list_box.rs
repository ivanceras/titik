use crate::{
    buffer::Buffer,
    Cmd,
    LayoutTree,
    Widget,
};
use std::{
    any::Any,
    fmt,
};
use stretch::{
    geometry::Size,
    style::{
        Dimension,
        FlexDirection,
        Style,
    },
};

/// a flex box
#[derive(Default, Debug)]
pub struct ListBox<MSG> {
    children: Vec<Box<dyn Widget<MSG>>>,
    width: Option<f32>,
    height: Option<f32>,
    flex_direction: FlexDirection,
    scroll_top: f32,
    id: Option<String>,
}

impl<MSG> ListBox<MSG> {
    ///create a new flexbox
    pub fn new() -> Self {
        ListBox {
            width: None,
            height: None,
            children: vec![],
            flex_direction: FlexDirection::Row,
            scroll_top: 0.0,
            id: None,
        }
    }
}

impl<MSG> Widget<MSG> for ListBox<MSG>
where
    MSG: fmt::Debug + 'static,
{
    fn style(&self) -> Style {
        Style {
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
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
                    Dimension::Auto
                },
                height: if let Some(height) = self.height {
                    Dimension::Points(height)
                } else {
                    Dimension::Points(1.0)
                },
            },
            ..Default::default()
        }
    }

    fn draw(&mut self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        buf.set_symbol(loc_x, loc_y, "This is listbox");
        vec![]
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

    fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }

    fn get_id(&self) -> &Option<String> {
        &self.id
    }
}
