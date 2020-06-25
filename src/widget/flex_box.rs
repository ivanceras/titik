use crate::{
    buffer::Buffer,
    widget::Flex,
    Cmd,
    LayoutTree,
    Widget,
};
use std::{
    any::Any,
    fmt,
};
use stretch::style::{
    FlexDirection,
    Style,
};

/// a flex box
#[derive(Default, Debug)]
pub struct FlexBox<MSG> {
    children: Vec<Box<dyn Widget<MSG>>>,
    width: Option<f32>,
    height: Option<f32>,
    flex_direction: FlexDirection,
    scroll_top: f32,
    id: Option<String>,
    has_border: bool,
    is_rounded_border: bool,
    is_thick_border: bool,
    /// take as much height as possible
    is_expand_height: bool,
    /// take as much width as possible
    is_expand_width: bool,
}

impl<MSG> FlexBox<MSG> {
    ///create a new flexbox
    pub fn new() -> Self {
        FlexBox {
            width: None,
            height: None,
            children: vec![],
            flex_direction: FlexDirection::Row,
            scroll_top: 0.0,
            id: None,
            has_border: false,
            is_rounded_border: false,
            is_thick_border: false,
            is_expand_height: false,
            is_expand_width: false,
        }
    }

    /// remove all children of this flex_box
    pub fn clear_children(&mut self) {
        self.children = vec![];
    }

    /// set to vertical column direction
    pub fn vertical(&mut self) {
        self.flex_direction = FlexDirection::Column;
    }

    /// set to horizontal row direction
    pub fn horizontal(&mut self) {
        self.flex_direction = FlexDirection::Row;
    }

    /// scroll the flexbox
    pub fn set_scroll_top(&mut self, scroll_top: f32) {
        self.scroll_top = scroll_top;
    }

    /// set if to expand the width or not
    pub fn set_expand_width(&mut self, is_expand_width: bool) {
        self.is_expand_width = is_expand_width;
    }

    /// set whether to expand the height or not
    pub fn set_expand_height(&mut self, is_expand_height: bool) {
        self.is_expand_height = is_expand_height;
    }
}

impl<MSG> Widget<MSG> for FlexBox<MSG>
where
    MSG: fmt::Debug + 'static,
{
    fn style(&self) -> Style {
        self.flex_style()
    }

    fn draw(&mut self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        self.draw_flex(buf, layout_tree)
    }

    fn add_child(&mut self, child: Box<dyn Widget<MSG>>) -> bool {
        self.children.push(child);
        true
    }

    fn children(&self) -> Option<&[Box<dyn Widget<MSG>>]> {
        Some(&self.children)
    }

    fn children_mut(&mut self) -> Option<&mut [Box<dyn Widget<MSG>>]> {
        Some(&mut self.children)
    }

    // TODO: use remove_item when it will be stabilized
    fn take_child(&mut self, index: usize) -> Option<Box<dyn Widget<MSG>>> {
        Some(self.children.remove(index))
    }

    fn child_mut<'a>(
        &'a mut self,
        index: usize,
    ) -> Option<&'a mut Box<dyn Widget<MSG>>> {
        self.children.get_mut(index)
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

impl<MSG> Flex<MSG> for FlexBox<MSG>
where
    MSG: fmt::Debug + 'static,
{
    fn has_border(&self) -> bool {
        self.has_border
    }

    fn is_rounded_border(&self) -> bool {
        self.is_rounded_border
    }

    fn is_thick_border(&self) -> bool {
        self.is_thick_border
    }

    fn is_expand_width(&self) -> bool {
        self.is_expand_width
    }

    fn is_expand_height(&self) -> bool {
        self.is_expand_height
    }

    fn flex_direction(&self) -> FlexDirection {
        self.flex_direction
    }

    fn width(&self) -> Option<f32> {
        self.width
    }

    fn height(&self) -> Option<f32> {
        self.height
    }

    fn scroll_top(&self) -> f32 {
        self.scroll_top
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_mut() {
        let mut fb = FlexBox::<()>::new();
        let btn1 = Button::new("Btn1");
        println!("btn1: {:?}", btn1);
        fb.add_child(Box::new(btn1));

        let btn1_mut = fb.child_mut(0).expect("must have a child 0");
        let btn1_cast = btn1_mut
            .as_any_mut()
            .downcast_mut::<Button<()>>()
            .expect("must be a button");
        btn1_cast.set_size(Some(20.0), Some(10.0));
        println!("btn1_cast: {:?}", btn1_cast);
        assert_eq!(Some(20.0), btn1_cast.width);
        assert_eq!(Some(10.0), btn1_cast.height);
    }

    #[test]
    fn test_children_mut() {
        let mut fb = FlexBox::<()>::new();
        let btn1 = Button::new("Btn1");
        println!("btn1: {:?}", btn1);
        fb.add_child(Box::new(btn1));

        let children = fb.children_mut().expect("must have children");
        let btn0 = children[0]
            .as_any_mut()
            .downcast_mut::<Button<()>>()
            .expect("must be a button");

        btn0.set_size(Some(40.0), Some(100.0));
        assert_eq!(Some(40.0), btn0.width);
        assert_eq!(Some(100.0), btn0.height);
    }
}
