use crate::Event;
use crate::{buffer::Buffer, Cmd, Widget};
use expanse::{
    geometry::{Rect, Size},
    result::Layout,
    style::{Dimension, FlexDirection, PositionType, Style},
};
use ito_canvas::unicode_canvas::{Border, Canvas};
use std::fmt;

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
#[derive(Debug)]
pub struct TabBox<MSG> {
    layout: Option<Layout>,
    /// The labels for each of the tabs for each corresponding children
    tab_labels: Vec<String>,
    active_tab: usize,
    /// The children could be flexbox, group_box,
    children: Vec<Vec<Box<dyn Widget<MSG>>>>,
    width: Option<f32>,
    height: Option<f32>,
    flex_direction: FlexDirection,
    scroll_top: f32,
    id: Option<String>,
    has_border: bool,
    is_rounded_border: bool,
    is_thick_border: bool,
}

impl<MSG> TabBox<MSG> {
    /// creates a new tab box
    pub fn new() -> Self {
        TabBox {
            layout: None,
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

    fn ensure_has_tab_index(&mut self, tab_index: usize) {
        let children_len = self.children.len();
        if tab_index >= children_len {
            for _i in children_len..tab_index {
                self.children.push(vec![]);
            }
        }
    }

    pub fn add_child_to_tab(
        &mut self,
        tab_index: usize,
        child: Box<dyn Widget<MSG>>,
    ) -> bool {
        self.ensure_has_tab_index(tab_index);
        if let Some(existing) = self.children.get_mut(tab_index) {
            existing.push(child);
        } else {
            self.children.push(vec![child]);
        }
        true
    }

    /// set to vertical column direction
    pub fn vertical(&mut self) {
        self.flex_direction = FlexDirection::Column;
    }

    /// set to horizontal row direction
    pub fn horizontal(&mut self) {
        self.flex_direction = FlexDirection::Row;
    }

    /// return the calculation of tab_level
    fn tab_label_rects(&self) -> Vec<((usize, usize), (usize, usize))> {
        let layout = self.layout.expect("must have a layout");
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let left_pad = 3;
        let mut left = loc_x + left_pad;
        let top = loc_y;
        let height = 2;
        let bottom = top + height;
        let mut tab_rects: Vec<((usize, usize), (usize, usize))> = vec![];
        for label in self.tab_labels.iter() {
            let label_width = label.len() + 3;
            let right = left + label_width;
            tab_rects.push(((left, top), (right, bottom)));
            left += label_width;
        }
        tab_rects
    }

    fn hit_tab_label(&self, x: usize, y: usize) -> Option<usize> {
        let tab_rects = self.tab_label_rects();
        for (tab_index, ((left, top), (right, bottom))) in
            tab_rects.iter().enumerate()
        {
            if x >= *left && x <= *right && y >= *top && y <= *bottom {
                return Some(tab_index);
            }
        }
        None
    }

    ///  ╭──────╮──────┬──────╮
    ///  │ tab1 │ tab2 │ tab2 │
    ///  └──────┴──────┴──────┴
    pub fn draw_labels(&self, buf: &mut Buffer, canvas: &mut Canvas) {
        let layout = self.layout.expect("must have a layout");
        let _loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let _left_pad = 3;
        let top = loc_y;
        let _width = layout.size.width.round() as usize;
        let height = 2;
        let _bottom = top + height;

        let tab_rects = self.tab_label_rects();

        // draw the tabs
        for (tab_index, ((left, top), (right, bottom))) in
            tab_rects.iter().enumerate()
        {
            let tab_label = &self.tab_labels[tab_index];
            if self.active_tab == tab_index {
                buf.write_bold_str(left + 2, top + 1, tab_label);
                canvas.eraser_horizontal_line(
                    (*left, *bottom),
                    (*right, *bottom),
                    false,
                );
            } else {
                buf.write_str(left + 2, top + 1, tab_label);
            }
            canvas.draw_rect(
                (*left, *top),
                (*right, *bottom),
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
        }
        // redraw the active tab
        let ((active_left, active_top), (active_right, active_bottom)) =
            &tab_rects[self.active_tab];
        canvas.draw_rect(
            (*active_left, *active_top),
            (*active_right, *active_bottom),
            Border {
                use_thick_border: false,
                has_top: true,
                has_bottom: false,
                has_left: true,
                has_right: true,

                is_top_left_rounded: true,
                is_top_right_rounded: true,
                is_bottom_left_rounded: false,
                is_bottom_right_rounded: false,
            },
        );
    }

    /// set the tab labels
    pub fn set_tab_labels(&mut self, labels: Vec<String>) {
        self.tab_labels = labels;
    }

    /// set the active tab index
    pub fn set_active_tab(&mut self, index: usize) {
        if index < self.tab_labels.len() {
            self.active_tab = index;
        }
    }
}

impl<MSG> Widget<MSG> for TabBox<MSG>
where
    MSG: fmt::Debug + 'static,
{
    fn layout(&self) -> Option<&Layout> {
        self.layout.as_ref()
    }
    fn set_layout(&mut self, layout: Layout) {
        self.layout = Some(layout);
    }

    /// the tab box has an offset of 2 from the top before drawing the child components
    fn get_offset(&self) -> (f32, f32) {
        (0.0, 2.0)
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
            min_size: Size {
                height: Dimension::Points(3.0),
                ..Default::default()
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

    fn draw(&self, buf: &mut Buffer) -> Vec<Cmd> {
        // offset the position of the top_border
        let layout = self.layout.expect("must have a layout");
        let loc_x = layout.location.x.round();
        let loc_y = layout.location.y.round();
        let width = layout.size.width.round();
        let height = layout.size.height.round();
        let mut canvas = Canvas::new();
        let left = loc_x as usize;
        let right = left + width as usize - 1;
        let top = (loc_y + 2.0) as usize;
        let bottom = top + height as usize - 3;
        let border = Border {
            use_thick_border: false,
            has_top: true,
            has_bottom: true,
            has_left: true,
            has_right: true,
            is_top_left_rounded: true,
            is_top_right_rounded: true,
            is_bottom_left_rounded: true,
            is_bottom_right_rounded: true,
        };

        canvas.draw_rect((left, top), (right, bottom), border);

        self.draw_labels(buf, &mut canvas);
        buf.write_canvas(canvas);
        vec![]
    }

    /// add a child widget to the current active tab
    fn add_child(&mut self, child: Box<dyn Widget<MSG>>) -> bool {
        self.add_child_to_tab(self.active_tab, child)
    }

    fn children(&self) -> Option<&[Box<dyn Widget<MSG>>]> {
        self.children
            .get(self.active_tab)
            .map(|children| children.as_slice())
    }

    fn children_mut(&mut self) -> Option<&mut [Box<dyn Widget<MSG>>]> {
        self.children
            .get_mut(self.active_tab)
            .map(|children| children.as_mut_slice())
    }

    // TODO: use remove_item when it will be stabilized
    fn take_child(&mut self, index: usize) -> Option<Box<dyn Widget<MSG>>> {
        Some(self.children[self.active_tab].remove(index))
    }

    fn child_mut<'a>(
        &'a mut self,
        index: usize,
    ) -> Option<&'a mut Box<dyn Widget<MSG>>> {
        self.children[self.active_tab].get_mut(index)
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

    fn process_event(&mut self, event: Event) -> Vec<MSG> {
        if event.is_mouse_click() {
            let (x, y) =
                event.extract_location().expect("must have a location");
            if let Some(active_tab) = self.hit_tab_label(x as usize, y as usize)
            {
                self.active_tab = active_tab;
            }
            vec![]
        } else {
            vec![]
        }
    }
}
