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
use ito_canvas::unicode_canvas::{
    Border,
    Canvas,
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
///     ╭──────╮──────┬──────╮
///     │ tab1 │ tab2 │ tab2 │
///  ┌──┘      └──────┴──────┴────────┐
///  │                                │
///  │                                │
///  │                                │
///  │                                │
///  │                                │
///  └────────────────────────────────┘
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

    ///  ╭──────╮──────┬──────╮
    ///  │ tab1 │ tab2 │ tab2 │
    ///  └──────┴──────┴──────┴
    pub fn draw_labels(
        &self,
        buf: &mut Buffer,
        canvas: &mut Canvas,
        layout_tree: &LayoutTree,
    ) {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let left_pad = 3;
        let mut left = loc_x + left_pad;
        let top = loc_y;
        let width = layout.size.width.round() as usize;
        let height = 2;
        let bottom = top + height;

        for (tab_index, label) in self.tab_labels.iter().enumerate() {
            let label_width = label.len() + 3;
            let right = left + label_width;
            buf.write_str(left + 2, top + 1, label);
            canvas.draw_rect(
                (left, top),
                (right, bottom),
                Border {
                    use_thick_border: false,
                    has_top: true,
                    has_bottom: tab_index != self.active_tab,
                    has_left: true,
                    has_right: true,

                    is_top_left_rounded: true,
                    is_top_right_rounded: true,
                    is_bottom_left_rounded: false,
                    is_bottom_right_rounded: false,
                },
            );

            left += label_width;
        }
        // draw a line to the rest of the width
        canvas.draw_horizontal_line(
            (left, bottom),
            (loc_x + width - left, bottom),
            false,
        );
        canvas.draw_horizontal_line((loc_x, bottom), (left_pad, bottom), false);
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
        let mut canvas = Canvas::new();
        let left = loc_x as usize;
        let right = left + width as usize;
        let top = (loc_y + 2.0) as usize;
        let bottom = top + height as usize - 2;
        let border = Border {
            use_thick_border: self.is_expand_width(),
            has_top: false,
            has_bottom: true,
            has_left: true,
            has_right: true,
            is_top_left_rounded: self.is_rounded_border(),
            is_top_right_rounded: self.is_rounded_border(),
            is_bottom_left_rounded: self.is_rounded_border(),
            is_bottom_right_rounded: self.is_rounded_border(),
        };

        self.draw_labels(buf, &mut canvas, layout_tree);
        canvas.draw_rect((left, top), (right, bottom), border);
        buf.write_canvas(canvas);
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
