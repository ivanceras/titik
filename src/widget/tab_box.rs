use crate::{
    buffer::{
        Buffer,
        Cell,
    },
    symbol::{
        bar,
        line,
        rounded,
        thick_line,
    },
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

/// A Tab box contains multiple box which
/// can only be shown one at a time
///```ignore
///        ╭──────╮
///        │ tab1 ├─tab2─┬─tab2─╮
///     ┌──┘      └──────┴──────┴────────┐
///     │                                │
///     │                                │
///     │                                │
///     │                                │
///     │                                │
///     └────────────────────────────────┘
/// ```

#[derive(Default, Debug)]
pub struct TabBox<MSG> {
    /// The labels for each of the tabs for each corresponding children
    pub tab_labels: Vec<String>,
    pub active_tab: usize,
    /// The children could be flexbox, group_box,
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

impl<MSG> TabBox<MSG> {
    pub fn new() -> Self {
        TabBox {
            width: None,
            height: None,
            tab_labels: vec![],
            active_tab: 0,
            children: vec![],
            flex_direction: FlexDirection::Column,
            scroll_top: 0.0,
            id: None,
            has_border: true,
            is_rounded_border: true,
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

    pub fn draw_labels(&self, buf: &mut Buffer, layout_tree: &LayoutTree) {
        //Issue: can not call of get_symbol since draw_labels is not part of Flex trait
        let horizontal_symbol = line::HORIZONTAL;
        let vertical_symbol = line::VERTICAL;
        let top_left_symbol = rounded::TOP_LEFT;
        let top_right_symbol = rounded::TOP_RIGHT;
        let bottom_left_symbol = line::BOTTOM_RIGHT;
        let bottom_right_symbol = line::BOTTOM_LEFT;

        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let left_pad = 3;
        let mut left = loc_x + left_pad;
        for (tab_index, label) in self.tab_labels.iter().enumerate() {
            let label_width = label.len() + 3;
            left += label_width;
            let right = left + label_width;
            let top = loc_y;
            let bottom = top + 2;

            for (t, ch) in label.chars().enumerate() {
                let mut cell = Cell::new(ch);
                // draw the label
                buf.set_cell(left + 2 + t, top + 1, cell);
            }

            // draw the horizontal lines
            for i in 0..label_width {
                // draw line at the top of the label
                buf.set_symbol(left + i, loc_y, horizontal_symbol);
                // erase the flex border with empty cell
                //if tab_index == self.active_tab {
                //buf.set_cell(left + i, bottom, Cell::empty());
                //}
            }
            // draw the vertical lines on both sides
            for j in 0..3 {
                // draw line at the top of the label
                buf.set_symbol(left, loc_y + j, vertical_symbol);
                buf.set_symbol(right, loc_y + j, vertical_symbol);
            }

            //draw only the left curves at the first tabs
            buf.set_symbol(left as usize, top as usize, top_left_symbol);
            buf.set_symbol(left as usize, bottom as usize, bottom_left_symbol);
            buf.set_symbol(right as usize, top as usize, top_right_symbol);
            buf.set_symbol(
                right as usize,
                bottom as usize,
                bottom_right_symbol,
            );
        }
    }
}

impl<MSG> Widget<MSG> for TabBox<MSG>
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
                    Dimension::Points((self.children.len() + 10) as f32)
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
        // offset the position of the top_border
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round();
        let loc_y = layout.location.y.round();
        let width = layout.size.width.round();
        let height = layout.size.height.round();
        self.draw_border(buf, loc_x, loc_y + 2.0, width, height - 2.0);
        self.draw_labels(buf, layout_tree);
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

impl<MSG> Flex<MSG> for TabBox<MSG>
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
