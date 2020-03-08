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
use crossterm::style::Color;
pub use image_widget::Image;
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

mod image_widget;
mod layout;

pub enum Control {
    Button(Button),
    Image(Image),
    Box(Box),
}

#[derive(Default)]
pub struct Button {
    pub label: String,
    style: Style,
    pub is_rounded: bool,
}

#[derive(Default)]
pub struct Box {
    children: Vec<Control>,
    style: Style,
}

impl Button {
    pub fn new<S>(label: S) -> Self
    where
        S: ToString,
    {
        Button {
            label: label.to_string(),
            ..Default::default()
        }
    }

    pub fn set_label<S: ToString>(&mut self, label: S) {
        self.label = label.to_string();
    }

    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn set_rounded(&mut self, rounded: bool) {
        self.is_rounded = rounded;
    }

    /// draw this button to the buffer, with the given computed layout
    pub fn draw(&self, buf: &mut Buffer, layout_tree: LayoutTree) {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let width = layout.size.width.round() as usize;
        let height = layout.size.height.round() as usize;
        let padding_start = match self.style.padding.start {
            Dimension::Points(p) => p.round() as usize,
            _ => 0,
        };
        for i in 0..width {
            buf.set_symbol(loc_x + i, loc_y + 1, line::HORIZONTAL);
            buf.set_symbol(loc_x + i, loc_y + height, line::HORIZONTAL);
        }
        for j in 0..height {
            buf.set_symbol(loc_x, loc_y + 1 + j, line::VERTICAL);
            buf.set_symbol(loc_x + width, loc_y + 1 + j, line::VERTICAL);
        }
        for (t, ch) in self.label.chars().enumerate() {
            buf.set_symbol(loc_x + 1 + padding_start + t, loc_y + 2, ch);
        }

        let top_left = if self.is_rounded {
            rounded::TOP_LEFT
        } else {
            line::TOP_LEFT
        };
        let top_right = if self.is_rounded {
            rounded::TOP_RIGHT
        } else {
            line::TOP_RIGHT
        };
        let bottom_left = if self.is_rounded {
            rounded::BOTTOM_LEFT
        } else {
            line::BOTTOM_LEFT
        };
        let bottom_right = if self.is_rounded {
            rounded::BOTTOM_RIGHT
        } else {
            line::BOTTOM_RIGHT
        };
        buf.set_symbol(loc_x, loc_y + 1, top_left);
        buf.set_symbol(loc_x, loc_y + height, bottom_left);
        buf.set_symbol(loc_x + width, loc_y + 1, top_right);
        buf.set_symbol(loc_x + width, loc_y + height, bottom_right);
    }
}

impl From<Button> for Control {
    fn from(btn: Button) -> Self {
        Control::Button(btn)
    }
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
