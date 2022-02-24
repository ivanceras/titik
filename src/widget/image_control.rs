use crate::symbol::bar;
use crate::{buffer::Buffer, buffer::Cell, Cmd, Widget};
use crossterm::style::Color;
use expanse::result::Layout;
use expanse::{
    geometry::Size,
    style::{Dimension, PositionType, Style},
};
use image::{self, DynamicImage, GenericImageView};
use std::{fmt, marker::PhantomData};

/// Image widget, supported formats: jpg, png
pub struct Image<MSG> {
    layout: Option<Layout>,
    image: DynamicImage,
    /// the width of cells used for this image
    width: Option<f32>,
    /// the height of unit cells, will be divided by 2 when used for computing
    /// style layout
    height: Option<f32>,
    id: Option<String>,
    _phantom_msg: PhantomData<MSG>,
}

impl<MSG> Image<MSG> {
    /// create a new image widget
    pub fn new(bytes: Vec<u8>) -> Self {
        Image {
            layout: None,
            image: image::load_from_memory(&bytes)
                .expect("unable to load from memory"),
            width: None,
            height: None,
            id: None,
            _phantom_msg: PhantomData,
        }
    }

    fn create_cells(&self, width: f32, height: f32) -> Vec<Vec<Cell>> {
        let width_multiplier = 1.0 / 2.0;
        let height_multiplier = 1.0 / 2.0;
        let w = width * width_multiplier;
        let h = width * height_multiplier;
        let img = self.image.thumbnail(w as u32, h as u32);
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
}

impl<MSG> Widget<MSG> for Image<MSG>
where
    MSG: 'static,
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

    /// draw this button to the buffer, with the given computed layout
    fn draw(&self, buf: &mut Buffer) -> Vec<Cmd> {
        let layout = self.layout().expect("must have a layout");
        let loc_x = layout.location.x;
        let loc_y = layout.location.y;
        let width = layout.size.width;
        let height = layout.size.height;

        let bottom = loc_y + height - 1.0;
        let right = loc_x + width - 1.0;

        let cells = self.create_cells(width, height);
        for (y, line) in cells.iter().enumerate() {
            for (i, cell) in line.iter().enumerate() {
                let line_y = loc_y as usize + y;
                let cell_x = loc_x as usize + i;
                if line_y < bottom as usize {
                    buf.set_cell(cell_x, line_y, cell.clone());
                }
            }
        }
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

impl<MSG> fmt::Debug for Image<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image")
    }
}
