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
    use stretch::style::*;

    #[test]
    fn layout() {
        let bx = Box {
            children: vec![
                Control::Button(Button::new("Hello")),
                Control::Button(Button {
                    label: "world".to_string(),
                    style: Style {
                        size: Size {
                            width: Dimension::Points(160.0),
                            height: Dimension::Points(140.0),
                        },
                        max_size: Size {
                            width: Dimension::Points(150.0),
                            height: Dimension::Points(140.0),
                        },
                        ..Default::default()
                    },
                }),
            ],
            style: Style {
                max_size: Size {
                    width: Dimension::Points(100.0),
                    height: Dimension::Points(100.0),
                },
                overflow: Overflow::Hidden,
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                ..Default::default()
            },
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
        for child in children {
            println!(
                "child layout: {:?}",
                stretch.layout(child).expect("must have a layout")
            );
        }
        panic!();
    }
}
