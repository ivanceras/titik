use super::LayoutTree;
use crate::{
    buffer::{
        Buffer,
        Cell,
    },
    symbol::{
        bar,
        line,
        rounded,
    },
    Widget,
};
use std::boxed;
use stretch::{
    geometry::Size,
    node::{
        Node,
        Stretch,
    },
    number::Number,
    result::Layout,
    style::{
        Dimension,
        FlexDirection,
        Style,
    },
};

pub struct FlexBox {
    pub children: Vec<boxed::Box<Widget>>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub flex_direction: FlexDirection,
}

impl FlexBox {
    pub fn new() -> Self {
        FlexBox {
            width: None,
            height: None,
            children: vec![],
            flex_direction: FlexDirection::Row,
        }
    }

    pub fn set_size(&mut self, width: Option<f32>, height: Option<f32>) {
        self.width = width;
        self.height = height;
    }

    /// set to vertical column direction
    pub fn vertical(&mut self) {
        self.flex_direction = FlexDirection::Column;
    }

    /// set to horizontal row direction
    pub fn horizontal(&mut self) {
        self.flex_direction = FlexDirection::Row;
    }
}

impl Widget for FlexBox {
    fn style(&self) -> Style {
        Style {
            flex_direction: self.flex_direction,
            size: Size {
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
                    Dimension::Auto
                },
                height: if let Some(height) = self.height {
                    Dimension::Points(height)
                } else {
                    Dimension::Auto
                },
            },
            max_size: Size {
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
                    Dimension::Auto
                },
                height: if let Some(height) = self.height {
                    Dimension::Points(height)
                } else {
                    Dimension::Auto
                },
            },
            ..Default::default()
        }
    }

    fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) {
        self.children
            .iter()
            .zip(layout_tree.children_layout.iter())
            .for_each(|(child, child_layout)| child.draw(buf, child_layout));
    }

    fn add_child(&mut self, child: boxed::Box<Widget>) -> bool {
        self.children.push(child);
        true
    }

    fn children(&self) -> Option<&[boxed::Box<dyn Widget>]> {
        Some(&self.children)
    }
}
