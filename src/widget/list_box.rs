use crate::{buffer::Buffer, Callback, Cmd, Event, Widget};
use expanse::{
    geometry::Size,
    result::Layout,
    style::{Dimension, FlexDirection, PositionType, Style},
};
use ito_canvas::unicode_canvas::{Border, Canvas};
use std::fmt;

/// a flex box
#[derive(Default, Debug)]
pub struct ListBox<MSG> {
    layout: Option<Layout>,
    list: Vec<String>,
    width: Option<f32>,
    height: Option<f32>,
    flex_direction: FlexDirection,
    scroll_top: f32,
    on_input: Vec<Callback<Event, MSG>>,
    id: Option<String>,
    use_divider: bool,
}

impl<MSG> ListBox<MSG>
where
    MSG: fmt::Debug + 'static,
{
    ///create a new flexbox
    pub fn new() -> Self {
        ListBox {
            layout: None,
            width: None,
            height: None,
            flex_direction: FlexDirection::Row,
            scroll_top: 0.0,
            on_input: vec![],
            list: vec![],
            id: None,
            use_divider: true,
        }
    }

    pub fn set_use_divider(&mut self, use_divider: bool) {
        self.use_divider = use_divider;
    }

    /// set the list of this listbox;
    pub fn set_list(&mut self, list: Vec<String>) {
        self.list = list;
    }

    fn draw_items(&self, buf: &mut Buffer) {
        for (j, li) in self.list.iter().enumerate() {
            let item_left = self.inner_left();
            let item_right = self.inner_right() - 1.0;

            let item_top = if self.use_divider {
                self.inner_top() + (j as f32 * 2.0)
            } else {
                self.inner_top() + j as f32
            };

            let item_bottom = item_top + 2.0;
            if item_bottom < self.bottom() {
                let mut canvas = Canvas::new();
                buf.write_str(
                    (item_left + 1.0) as usize,
                    (item_top + 1.0) as usize,
                    li,
                );
                if self.use_divider {
                    canvas.draw_horizontal_line(
                        (item_left as usize, item_bottom as usize),
                        (item_right as usize, item_bottom as usize),
                        false,
                    );
                    buf.write_canvas(canvas);
                }
            }
        }
    }
}

impl<MSG> Widget<MSG> for ListBox<MSG>
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
            ..Default::default()
        }
    }

    fn has_border(&self) -> bool {
        true
    }

    fn border_style(&self) -> Border {
        Border {
            use_thick_border: false,
            has_top: true,
            has_bottom: true,
            has_left: true,
            has_right: true,
            is_top_left_rounded: false,
            is_top_right_rounded: false,
            is_bottom_left_rounded: false,
            is_bottom_right_rounded: false,
        }
    }

    fn draw(&self, buf: &mut Buffer) -> Vec<Cmd> {
        self.draw_border(buf);
        self.draw_items(buf);
        vec![]
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
