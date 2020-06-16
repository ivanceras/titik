use crate::{
    buffer::Buffer,
    Cmd,
    LayoutTree,
};
pub use button::Button;
pub use checkbox::Checkbox;
use crossterm::event::Event;
pub use flex_box::FlexBox;
pub use image_control::Image;
pub use radio::Radio;
use std::{
    any::Any,
    fmt,
};
use stretch::{
    node::{
        Node,
        Stretch,
    },
    result::Layout,
    style::Style,
};
pub use svg_image::SvgImage;
pub use text_area::TextArea;
pub use text_input::TextInput;

mod button;
mod checkbox;
mod flex_box;
mod image_control;
mod radio;
mod svg_image;
mod text_area;
mod text_input;

pub trait Widget<MSG>
where
    Self: fmt::Debug,
{
    fn style(&self) -> Style;
    fn add_child(&mut self, _child: Box<dyn Widget<MSG>>) -> bool {
        false
    }

    fn children(&self) -> Option<&[Box<dyn Widget<MSG>>]> {
        None
    }

    fn children_mut(&mut self) -> Option<&mut [Box<dyn Widget<MSG>>]> {
        None
    }

    fn child_mut(
        &mut self,
        _index: usize,
    ) -> Option<&mut Box<dyn Widget<MSG>>> {
        None
    }

    fn draw(&mut self, but: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd>;

    fn style_node(&self, stretch: &mut Stretch) -> Option<Node> {
        let children_styles = if let Some(children) = self.children() {
            children
                .iter()
                .filter_map(|c| c.style_node(stretch))
                .collect()
        } else {
            vec![]
        };
        stretch.new_node(self.style(), children_styles).ok()
    }

    fn set_focused(&mut self, _focused: bool) {}

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn set_size(&mut self, width: Option<f32>, height: Option<f32>);

    fn as_mut(&mut self) -> Option<&mut Self>
    where
        Self: Sized + 'static,
    {
        self.as_any_mut().downcast_mut::<Self>()
    }
    fn process_event(&mut self, _event: Event) -> Vec<MSG> {
        vec![]
    }

    ///  take the children at this index location
    fn take_child(&mut self, index: usize) -> Option<Box<dyn Widget<MSG>>> {
        None
    }

    fn set_id(&mut self, id: &str);

    fn get_id(&self) -> &Option<String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use std::boxed;
    use stretch::{
        geometry::*,
        number::Number,
    };

    #[test]
    fn layout() {
        let mut control = FlexBox::<()>::new();
        control.horizontal();
        let mut btn = Button::<()>::new("Hello");
        btn.set_size(Some(30.0), Some(34.0));

        control.add_child(boxed::Box::new(btn));

        let mut btn = Button::<()>::new("world");
        btn.set_size(Some(20.0), Some(10.0));
        control.add_child(boxed::Box::new(btn));

        let layout_tree = compute_layout(
            &mut control,
            Size {
                width: Number::Defined(100.0),
                height: Number::Defined(100.0),
            },
        );

        let layout1 = layout_tree.children_layout[1].layout;
        assert_eq!(
            layout1.size,
            Size {
                width: 20.0,
                height: 10.0
            }
        );

        assert_eq!(layout1.location, Point { x: 30.0, y: 0.0 });
    }

    #[test]
    fn layout2() {
        let mut control = FlexBox::<()>::new();
        control.vertical();

        let mut btn1 = Button::<()>::new("Hello");
        btn1.set_size(Some(100.0), Some(20.0));

        control.add_child(boxed::Box::new(btn1));

        let mut btn2 = Button::<()>::new("world");
        btn2.set_size(Some(20.0), Some(10.0));

        let mut btn3 = Button::<()>::new("world");
        btn3.set_size(Some(20.0), Some(10.0));

        let mut hrow = FlexBox::<()>::new();
        hrow.horizontal();

        hrow.add_child(boxed::Box::new(btn2));
        hrow.add_child(boxed::Box::new(btn3));

        control.add_child(boxed::Box::new(hrow));

        let layout_tree = compute_layout(
            &mut control,
            Size {
                width: Number::Defined(100.0),
                height: Number::Defined(100.0),
            },
        );

        println!("{:#?}", layout_tree);

        let layout_btn2 =
            layout_tree.children_layout[1].children_layout[1].layout;
        assert_eq!(layout_btn2.location, Point { x: 20.0, y: 20.0 });
    }
}
