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
pub use button::Button;
use crossterm::style::Color;
pub use image_control::Image;
pub use layout::LayoutTree;
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

mod button;
mod image_control;
mod layout;

pub enum Control {
    Button(Button),
    Image(Image),
    Box(Box),
}

#[derive(Default)]
pub struct Box {
    children: Vec<Control>,
    style: Style,
}

impl Box {
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn draw(&self, buf: &mut Buffer, layout_tree: LayoutTree) {
        self.children
            .iter()
            .zip(layout_tree.children_layout.into_iter())
            .for_each(|(child, child_layout)| child.draw(buf, child_layout));
    }

    pub fn add_child<C: Into<Control>>(&mut self, child: C) {
        self.children.push(child.into());
    }
}

impl Control {
    fn get_style(&self) -> Style {
        match self {
            Control::Button(btn) => btn.style,
            Control::Box(bax) => bax.style,
            Control::Image(image) => image.style(),
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
            Control::Button(btn) => btn.draw(buffer, layout_tree),
            Control::Image(img) => img.draw(buffer, layout_tree),
            Control::Box(bx) => bx.draw(buffer, layout_tree),
        }
    }
}

/// Compute a flex layout of the node and it's children
pub fn compute_layout(
    control: &mut Control,
    parent_size: Size<Number>,
) -> LayoutTree {
    let mut stretch = Stretch::new();
    let node = control
        .style_node(&mut stretch)
        .expect("must compute style node");
    stretch
        .compute_layout(node, parent_size)
        .expect("must compute layout");

    derive_layout_tree(node, &stretch)
}

fn derive_layout_tree(node: Node, stretch: &Stretch) -> LayoutTree {
    let layout = *stretch.layout(node).expect("must have layout");
    let children: Vec<Node> =
        stretch.children(node).expect("must get children");
    let children_layout: Vec<LayoutTree> = children
        .into_iter()
        .map(|child| derive_layout_tree(child, stretch))
        .collect();
    LayoutTree {
        layout,
        children_layout,
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
