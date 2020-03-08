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
pub use box_control::Box;
pub use button::Button;
pub use checkbox::Checkbox;
use crossterm::style::Color;
pub use image_control::Image;
pub use layout::{
    compute_layout,
    LayoutTree,
};
pub use radio::Radio;
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

mod box_control;
mod button;
mod checkbox;
mod image_control;
mod layout;
mod radio;

pub enum Control {
    Button(Button),
    Checkbox(Checkbox),
    Radio(Radio),
    Image(Image),
    Box(Box),
}

impl Control {
    fn get_style(&self) -> Style {
        match self {
            Control::Button(widget) => widget.style,
            Control::Checkbox(widget) => widget.style(),
            Control::Radio(widget) => widget.style(),
            Control::Box(widget) => widget.style,
            Control::Image(widget) => widget.style(),
        }
    }

    pub fn add_child<C: Into<Control>>(&mut self, child: C) {
        match self {
            Control::Box(bax) => bax.add_child(child),
            _ => (), //TODO: warning here
        }
    }

    fn children(&self) -> Option<&Vec<Control>> {
        match self {
            Control::Box(bax) => Some(&bax.children),
            _ => None,
        }
    }

    fn children_mut(&mut self) -> Option<&mut Vec<Control>> {
        match *self {
            Control::Box(ref mut bax) => Some(&mut bax.children),
            _ => None,
        }
    }

    fn style_node(&mut self, stretch: &mut Stretch) -> Option<Node> {
        let children_styles = if let Some(children) = self.children_mut() {
            children
                .iter_mut()
                .filter_map(|c| c.style_node(stretch))
                .collect()
        } else {
            vec![]
        };
        stretch.new_node(self.get_style(), children_styles).ok()
    }

    pub fn draw(&self, buffer: &mut Buffer, layout_tree: LayoutTree) {
        match self {
            Control::Button(widget) => widget.draw(buffer, layout_tree),
            Control::Checkbox(widget) => widget.draw(buffer, layout_tree),
            Control::Radio(widget) => widget.draw(buffer, layout_tree),
            Control::Image(widget) => widget.draw(buffer, layout_tree),
            Control::Box(widget) => widget.draw(buffer, layout_tree),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stretch::{
        geometry::*,
        result::*,
        style::*,
    };

    #[test]
    fn layout() {
        let bx = Box {
            style: Style {
                max_size: Size {
                    width: Dimension::Points(100.0),
                    height: Dimension::Points(100.0),
                },
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            children: vec![
                Control::Button(Button {
                    style: Style {
                        size: Size {
                            width: Dimension::Points(30.0),
                            height: Dimension::Points(34.0),
                        },
                        ..Default::default()
                    },
                    label: "Hello".to_string(),
                }),
                Control::Button(Button {
                    style: Style {
                        size: Size {
                            width: Dimension::Points(20.0),
                            height: Dimension::Points(10.0),
                        },
                        ..Default::default()
                    },
                    label: "world".to_string(),
                }),
            ],
        };
        let control = Control::Box(bx);
        let (parent, stretch) = compute_layout(
            &control,
            Size {
                width: Number::Defined(100.0),
                height: Number::Defined(100.0),
            },
        );

        let parent = parent.expect("must have a node");
        println!(
            "parent layout: {:?}",
            stretch.layout(parent).expect("must have a layout")
        );
        let children = stretch.children(parent).expect("must have children");
        for child in children.iter() {
            println!(
                "child layout: {:?}",
                stretch.layout(*child).expect("must have a layout")
            );
        }
        let layout1 = stretch.layout(children[1]).expect("must have layout");
        assert_eq!(
            layout1.size,
            Size {
                width: 20.0,
                height: 10.0
            }
        );

        assert_eq!(layout1.location, Point { x: 30.0, y: 0.0 });
    }
}
