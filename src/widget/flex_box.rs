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
    Cmd,
    LayoutTree,
    Widget,
};
use crossterm::Command;
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

#[derive(Default, Debug)]
pub struct FlexBox {
    pub children: Vec<Box<dyn Widget>>,
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

    /// remove all children of this flex_box
    pub fn clear_children(&mut self) {
        self.children = vec![];
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

    fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        self.children
            .iter()
            .zip(layout_tree.children_layout.iter())
            .flat_map(|(child, child_layout)| child.draw(buf, child_layout))
            .collect()
    }

    fn add_child(&mut self, child: Box<dyn Widget>) -> bool {
        self.children.push(child);
        true
    }

    fn children(&self) -> Option<&[Box<dyn Widget>]> {
        Some(&self.children)
    }

    fn children_mut(&mut self) -> Option<&mut [Box<dyn Widget>]> {
        Some(&mut self.children)
    }

    fn child_mut<'a>(
        &'a mut self,
        index: usize,
    ) -> Option<&'a mut Box<dyn Widget>> {
        self.children.get_mut(index)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_mut() {
        let mut fb = FlexBox::new();
        let btn1 = Button::new("Btn1");
        println!("btn1: {:?}", btn1);
        fb.add_child(Box::new(btn1));

        let mut btn1_mut = fb.child_mut(0).expect("must have a child 0");
        let mut btn1_cast = btn1_mut
            .as_any_mut()
            .downcast_mut::<Button>()
            .expect("must be a button");
        btn1_cast.set_size(Some(20.0), Some(10.0));
        println!("btn1_cast: {:?}", btn1_cast);
        assert_eq!(Some(20.0), btn1_cast.width);
        assert_eq!(Some(10.0), btn1_cast.height);
    }

    #[test]
    fn test_children_mut() {
        let mut fb = FlexBox::new();
        let btn1 = Button::new("Btn1");
        println!("btn1: {:?}", btn1);
        fb.add_child(Box::new(btn1));

        let mut children = fb.children_mut().expect("must have children");
        let mut btn0 = children[0]
            .as_any_mut()
            .downcast_mut::<Button>()
            .expect("must be a button");

        btn0.set_size(Some(40.0), Some(100.0));
        assert_eq!(Some(40.0), btn0.width);
        assert_eq!(Some(100.0), btn0.height);
    }
}
