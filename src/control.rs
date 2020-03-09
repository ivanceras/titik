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
pub use text_input::TextInput;

mod box_control;
mod button;
mod checkbox;
mod image_control;
mod layout;
mod radio;
mod text_input;

//TODO: can be converted to trait, and the children can be stored as Vec<Box<Control>>
pub enum Control {
    Button(Button),
    Checkbox(Checkbox),
    TextInput(TextInput),
    Radio(Radio),
    Image(Image),
    Box(Box),
}

impl Control {
    fn get_style(&self) -> Style {
        match self {
            Control::Button(widget) => widget.style(),
            Control::Checkbox(widget) => widget.style(),
            Control::TextInput(widget) => widget.style(),
            Control::Radio(widget) => widget.style(),
            Control::Box(widget) => widget.style(),
            Control::Image(widget) => widget.style(),
        }
    }

    pub fn set_size(&mut self, width: Option<f32>, height: Option<f32>) {
        match self {
            Control::Box(bax) => bax.set_size(width, height),
            _ => (), //TODO: every control will have a style and can be merged/overriden
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

    pub fn draw(&self, buffer: &mut Buffer, layout_tree: &LayoutTree) {
        match self {
            Control::Button(widget) => widget.draw(buffer, layout_tree),
            Control::Checkbox(widget) => widget.draw(buffer, layout_tree),
            Control::TextInput(widget) => widget.draw(buffer, layout_tree),
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
        let mut bx = Box::new();
        bx.horizontal();
        let mut btn = Button::new("Hello");
        btn.set_size(Some(30.0), Some(34.0));

        bx.add_child(Into::<Control>::into(btn));

        let mut btn = Button::new("world");
        btn.set_size(Some(20.0), Some(10.0));
        bx.add_child(Into::<Control>::into(btn));

        let mut control = Control::Box(bx);
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
        let mut bx = Box::new();
        bx.vertical();
        let mut control = Control::Box(bx);

        let mut btn1 = Button::new("Hello");
        btn1.set_size(Some(100.0), Some(20.0));

        control.add_child(Into::<Control>::into(btn1));

        let mut btn2 = Button::new("world");
        btn2.set_size(Some(20.0), Some(10.0));

        let mut btn3 = Button::new("world");
        btn3.set_size(Some(20.0), Some(10.0));

        let mut row = Box::new();
        row.horizontal();

        let mut hrow: Control = row.into();
        hrow.add_child(Into::<Control>::into(btn2));
        hrow.add_child(Into::<Control>::into(btn3));

        control.add_child(hrow);

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
