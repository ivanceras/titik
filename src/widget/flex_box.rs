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
    LayoutTree,
    Widget,
};
use std::any::Any;
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

    fn add_child(&mut self, child: Box<dyn Widget>) -> bool {
        self.children.push(child);
        true
    }

    fn children(&self) -> Option<&[Box<dyn Widget>]> {
        Some(&self.children)
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

        let mut btn1_mut = fb.child_mut(0);
        if let Some(btn1_mut) = btn1_mut {
            let mut btn1_cast = btn1_mut
                .as_any_mut()
                .downcast_mut::<Button>()
                .expect("must be a button");
            btn1_cast.set_size(Some(20.0), Some(10.0));
            println!("btn1_cast: {:?}", btn1_cast);
            assert_eq!(Some(20.0), btn1_cast.width);
            assert_eq!(Some(10.0), btn1_cast.height);
        }
    }
}
