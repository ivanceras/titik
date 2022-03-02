use crate::{buffer::Buffer, Cmd, Widget};
use expanse::result::Layout;
use expanse::{
    geometry::{Rect, Size},
    style::{Dimension, FlexDirection, PositionType, Style},
};
use ito_canvas::unicode_canvas::{Border, Canvas};
use std::fmt;

/// a flex box
#[derive(Default, Debug)]
pub struct FlexBox<MSG> {
    layout: Option<Layout>,
    children: Vec<Box<dyn Widget<MSG>>>,
    width: Option<f32>,
    height: Option<f32>,
    flex_direction: FlexDirection,
    scroll_top: f32,
    id: Option<String>,
    has_border: bool,
    is_rounded_border: bool,
    is_thick_border: bool,
    /// take as much height as possible
    is_expand_height: bool,
    /// take as much width as possible
    is_expand_width: bool,
}

impl<MSG> FlexBox<MSG> {
    ///create a new flexbox
    pub fn new() -> Self {
        FlexBox {
            layout: None,
            width: None,
            height: None,
            children: vec![],
            flex_direction: FlexDirection::Row,
            scroll_top: 0.0,
            id: None,
            has_border: false,
            is_rounded_border: false,
            is_thick_border: false,
            is_expand_height: false,
            is_expand_width: false,
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

    /// scroll the flexbox
    pub fn set_scroll_top(&mut self, scroll_top: f32) {
        self.scroll_top = scroll_top;
    }

    /// set if to expand the width or not
    pub fn set_expand_width(&mut self, is_expand_width: bool) {
        self.is_expand_width = is_expand_width;
    }

    /// set whether to expand the height or not
    pub fn set_expand_height(&mut self, is_expand_height: bool) {
        self.is_expand_height = is_expand_height;
    }

    pub fn set_border(&mut self, has_border: bool) {
        self.has_border = has_border;
    }

    pub fn set_thick_border(&mut self, use_thick_border: bool) {
        self.is_thick_border = use_thick_border;
    }

    pub fn set_rounded(&mut self, use_rounded_border: bool) {
        self.is_rounded_border = use_rounded_border;
    }
}

impl<MSG> Widget<MSG> for FlexBox<MSG>
where
    MSG: fmt::Debug + 'static,
{
    fn layout(&self) -> Option<&Layout> {
        self.layout.as_ref()
    }
    fn set_layout(&mut self, layout: Layout) {
        self.layout = Some(layout);
    }
    fn style(&self) -> Style {
        Style {
            position_type: PositionType::Relative,
            flex_direction: self.flex_direction,
            size: Size {
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
                    Dimension::Percent(1.0)
                },
                height: if let Some(height) = self.height {
                    Dimension::Points(height)
                } else {
                    Dimension::Percent(1.0)
                },
            },
            border: Rect {
                top: Dimension::Points(self.border_top()),
                bottom: Dimension::Points(self.border_bottom()),
                start: Dimension::Points(self.border_left()),
                end: Dimension::Points(self.border_right()),
            },
            ..Default::default()
        }
    }

    fn has_border(&self) -> bool {
        self.has_border
    }

    fn border_style(&self) -> Border {
        Border {
            use_thick_border: self.is_thick_border,
            has_top: true,
            has_bottom: true,
            has_left: true,
            has_right: true,
            is_top_left_rounded: self.is_rounded_border,
            is_top_right_rounded: self.is_rounded_border,
            is_bottom_left_rounded: self.is_rounded_border,
            is_bottom_right_rounded: self.is_rounded_border,
        }
    }

    fn draw(&self, buf: &mut Buffer) -> Vec<Cmd> {
        self.draw_border(buf);
        vec![]
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
