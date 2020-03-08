use super::{
    Control,
    LayoutTree,
};
use crate::{
    buffer::{
        Buffer,
        Cell,
    },
    symbol::{
        bar,
        line,
        rounded,
    },
};
use crossterm::style::Color;
use image::{
    self,
    DynamicImage,
    GenericImageView,
};
use stretch::{
    geometry::Size,
    node::{
        Node,
        Stretch,
    },
    number::Number,
    result::Layout,
    style::{
        Dimension,
        Style,
    },
};

pub struct Image {
    pub image: DynamicImage,
    /// the width of cells used for this image
    pub width: Option<f32>,
    /// the height of unit cells, will be divided by 2 when used for computing
    /// style layout
    pub height: Option<f32>,
    pub cells: Vec<Vec<Cell>>,
}

impl Image {
    pub fn new(bytes: Vec<u8>) -> Self {
        let mut image = Image {
            image: image::load_from_memory(&bytes)
                .expect("unable to load from memory"),
            width: None,
            height: None,
            cells: vec![],
        };
        image.create_cells();
        image
    }

    pub fn set_size(&mut self, width: Option<f32>, height: Option<f32>) {
        let size_changed = self.width != width || self.height != height;
        if size_changed {
            self.width = width;
            self.height = height;
            self.create_cells();
        }
    }

    pub fn style(&self) -> Style {
        Style {
            size: Size {
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
                    Dimension::Auto
                },
                height: if let Some(height) = self.height {
                    Dimension::Points(height / 2.0)
                } else {
                    Dimension::Auto
                },
            },
            ..Default::default()
        }
    }

    /// the cells will be stored in the image control to avoid re-creation every after redraw
    fn create_cells(&mut self) {
        let width = self.width.unwrap_or(10.0);
        let height = self.height.unwrap_or(10.0);
        let img = self.image.thumbnail(width as u32, height as u32);
        let (img_width, img_height) = img.dimensions();
        let rgb = img.to_rgb();
        let cells = (0..img_height as usize)
            .step_by(2)
            .enumerate()
            .map(|(y, j)| {
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
            .collect();
        self.cells = cells;
    }

    /// draw this button to the buffer, with the given computed layout
    pub fn draw(&self, buf: &mut Buffer, layout_tree: LayoutTree) {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        for (y, line) in self.cells.iter().enumerate() {
            for (i, cell) in line.iter().enumerate() {
                buf.set_cell(loc_x + 1 + i, loc_y + 1 + y, cell.clone());
            }
        }
    }
}

impl From<Image> for Control {
    fn from(img: Image) -> Self {
        Control::Image(img)
    }
}
