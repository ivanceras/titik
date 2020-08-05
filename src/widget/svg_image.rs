use crate::{buffer::Buffer, widget::ImageTrait, Cmd, Widget};
use image::{self, DynamicImage, ImageBuffer, RgbaImage};
use std::{any::Any, fmt, marker::PhantomData};
use stretch::result::Layout;
use stretch::style::Style;

/// an Image made from svg document
pub struct SvgImage<MSG> {
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

impl<MSG> SvgImage<MSG> {
    /// create a new svg image from svg text document
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

        SvgImage {
            layout: None,
            image: DynamicImage::ImageRgba8(img_buffer),
            width: Some(width as f32 / 10.0),
            height: Some(height as f32 / 10.0 / 2.0),
            id: None,
            _phantom_msg: PhantomData,
        }
    }
}

impl<MSG> ImageTrait for SvgImage<MSG> {
    fn image_layout(&self) -> Option<&Layout> {
        self.layout.as_ref()
    }
    fn width(&self) -> Option<f32> {
        self.width
    }

    fn height(&self) -> Option<f32> {
        self.height
    }

    fn image(&self) -> &DynamicImage {
        &self.image
    }
}

impl<MSG> Widget<MSG> for SvgImage<MSG>
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
        self.image_style()
    }

    /// draw this button to the buffer, with the given computed layout
    fn draw(&self, buf: &mut Buffer) -> Vec<Cmd> {
        self.draw_image(buf)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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

impl<MSG> fmt::Debug for SvgImage<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SvgImage")
    }
}
