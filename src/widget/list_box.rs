use crate::{buffer::Buffer, Callback, Cmd, Widget};
use crossterm::event::Event;
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

impl<MSG> ListBox<MSG> {
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

    fn draw_border(&self, buf: &mut Buffer) {
        let layout = self.layout.expect("must have a layout");
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let width = layout.size.width.round() as usize;
        let height = layout.size.height.round() as usize;

        let left = loc_x;
        let top = loc_y;
        let bottom = top + height - 1;
        let right = left + width - 1;

        let border = Border {
            use_thick_border: false,
            has_top: true,
            has_bottom: true,
            has_left: true,
            has_right: true,
            is_top_left_rounded: false,
            is_top_right_rounded: false,
            is_bottom_left_rounded: false,
            is_bottom_right_rounded: false,
        };
        let mut canvas = Canvas::new();
        canvas.draw_rect((left, top), (right, bottom), border);
        buf.write_canvas(canvas);
    }

    /// set the list of this listbox;
    pub fn set_list(&mut self, list: Vec<String>) {
        self.list = list;
    }

    fn draw_items(&self, buf: &mut Buffer) {
        let layout = self.layout.expect("must have a layout");
        let loc_x = layout.location.x;
        let loc_y = layout.location.y;
        let width = layout.size.width;
        let height = layout.size.height;
        let bottom = layout.location.y + height - 1.0;

        for (j, li) in self.list.iter().enumerate() {
            let item_left = loc_x + 1.0;
            let item_right = loc_x + width - 2.0;

            let item_top = if self.use_divider {
                loc_y + (j as f32 * 2.0)
            } else {
                loc_y + j as f32
            };

            let item_bottom = item_top + 2.0;
            if item_bottom < bottom {
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
