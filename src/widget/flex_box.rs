use crate::{buffer::Buffer, Cmd, LayoutTree, Widget};

use std::{any::Any, fmt};
use stretch::{
    geometry::{Rect, Size},
    style::{Dimension, FlexDirection, Style},
};

#[derive(Default, Debug)]
pub struct FlexBox<MSG> {
    pub children: Vec<Box<dyn Widget<MSG>>>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub flex_direction: FlexDirection,
    pub scroll_top: f32,
	pub id: Option<String>,
}

impl<MSG> FlexBox<MSG> {
    pub fn new() -> Self {
        FlexBox {
            width: None,
            height: None,
            children: vec![],
            flex_direction: FlexDirection::Row,
            scroll_top: 0.0,
			id: None,
        }
    }

    /// remove all children of this flex_box
    pub fn clear_children(&mut self) {
        self.children = vec![];
    }

    /// set to vertical column direction
    pub fn vertical(&mut self) {
        self.flex_direction = FlexDirection::Column;
    }

    /// set to horizontal row direction
    pub fn horizontal(&mut self) {
        self.flex_direction = FlexDirection::Row;
    }

    pub fn set_scroll_top(&mut self, scroll_top: f32) {
        self.scroll_top = scroll_top;
    }
}

impl<MSG> Widget<MSG> for FlexBox<MSG>
where
    MSG: fmt::Debug + 'static,
{
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
            padding: Rect {
                top: Dimension::Points(0.0),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /*
    fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        self.children
            .iter()
            .zip(layout_tree.children_layout.iter())
            .flat_map(|(child, child_layout)| child.draw(buf, child_layout))
            .collect()
    }
    */

    fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let width = layout.size.width.round();
        let height = layout.size.height.round();
        let mut inner_buf = Buffer::new(width as usize, height as usize);
        let cmds = self
            .children
            .iter()
            .zip(layout_tree.children_layout.iter())
            .flat_map(|(child, child_layout)| {
                child.draw(&mut inner_buf, child_layout)
            })
            .collect();

        for (j, line) in inner_buf.cells.iter().enumerate() {
            for (i, cell) in line.iter().enumerate() {
                if j >= self.scroll_top as usize {
                    let y = j - self.scroll_top as usize;
                    buf.set_cell(loc_x + i, loc_y + y, cell.clone())
                }
            }
        }
        cmds
    }

    fn add_child(&mut self, child: Box<dyn Widget<MSG>>) -> bool {
        self.children.push(child);
        true
    }

    fn children(&self) -> Option<&[Box<dyn Widget<MSG>>]> {
        Some(&self.children)
    }

    fn children_mut(&mut self) -> Option<&mut [Box<dyn Widget<MSG>>]> {
        Some(&mut self.children)
    }

    fn child_mut<'a>(
        &'a mut self,
        index: usize,
    ) -> Option<&'a mut Box<dyn Widget<MSG>>> {
        self.children.get_mut(index)
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
	fn set_id(&mut self, id: &str){
		self.id = Some(id.to_string());
	}

	fn get_id(&self) -> &Option<String> {
		&self.id
	}
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_mut() {
        let mut fb = FlexBox::<()>::new();
        let btn1 = Button::new("Btn1");
        println!("btn1: {:?}", btn1);
        fb.add_child(Box::new(btn1));

        let btn1_mut = fb.child_mut(0).expect("must have a child 0");
        let btn1_cast = btn1_mut
            .as_any_mut()
            .downcast_mut::<Button<()>>()
            .expect("must be a button");
        btn1_cast.set_size(Some(20.0), Some(10.0));
        println!("btn1_cast: {:?}", btn1_cast);
        assert_eq!(Some(20.0), btn1_cast.width);
        assert_eq!(Some(10.0), btn1_cast.height);
    }

    #[test]
    fn test_children_mut() {
        let mut fb = FlexBox::<()>::new();
        let btn1 = Button::new("Btn1");
        println!("btn1: {:?}", btn1);
        fb.add_child(Box::new(btn1));

        let children = fb.children_mut().expect("must have children");
        let btn0 = children[0]
            .as_any_mut()
            .downcast_mut::<Button<()>>()
            .expect("must be a button");

        btn0.set_size(Some(40.0), Some(100.0));
        assert_eq!(Some(40.0), btn0.width);
        assert_eq!(Some(100.0), btn0.height);
    }
}
