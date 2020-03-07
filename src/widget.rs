use crate::buffer::Buffer;
use stretch::{
    geometry::Size,
    node::{
        Node,
        Stretch,
    },
    number::Number,
    style::Style,
};

pub enum Control {
    Button(Button),
    Box(Box),
}

pub struct Button {
    pub label: String,
    style: Style,
}

pub struct Box {
    children: Vec<Control>,
    style: Style,
}

impl Button {
    fn new<S>(label: S) -> Self
    where
        S: ToString,
    {
        Button {
            label: label.to_string(),
            style: Style::default(),
        }
    }

    fn draw(&self, buf: &mut Buffer) {}
}

impl Control {
    fn get_style(&self) -> Style {
        match self {
            Control::Button(btn) => btn.style,
            Control::Box(box_) => box_.style,
        }
    }

    fn children(&self) -> Option<&Vec<Control>> {
        match self {
            Control::Box(box_) => Some(&box_.children),
            Control::Button(_btn) => None,
        }
    }

    fn style_node(&self, stretch: &mut Stretch) -> Option<Node> {
        let children_styles = if let Some(children) = self.children() {
            children
                .into_iter()
                .filter_map(|c| c.style_node(stretch))
                .inspect(|n| println!("node: {:?}", n))
                .collect()
        } else {
            vec![]
        };
        stretch.new_node(self.get_style(), children_styles).ok()
    }
}

/// Compute a flex layout of the node and it's children
pub fn compute_layout(
    control: &Control,
    parent_size: Size<Number>,
) -> (Option<Node>, Stretch) {
    let mut stretch = Stretch::new();
    let node = if let Some(node) = control.style_node(&mut stretch) {
        stretch.compute_layout(node, parent_size);
        Some(node)
    } else {
        None
    };
    (node, stretch)
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
