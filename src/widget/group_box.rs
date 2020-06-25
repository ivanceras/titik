use crate::{
    buffer::Buffer,
    widget::Flex,
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
pub struct GroupBox<MSG> {
    pub children: Vec<Box<dyn Widget<MSG>>>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub flex_direction: FlexDirection,
    pub scroll_top: f32,
    pub id: Option<String>,
    pub has_border: bool,
    pub is_rounded_border: bool,
    pub is_thick_border: bool,
    pub label: Option<String>,
}

impl<MSG> GroupBox<MSG> {
    pub fn new() -> Self {
        GroupBox {
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

    pub fn set_scroll_top(&mut self, scroll_top: f32) {
        self.scroll_top = scroll_top;
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = Some(label.to_string());
    }

    pub fn draw_label(&self, buf: &mut Buffer, layout_tree: &LayoutTree) {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        if let Some(label) = &self.label {
            for (t, ch) in label.chars().enumerate() {
                buf.set_symbol(loc_x + 3 + t, loc_y, ch);
            }
        }
    }
}

impl<MSG> Widget<MSG> for GroupBox<MSG>
where
    MSG: fmt::Debug + 'static,
{
    fn style(&self) -> Style {
        Style {
            flex_direction: self.flex_direction(),
            size: Size {
                width: if let Some(width) = self.width() {
                    Dimension::Points(width)
                } else {
                    Dimension::Percent(1.0)
                },
                height: if let Some(height) = self.height() {
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
                top: Dimension::Points(1.0),
                start: Dimension::Points(1.0),
                bottom: Dimension::Points(0.0),
                end: Dimension::Points(0.0),
            },
            flex_wrap: FlexWrap::NoWrap,
            position_type: PositionType::Relative,
            ..Default::default()
        }
    }

    fn draw(&mut self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        let cmds = self.draw_flex(buf, layout_tree);
        self.draw_label(buf, layout_tree);
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

impl<MSG> Flex<MSG> for GroupBox<MSG>
where
    MSG: fmt::Debug + 'static,
{
    fn has_border(&self) -> bool {
        self.has_border
    }

    fn is_rounded_border(&self) -> bool {
        self.is_rounded_border
    }

    fn is_thick_border(&self) -> bool {
        self.is_thick_border
    }

    fn flex_direction(&self) -> FlexDirection {
        self.flex_direction
    }

    fn width(&self) -> Option<f32> {
        self.width
    }

    fn height(&self) -> Option<f32> {
        self.height
    }

    fn scroll_top(&self) -> f32 {
        self.scroll_top
    }
}
