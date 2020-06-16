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

pub trait Flex<MSG>: Widget<MSG> {
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

    fn get_symbols(&self) -> (&str, &str, &str, &str, &str, &str) {
        let mut top_left_symbol = if self.is_rounded_border() {
            rounded::TOP_LEFT
        } else {
            line::TOP_LEFT
        };

        let mut top_right_symbol = if self.is_rounded_border() {
            rounded::TOP_RIGHT
        } else {
            line::TOP_RIGHT
        };
        let mut bottom_left_symbol = if self.is_rounded_border() {
            rounded::BOTTOM_LEFT
        } else {
            line::BOTTOM_LEFT
        };
        let mut bottom_right_symbol = if self.is_rounded_border() {
            rounded::BOTTOM_RIGHT
        } else {
            line::BOTTOM_RIGHT
        };

        let mut horizontal_symbol = line::HORIZONTAL;
        let mut vertical_symbol = line::VERTICAL;

        // Note: the rounded border is override with square thick line since there is no thick
        // rounded corner
        if self.is_thick_border() {
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
        if self.has_border() {
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

    fn flex_style(&self) -> Style {
        Style {
            flex_direction: self.flex_direction(),
            size: Size {
                width: if let Some(width) = self.width() {
                    Dimension::Points(width)
                } else {
                    if self.is_expand_width() {
                        Dimension::Percent(1.0)
                    } else {
                        Dimension::Auto
                    }
                },
                height: if let Some(height) = self.height() {
                    Dimension::Points(height)
                } else {
                    if self.is_expand_height() {
                        Dimension::Percent(1.0)
                    } else {
                        Dimension::Auto
                    }
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

    fn draw_flex(
        &mut self,
        buf: &mut Buffer,
        layout_tree: &LayoutTree,
    ) -> Vec<Cmd> {
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
            .children_mut()
            .expect("must have children")
            .iter_mut()
            .zip(layout_tree.children_layout.iter())
            .flat_map(|(child, child_layout)| {
                child.draw(&mut inner_buf, child_layout)
            })
            .collect();

        for (j, line) in inner_buf.cells.iter().enumerate() {
            for (i, cell) in line.iter().enumerate() {
                if j >= self.scroll_top() as usize {
                    let y = j - self.scroll_top() as usize;
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
}
