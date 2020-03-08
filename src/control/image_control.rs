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
}

impl Image {
    pub fn new(bytes: Vec<u8>) -> Self {
        Image {
            image: image::load_from_memory(&bytes)
                .expect("unable to load from memory"),
            width: None,
            height: None,
        }
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = Some(width);
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = Some(height);
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

    /// draw this button to the buffer, with the given computed layout
    pub fn draw(&self, buf: &mut Buffer, layout_tree: LayoutTree) {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let width = layout.size.width.round() as usize;
        let height = layout.size.height.round() as usize;
        let img = self.image.thumbnail(width as u32, height as u32 * 2);
        let (img_width, img_height) = img.dimensions();
        let rgb = img.to_rgb();
        for (y, j) in (0..img_height as usize).step_by(2).enumerate() {
            for i in 0..img_width as usize {
                let mut cell = Cell::new(bar::HALF);
                let top_pixel = rgb.get_pixel(i as u32, j as u32);
                let bottom_pixel = rgb.get_pixel(i as u32, (j + 1) as u32);
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
                buf.set_cell(loc_x + 1 + i, loc_y + 1 + y, cell);
            }
        }
    }
}

impl From<Image> for Control {
    fn from(img: Image) -> Self {
        Control::Image(img)
    }
}
