use crate::Event;
use crate::{buffer::Buffer, Cmd};
pub use button::Button;
pub use checkbox::Checkbox;
use expanse::geometry::Size;
use expanse::number::Number;
use expanse::result::Layout;
use expanse::{
    node::{Node, Stretch},
    style::Style,
};
pub use flex_box::FlexBox;
pub use group_box::GroupBox;
pub use image_control::Image;
pub use link::Link;
pub use list_box::ListBox;
pub use radio::Radio;
pub use slider::Slider;
use std::{any::Any, fmt};
pub use tab_box::TabBox;
pub use text_area::TextArea;
pub use text_input::TextInput;
pub use text_label::TextLabel;

mod button;
mod checkbox;
mod flex_box;
mod group_box;
mod image_control;
mod link;
mod list_box;
mod radio;
mod slider;
mod tab_box;
mod text_area;
mod text_input;
mod text_label;

/// All widgets must implement the Widget trait
pub trait Widget<MSG>
where
    Self: fmt::Debug,
{
    /// return the style of this widget
    fn style(&self) -> Style;

    /// return the layout of thiswidget
    fn layout(&self) -> Option<&Layout>;

    /// return the offset of the parent,
    /// this before any of it's children to be drawn
    fn get_offset(&self) -> (f32, f32) {
        (0.0, 0.0)
    }

    fn set_layout(&mut self, layout: Layout);

    /// add a child to this widget
    /// returns true if it can accept a child, false otherwise
    fn add_child(&mut self, _child: Box<dyn Widget<MSG>>) -> bool {
        false
    }

    /// get a referemce tp the children of this widget
    fn children(&self) -> Option<&[Box<dyn Widget<MSG>>]> {
        None
    }

    /// get a mutable reference to the children of this widget
    fn children_mut(&mut self) -> Option<&mut [Box<dyn Widget<MSG>>]> {
        None
    }

    /// return a mutable reference to a child at index location
    fn child_mut(
        &mut self,
        _index: usize,
    ) -> Option<&mut Box<dyn Widget<MSG>>> {
        None
    }

    /// this is called in the render loop in the renderer where the widget
    /// writes into the buffer. The result will then be written into the
    /// stdout terminal.
    fn draw(&self, but: &mut Buffer) -> Vec<Cmd>;

    /// build a node with styles from this widget and its children
    /// The Layout tree is then calculated see `layout::compute_layout`
    fn style_node(&self, stretch: &mut Stretch) -> Option<Node> {
        let children_styles = if let Some(children) = self.children() {
            children
                .iter()
                .filter_map(|c| c.style_node(stretch))
                .collect()
        } else {
            vec![]
        };
        stretch.new_node(self.style(), &children_styles).ok()
    }

    /// set the widget as focused
    fn set_focused(&mut self, _focused: bool) {}

    /// get an Any reference
    fn as_any(&self) -> &dyn Any;

    /// get an Any mutable reference for casting purposed
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// set the size of the widget
    fn set_size(&mut self, width: Option<f32>, height: Option<f32>);

    /// get a mutable reference of this widget
    fn as_mut(&mut self) -> Option<&mut Self>
    where
        Self: Sized + 'static,
    {
        self.as_any_mut().downcast_mut::<Self>()
    }

    /// this process the event and all callbacks attached to the widgets will be dispatched.
    fn process_event(&mut self, _event: Event) -> Vec<MSG> {
        vec![]
    }

    ///  take the children at this index location
    fn take_child(&mut self, _index: usize) -> Option<Box<dyn Widget<MSG>>> {
        None
    }

    /// set the id of this widget
    fn set_id(&mut self, id: &str);

    /// get the id of this widget
    fn get_id(&self) -> &Option<String>;

    fn build_stretch_node_recursive(
        &self,
        stretch: &mut Stretch,
    ) -> Option<expanse::node::Node> {
        let children_styles = if let Some(children) = self.children() {
            children
                .iter()
                .filter_map(|c| c.build_stretch_node_recursive(stretch))
                .collect()
        } else {
            vec![]
        };
        let node_style = self.style();
        stretch.new_node(node_style, &children_styles).ok()
    }

    fn set_node_layout_from_stretch_node(
        &mut self,
        stretch_node: expanse::node::Node,
        stretch: &Stretch,
        parent_loc: (f32, f32),
        parent_offset: (f32, f32),
    ) {
        let mut layout =
            *stretch.layout(stretch_node).expect("must have layout");

        let (parent_loc_x, parent_loc_y) = parent_loc;
        let (parent_offset_x, parent_offset_y) = parent_offset;
        let (child_offset_x, child_offset_y) = self.get_offset();

        layout.location.x += parent_loc_x + parent_offset_x;
        layout.location.y += parent_loc_y + parent_offset_y;

        layout.size.width -= parent_offset_x;
        layout.size.height -= parent_offset_y;

        let stretch_node_children: Vec<expanse::node::Node> =
            stretch.children(stretch_node).expect("must get children");

        let widget_children = self.children_mut().unwrap_or(&mut []);

        stretch_node_children
            .into_iter()
            .zip(widget_children.iter_mut())
            .for_each(|(stretch_node_child, widget_child)| {
                widget_child.set_node_layout_from_stretch_node(
                    stretch_node_child,
                    stretch,
                    (layout.location.x, layout.location.y),
                    (child_offset_x, child_offset_y),
                )
            });

        self.set_layout(layout);
    }

    fn node_hit_at(
        &self,
        x: f32,
        y: f32,
        cur_node_idx: &mut usize,
    ) -> Vec<usize> {
        let layout = self.layout().expect("must have a layout");
        let loc = layout.location;
        let width = layout.size.width;
        let height = layout.size.height;

        let mut hits = vec![];

        if x >= loc.x && x < loc.x + width && y >= loc.y && y < loc.y + height {
            hits.push(*cur_node_idx);
        }
        if let Some(children) = self.children() {
            for child in children.iter() {
                *cur_node_idx += 1;
                hits.extend(child.node_hit_at(x, y, cur_node_idx));
            }
        }

        hits
    }

    /// calculate the layout of the nodes utilizing the styles set on each of the widget
    /// and its children widget styles
    fn compute_node_layout(&mut self, parent_size: Size<Number>) {
        let mut stretch = Stretch::new();
        let stretch_node = self
            .build_stretch_node_recursive(&mut stretch)
            .expect("must have built a style node");
        stretch
            .compute_layout(stretch_node, parent_size)
            .expect("must compute the layout");
        self.set_node_layout_from_stretch_node(
            stretch_node,
            &stretch,
            (0.0, 0.0),
            (0.0, 0.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use expanse::{geometry::*, number::Number};
    use std::boxed;

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

        control.compute_node_layout(Size {
            width: Number::Defined(100.0),
            height: Number::Defined(100.0),
        });

        let layout1 = control.children().expect("must have children")[1]
            .layout()
            .expect("must have layout");
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

        control.compute_node_layout(Size {
            width: Number::Defined(100.0),
            height: Number::Defined(100.0),
        });

        let layout_btn2 = control.children().expect("must have children")[1]
            .children()
            .expect("must have grand children")[1]
            .layout()
            .expect("must have a layout");
        assert_eq!(layout_btn2.location, Point { x: 20.0, y: 17.0 });
    }
}
