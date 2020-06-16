use crate::{
    buffer::Buffer,
    symbol::{
        bar,
        line,
        rounded,
        thick_line,
    },
    Cmd,
    LayoutTree,
    Widget,
};

use std::{
    any::Any,
    fmt,
};
use stretch::{
    geometry::{
        Rect,
        Size,
    },
    style::{
        AlignContent,
        AlignItems,
        AlignSelf,
        Dimension,
        FlexDirection,
        FlexWrap,
        JustifyContent,
        Overflow,
        PositionType,
        Style,
    },
};

#[derive(Default, Debug)]
pub struct FlexBox<MSG> {
    pub children: Vec<Box<dyn Widget<MSG>>>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub flex_direction: FlexDirection,
    pub scroll_top: f32,
    pub id: Option<String>,
    pub has_border: bool,
    pub is_rounded_border: bool,
    pub is_thick_border: bool,
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
            has_border: false,
            is_rounded_border: false,
            is_thick_border: false,
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

    fn border_top(&self) -> f32 {
        if self.has_border {
            1.0
        } else {
            0.0
        }
    }

    fn border_bottom(&self) -> f32 {
        if self.has_border {
            1.0
        } else {
            0.0
        }
    }

    fn border_left(&self) -> f32 {
        if self.has_border {
            1.0
        } else {
            0.0
        }
    }

    fn border_right(&self) -> f32 {
        if self.has_border {
            1.0
        } else {
            0.0
        }
    }

    fn get_symbols(&self) -> (&str, &str, &str, &str, &str, &str) {
        let mut top_left_symbol = if self.is_rounded_border {
            rounded::TOP_LEFT
        } else {
            line::TOP_LEFT
        };

        let mut top_right_symbol = if self.is_rounded_border {
            rounded::TOP_RIGHT
        } else {
            line::TOP_RIGHT
        };
        let mut bottom_left_symbol = if self.is_rounded_border {
            rounded::BOTTOM_LEFT
        } else {
            line::BOTTOM_LEFT
        };
        let mut bottom_right_symbol = if self.is_rounded_border {
            rounded::BOTTOM_RIGHT
        } else {
            line::BOTTOM_RIGHT
        };

        let mut horizontal_symbol = line::HORIZONTAL;
        let mut vertical_symbol = line::VERTICAL;

        // Note: the rounded border is override with square thick line since there is no thick
        // rounded corner
        if self.is_thick_border {
            top_left_symbol = thick_line::TOP_LEFT;
            top_right_symbol = thick_line::TOP_RIGHT;
            bottom_left_symbol = thick_line::BOTTOM_LEFT;
            bottom_right_symbol = thick_line::BOTTOM_RIGHT;
            horizontal_symbol = thick_line::HORIZONTAL;
            vertical_symbol = thick_line::VERTICAL;
        }

        (
            top_left_symbol,
            top_right_symbol,
            bottom_left_symbol,
            bottom_right_symbol,
            horizontal_symbol,
            vertical_symbol,
        )
    }

    fn draw_border(&self, buf: &mut Buffer, layout_tree: &LayoutTree) {
        if self.has_border {
            let layout = layout_tree.layout;
            let loc_x = layout.location.x.round();
            let loc_y = layout.location.y.round();
            let width = layout.size.width.round();
            let height = layout.size.height.round();

            let (
                top_left_symbol,
                top_right_symbol,
                bottom_left_symbol,
                bottom_right_symbol,
                horizontal_symbol,
                vertical_symbol,
            ) = self.get_symbols();

            let bottom = loc_y + height - 1.0;
            let right = loc_x + width - 1.0;
            // draw the horizontal border
            for i in 0..width as usize {
                buf.set_symbol(
                    loc_x as usize + i,
                    loc_y as usize,
                    horizontal_symbol,
                );
                buf.set_symbol(
                    loc_x as usize + i,
                    bottom as usize,
                    horizontal_symbol,
                );
            }

            // draw the vertical border
            for j in 0..height as usize {
                buf.set_symbol(
                    loc_x as usize,
                    loc_y as usize + j,
                    vertical_symbol,
                );
                buf.set_symbol(
                    right as usize,
                    loc_y as usize + j,
                    vertical_symbol,
                );
            }

            buf.set_symbol(loc_x as usize, loc_y as usize, top_left_symbol);
            buf.set_symbol(loc_x as usize, bottom as usize, bottom_left_symbol);
            buf.set_symbol(right as usize, loc_y as usize, top_right_symbol);
            buf.set_symbol(
                right as usize,
                bottom as usize,
                bottom_right_symbol,
            );
        }
    }
}

impl<MSG> Widget<MSG> for FlexBox<MSG>
where
    MSG: fmt::Debug + 'static,
{
    //TODO: make this style assignable
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
            overflow: Overflow::Scroll,
            border: Rect {
                top: Dimension::Points(self.border_top()),
                bottom: Dimension::Points(self.border_bottom()),
                start: Dimension::Points(self.border_left()),
                end: Dimension::Points(self.border_right()),
            },
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            align_self: AlignSelf::FlexStart,
            align_content: AlignContent::FlexStart,
            flex_shrink: 1.0,
            flex_grow: 0.0,
            position: Rect {
                top: Dimension::Points(0.0),
                start: Dimension::Points(0.0),
                bottom: Dimension::Points(0.0),
                end: Dimension::Points(0.0),
            },
            margin: Rect {
                top: Dimension::Points(0.0),
                start: Dimension::Points(0.0),
                bottom: Dimension::Points(0.0),
                end: Dimension::Points(0.0),
            },
            padding: Rect {
                top: Dimension::Points(0.0),
                start: Dimension::Points(0.0),
                bottom: Dimension::Points(0.0),
                end: Dimension::Points(0.0),
            },
            flex_wrap: FlexWrap::NoWrap,
            position_type: PositionType::Relative,
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

    fn draw(&mut self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round();
        let loc_y = layout.location.y.round();
        let width = layout.size.width.round();
        let height = layout.size.height.round();

        let mut inner_buf = Buffer::new(
            width as usize
                - (self.border_left() + self.border_right()) as usize,
            height as usize
                - (self.border_top() + self.border_bottom()) as usize,
        );

        let cmds = self
            .children
            .iter_mut()
            .zip(layout_tree.children_layout.iter())
            .flat_map(|(child, child_layout)| {
                child.draw(&mut inner_buf, child_layout)
            })
            .collect();

        for (j, line) in inner_buf.cells.iter().enumerate() {
            for (i, cell) in line.iter().enumerate() {
                if j >= self.scroll_top as usize {
                    let y = j - self.scroll_top as usize;
                    buf.set_cell(
                        loc_x as usize + i,
                        loc_y as usize + y,
                        cell.clone(),
                    )
                }
            }
        }
        self.draw_border(buf, layout_tree);
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

    // TODO: use remove_item when it will be stabilized
    fn take_child(&mut self, index: usize) -> Option<Box<dyn Widget<MSG>>> {
        Some(self.children.remove(index))
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

    fn set_id(&mut self, id: &str) {
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
