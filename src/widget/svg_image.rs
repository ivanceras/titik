use crate::{
    buffer::{
        Buffer,
        Cell,
    },
    symbol::bar,
    widget::ImageTrait,
    Cmd,
    LayoutTree,
    Widget,
};
use crossterm::style::Color;
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
    pub id: Option<String>,
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
            id: None,
            _phantom_msg: PhantomData,
        };
        image
    }
}

impl<MSG> ImageTrait for SvgImage<MSG> {
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
    fn style(&self) -> Style {
        self.image_style()
    }

    /// draw this button to the buffer, with the given computed layout
    fn draw(&mut self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        self.draw_image(buf, layout_tree)
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
