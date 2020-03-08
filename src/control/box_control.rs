use super::{
    Control,
    LayoutTree,
};
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
};
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

pub struct Box {
    pub children: Vec<Control>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub alignment: Alignment,
}

pub enum Alignment {
    Vertical,
    Horizontal,
}

impl Box {
    pub fn new() -> Self {
        Box {
            width: None,
            height: None,
            children: vec![],
            alignment: Alignment::Horizontal,
        }
    }

    pub fn set_size(&mut self, width: Option<f32>, height: Option<f32>) {
        self.width = width;
        self.height = height;
    }

    /// set to vertical column direction
    pub fn vertical(&mut self) {
        self.alignment = Alignment::Vertical;
    }

    /// set to horizontal row direction
    pub fn horizontal(&mut self) {
        self.alignment = Alignment::Horizontal;
    }

    pub fn style(&self) -> Style {
        Style {
            flex_direction: match self.alignment {
                Alignment::Horizontal => FlexDirection::Row,
                Alignment::Vertical => FlexDirection::Column,
            },
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
            ..Default::default()
        }
    }

    pub fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) {
        self.children
            .iter()
            .zip(layout_tree.children_layout.iter())
            .for_each(|(child, child_layout)| child.draw(buf, child_layout));
    }

    pub fn add_child<C: Into<Control>>(&mut self, child: C) {
        self.children.push(child.into());
    }
}

impl From<Box> for Control {
    fn from(bax: Box) -> Self {
        Control::Box(bax)
    }
}
