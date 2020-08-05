//! Shared image functions
//! ImageTrait is used so as to avoid conflict with image crate and Image structs.
//!
use crate::{
    buffer::{Buffer, Cell},
    symbol::bar,
    Cmd,
};
use crossterm::style::Color;
use image::{self, DynamicImage, GenericImageView};
use stretch::result::Layout;
use stretch::{
    geometry::Size,
    style::{Dimension, Style},
};

pub trait ImageTrait {
    fn layout(&self) -> Option<&Layout>;
    fn width(&self) -> Option<f32>;
    fn height(&self) -> Option<f32>;

    fn image_style(&self) -> Style {
        Style {
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
            ..Default::default()
        }
    }
    fn image(&self) -> &DynamicImage;

    /// create TUI cells based on the image.
    fn create_cells(&self, width: f32, height: f32) -> Vec<Vec<Cell>> {
        let img = self.image().thumbnail(width as u32, height as u32);
        let (img_width, img_height) = img.dimensions();
        let rgb = img.to_rgb();
        (0..img_height as usize - 1)
            .step_by(2)
            .enumerate()
            .map(|(_y, j)| {
                (0..img_width as usize)
                    .map(|i| {
                        let mut cell = Cell::new(bar::HALF);
                        let top_pixel = rgb.get_pixel(i as u32, j as u32);
                        let bottom_pixel =
                            rgb.get_pixel(i as u32, (j + 1) as u32);
                        let top_color = Color::Rgb {
                            r: top_pixel[0],
                            g: top_pixel[1],
                            b: top_pixel[2],
                        };
                        let bottom_color = Color::Rgb {
                            r: bottom_pixel[0],
                            g: bottom_pixel[1],
                            b: bottom_pixel[2],
                        };
                        cell.background(top_color);
                        cell.color(bottom_color);
                        cell
                    })
                    .collect()
            })
            .collect()
    }

    fn draw_image(&self, buf: &mut Buffer) -> Vec<Cmd> {
        let layout = self.layout().expect("must have a layout");
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let cells = self.create_cells(layout.size.width, layout.size.height);
        for (y, line) in cells.iter().enumerate() {
            for (i, cell) in line.iter().enumerate() {
                if i < layout.size.width as usize {
                    buf.set_cell(loc_x + i, loc_y + y, cell.clone());
                }
            }
        }
        vec![]
    }
}
