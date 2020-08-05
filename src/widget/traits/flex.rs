use crate::{buffer::Buffer, Cmd, Widget};
use ito_canvas::unicode_canvas::{Border, Canvas};
use stretch::result::Layout;
use stretch::{
    geometry::{Rect, Size},
    style::{
        AlignContent, AlignItems, AlignSelf, Dimension, FlexDirection,
        FlexWrap, JustifyContent, Overflow, PositionType, Style,
    },
};

pub trait Flex<MSG>: Widget<MSG> {
    fn layout(&self) -> Option<&Layout>;
    fn has_border(&self) -> bool;
    fn is_rounded_border(&self) -> bool;
    fn is_thick_border(&self) -> bool;
    fn flex_direction(&self) -> FlexDirection;
    fn width(&self) -> Option<f32>;
    fn height(&self) -> Option<f32>;
    fn scroll_top(&self) -> f32;
    fn is_expand_width(&self) -> bool {
        false
    }
    fn is_expand_height(&self) -> bool {
        false
    }

    fn border_top(&self) -> f32 {
        if self.has_border() {
            1.0
        } else {
            0.0
        }
    }

    fn border_bottom(&self) -> f32 {
        if self.has_border() {
            1.0
        } else {
            0.0
        }
    }

    fn border_left(&self) -> f32 {
        if self.has_border() {
            1.0
        } else {
            0.0
        }
    }

    fn border_right(&self) -> f32 {
        if self.has_border() {
            1.0
        } else {
            0.0
        }
    }

    fn draw_border(
        &self,
        buf: &mut Buffer,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        if self.has_border() {
            let border = Border {
                use_thick_border: self.is_thick_border(),
                has_top: true,
                has_bottom: true,
                has_left: true,
                has_right: true,
                is_top_left_rounded: self.is_rounded_border(),
                is_top_right_rounded: self.is_rounded_border(),
                is_bottom_left_rounded: self.is_rounded_border(),
                is_bottom_right_rounded: self.is_rounded_border(),
            };
            let loc_x = x.round();
            let loc_y = y.round();
            let width = width.round();
            let height = height.round();

            let left = loc_x as usize;
            let top = loc_y as usize;
            let bottom = (loc_y + height - 1.0) as usize;
            let right = (loc_x + width - 1.0) as usize;
            let mut canvas = Canvas::new();
            canvas.draw_rect((left, top), (right, bottom), border);
            for (i, j, ch) in canvas.get_cells() {
                buf.set_symbol(i, j, ch);
            }
        }
    }

    fn flex_style(&self) -> Style {
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

    fn draw_flex(&self, buf: &mut Buffer) -> Vec<Cmd> {
        let layout = self.layout().expect("must have a layout");
        let loc_x = layout.location.x.round();
        let loc_y = layout.location.y.round();
        let width = layout.size.width.round();
        let height = layout.size.height.round();

        self.draw_border(buf, loc_x, loc_y, width, height);
        vec![]
    }
}
