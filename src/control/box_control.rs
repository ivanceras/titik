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
        Style,
    },
};

#[derive(Default)]
pub struct Box {
    pub children: Vec<Control>,
    pub style: Style,
}

impl Box {
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn draw(&self, buf: &mut Buffer, layout_tree: LayoutTree) {
        self.children
            .iter()
            .zip(layout_tree.children_layout.into_iter())
            .for_each(|(child, child_layout)| child.draw(buf, child_layout));
    }

    pub fn add_child<C: Into<Control>>(&mut self, child: C) {
        self.children.push(child.into());
    }
}
