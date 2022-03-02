use crate::{buffer::Buffer, Cmd, Widget};
use expanse::{
    geometry::{Rect, Size},
    result::Layout,
    style::{Dimension, FlexDirection, PositionType, Style},
};
use ito_canvas::unicode_canvas::{Border, Canvas};
use std::fmt;

/// Group elements together
/// Radio buttons in the same group will have an exclusive behavior
#[derive(Default, Debug)]
pub struct GroupBox<MSG> {
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
    label: Option<String>,
}

impl<MSG> GroupBox<MSG>
where
    MSG: fmt::Debug + 'static,
{
    /// create a new groupbox
    pub fn new() -> Self {
        GroupBox {
            layout: None,
            width: None,
            height: None,
            children: vec![],
            flex_direction: FlexDirection::Column,
            scroll_top: 0.0,
            id: None,
            has_border: true,
            is_rounded_border: true,
            is_thick_border: false,
            label: None,
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

    /// set the label of the group box
    pub fn set_label(&mut self, label: &str) {
        self.label = Some(label.to_string());
    }

    fn draw_label(&self, buf: &mut Buffer) {
        if let Some(label) = &self.label {
            for (t, ch) in label.chars().enumerate() {
                buf.set_symbol(
                    self.left() as usize + 3 + t,
                    self.top() as usize,
                    ch,
                );
            }
        }
    }
}

impl<MSG> Widget<MSG> for GroupBox<MSG>
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
        self.draw_label(buf);
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
