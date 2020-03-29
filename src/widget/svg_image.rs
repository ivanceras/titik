use crate::{
    buffer::{
        Buffer,
        Cell,
    },
    symbol::bar,
    Cmd,
    LayoutTree,
    Widget,
};
use crossterm::{
    event::Event,
    style::Color,
    Command,
};
use image::{
    self,
    DynamicImage,
    GenericImageView,
    ImageBuffer,
    RgbaImage,
};
use std::{
    any::Any,
    fmt,
    marker::PhantomData,
};
use stretch::{
    geometry::Size,
    style::{
        Dimension,
        Style,
    },
};

pub struct SvgImage<MSG> {
    pub image: DynamicImage,
    /// the width of cells used for this image
    pub width: Option<f32>,
    /// the height of unit cells, will be divided by 2 when used for computing
    /// style layout
    pub height: Option<f32>,
    pub cells: Vec<Vec<Cell>>,
    _phantom_msg: PhantomData<MSG>,
}

impl<MSG> SvgImage<MSG> {
    pub fn new(svg: String) -> Self {
        let rtree =
            resvg::usvg::Tree::from_str(&svg, &resvg::usvg::Options::default())
                .expect("must be parse into tree");
        let svg_size = rtree.svg_node().size;
        //TODO: get the size when width and height is not defined
        let (width, height) =
            (svg_size.width() as u32, svg_size.height() as u32);
        let backend = resvg::default_backend();
        let mut img = backend
            .render_to_image(&rtree, &resvg::Options::default())
            .expect("must render to image");
        let rgba_vec = img.make_rgba_vec();
        let img_buffer: RgbaImage =
            ImageBuffer::from_raw(width, height, rgba_vec)
                .expect("must construct imagebuffer");

        let mut image = SvgImage {
            image: DynamicImage::ImageRgba8(img_buffer),
            width: Some(width as f32 / 10.0),
            height: Some(height as f32 / 10.0 / 2.0),
            cells: vec![],
            _phantom_msg: PhantomData,
        };
        image.create_cells();
        image
    }

    /// the cells will be stored in the image control to avoid re-creation every after redraw
    fn create_cells(&mut self) {
        let width = self.width.unwrap_or(10.0);
        let height = self.height.unwrap_or(10.0) * 2.0;
        let img = self.image.thumbnail(width as u32, height as u32);
        let (img_width, img_height) = img.dimensions();
        let rgb = img.to_rgb();
        let cells = (0..img_height as usize)
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
            .collect();
        self.cells = cells;
    }
}

impl<MSG> Widget<MSG> for SvgImage<MSG>
where
    MSG: 'static,
{
    fn style(&self) -> Style {
        Style {
            size: Size {
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
                    Dimension::Auto
                },
                height: if let Some(height) = self.height {
                    Dimension::Points(height)
                } else {
                    Dimension::Auto
                },
            },
            ..Default::default()
        }
    }

    /// draw this button to the buffer, with the given computed layout
    fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        for (y, line) in self.cells.iter().enumerate() {
            for (i, cell) in line.iter().enumerate() {
                if i < layout.size.width as usize {
                    buf.set_cell(loc_x + i, loc_y + y, cell.clone());
                }
            }
        }
        vec![]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn set_size(&mut self, width: Option<f32>, height: Option<f32>) {
        let size_changed = self.width != width || self.height != height;
        if size_changed {
            self.width = width;
            self.height = height;
            self.create_cells();
        }
    }
}

impl<MSG> fmt::Debug for SvgImage<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SvgImage")
    }
}