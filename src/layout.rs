use crate::Widget;
use stretch::{
    geometry::{
        Point,
        Size,
    },
    node::{
        Node,
        Stretch,
    },
    number::Number,
    result::Layout,
};

/// Contains the layout information of all the controls in the tree
/// This is needed for optimization purposes since recomputing layout is an expensive operation,
/// therefore can not be executed every draw call
#[derive(Debug)]
pub struct LayoutTree {
    pub layout: Layout,
    pub children_layout: Vec<LayoutTree>,
}

impl LayoutTree {
    /// get the NodeIdx in of this layout tree that hits the location
    /// Deep search first
    ///
    /// The last added element is the deepest child that is hit.
    fn at_location(&self, x: f32, y: f32, cur_index: &mut usize) -> Vec<usize> {
        let mut hits: Vec<usize> = vec![];
        let loc = self.layout.location;
        let width = self.layout.size.width;
        let height = self.layout.size.height;
        if x >= loc.x && x <= loc.x + width && y >= loc.y && y <= loc.y + height
        {
            hits.push(*cur_index);
        }
        let child_hits: Vec<usize> = self
            .children_layout
            .iter()
            .flat_map(|cl| {
                *cur_index += 1;
                cl.at_location(x, y, cur_index)
            })
            .collect();

        hits.extend(child_hits);
        hits
    }

    pub fn hit(&self, x: f32, y: f32) -> Vec<usize> {
        self.at_location(x, y, &mut 0)
    }
}

//TODO: keep track of the focused element,
// area and position of the layout also determines
// if the element is hit with a click

/// Get the widget with the node_idx by traversing to through the root_widget specified
pub fn find_widget<MSG>(
    root_widget: &dyn Widget<MSG>,
    node_idx: usize,
) -> Option<&dyn Widget<MSG>> {
    find_node(root_widget, node_idx, &mut 0)
}

pub fn find_widget_mut<MSG>(
    root_widget: &mut dyn Widget<MSG>,
    node_idx: usize,
) -> Option<&mut dyn Widget<MSG>> {
    find_node_mut(root_widget, node_idx, &mut 0)
}

/// return the widget that is hit at this location
/// base on the layout tree
pub fn widget_hit_at<'a, MSG>(
    root_widget: &'a dyn Widget<MSG>,
    layout_tree: &LayoutTree,
    x: f32,
    y: f32,
) -> Option<&'a dyn Widget<MSG>> {
    if let Some(hit) = layout_tree.hit(x, y).pop() {
        find_widget(root_widget, hit)
    } else {
        None
    }
}

pub fn widget_node_idx_at<'a>(
    layout_tree: &LayoutTree,
    x: f32,
    y: f32,
) -> Option<usize> {
    layout_tree.hit(x, y).pop()
}

/// Traverse the node tree until the node_idx is found
fn find_node<'a, MSG>(
    node: &'a dyn Widget<MSG>,
    node_idx: usize,
    cur_index: &mut usize,
) -> Option<&'a dyn Widget<MSG>> {
    if let Some(children) = node.children() {
        children.iter().find_map(|child| {
            *cur_index += 1;
            find_node(child.as_ref(), node_idx, cur_index)
        })
    } else if node_idx == *cur_index {
        return Some(node);
    } else {
        None
    }
}

fn find_node_mut<'a, MSG>(
    node: &'a mut dyn Widget<MSG>,
    node_idx: usize,
    cur_index: &mut usize,
) -> Option<&'a mut dyn Widget<MSG>> {
    if node_idx == *cur_index {
        return Some(node);
    } else if let Some(children) = node.children_mut() {
        children.iter_mut().find_map(|child| {
            *cur_index += 1;
            find_node_mut(child.as_mut(), node_idx, cur_index)
        })
    } else {
        None
    }
}

pub fn find_layout<'a>(
    node: &'a LayoutTree,
    node_idx: usize,
) -> Option<&'a LayoutTree> {
    find_layout_tree(node, node_idx, &mut 0)
}

fn find_layout_tree<'a>(
    node: &'a LayoutTree,
    node_idx: usize,
    cur_index: &mut usize,
) -> Option<&'a LayoutTree> {
    if node_idx == *cur_index {
        return Some(node);
    } else {
        node.children_layout.iter().find_map(|child| {
            *cur_index += 1;
            find_layout_tree(child, node_idx, cur_index)
        })
    }
}

pub fn set_focused_node<'a, MSG>(
    node: &'a mut dyn Widget<MSG>,
    node_idx: usize,
) {
    set_focused_widget(node, node_idx, &mut 0)
}

/// Set the node at node_idx as focused, while the rest
/// should be set to false
fn set_focused_widget<'a, MSG>(
    node: &'a mut dyn Widget<MSG>,
    node_idx: usize,
    cur_index: &mut usize,
) {
    if node_idx == *cur_index {
        node.set_focused(true);
    } else if let Some(children) = node.children_mut() {
        children.iter_mut().for_each(|child| {
            *cur_index += 1;
            set_focused_widget(child.as_mut(), node_idx, cur_index)
        })
    } else {
        node.set_focused(false);
    }
}

/// Compute a flex layout of the node and it's children
pub fn compute_layout<MSG>(
    control: &mut dyn Widget<MSG>,
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

/// retrieve the layout for each of the invidual unit in the node.
/// The locatio is in absolute position by adding the parent position to the child position
/// in order to easily draw the widgets independently
fn derive_layout_tree(node: Node, stretch: &Stretch) -> LayoutTree {
    let layout = *stretch.layout(node).expect("must have layout");
    let children: Vec<Node> =
        stretch.children(node).expect("must get children");
    let children_layout: Vec<LayoutTree> = children
        .into_iter()
        .map(|child| {
            let mut child_tree = derive_layout_tree(child, stretch);
            let orig_pos = child_tree.layout.location;
            child_tree.layout.location = Point {
                x: orig_pos.x + layout.location.x,
                y: orig_pos.y + layout.location.y,
            };
            child_tree
        })
        .collect();
    LayoutTree {
        layout,
        children_layout,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn layout() {
        let mut control = FlexBox::<()>::new();
        control.vertical();
        let mut btn1 = Button::<()>::new("Hello");
        btn1.set_size(Some(30.0), Some(34.0));
        let btn1_clone = btn1.clone();

        control.add_child(Box::new(btn1));

        let mut btn2 = Button::<()>::new("world");
        btn2.set_size(Some(20.0), Some(10.0));
        let btn2_clone = btn2.clone();
        control.add_child(Box::new(btn2));

        let layout_tree = compute_layout(
            &mut control,
            Size {
                width: Number::Defined(100.0),
                height: Number::Defined(100.0),
            },
        );
        println!("layout: {:#?}", layout_tree);

        let layout1 = layout_tree.children_layout[0].layout;
        assert_eq!(
            layout1.size,
            Size {
                width: 30.0,
                height: 34.0
            }
        );
        assert_eq!(layout1.location, Point { x: 00.0, y: 0.0 });
        let mut hit1 = layout_tree.hit(1.0, 1.0);
        assert_eq!(hit1.len(), 2);
        println!("hit1: {:?}", hit1);
        assert_eq!(hit1.pop(), Some(1));
        let trace_btn1: &Button<()> = find_widget(&control, 1)
            .expect("must return a widget")
            .as_any()
            .downcast_ref::<Button<()>>()
            .expect("must be button");
        assert_eq!(trace_btn1, &btn1_clone);
        println!("trace btn1: {:?}", trace_btn1);

        let widget_hit1 = widget_hit_at(&control, &layout_tree, 1.0, 1.0)
            .expect("must hit something")
            .as_any()
            .downcast_ref::<Button<()>>()
            .expect("must be button");

        assert_eq!(widget_hit1, &btn1_clone);

        let layout2 = layout_tree.children_layout[1].layout;
        assert_eq!(
            layout2.size,
            Size {
                width: 20.0,
                height: 10.0
            }
        );

        assert_eq!(layout2.location, Point { x: 00.0, y: 34.0 });
        assert_eq!(
            layout2.size,
            Size {
                width: 20.0,
                height: 10.0
            }
        );
        let mut hit2 = layout_tree.hit(1.0, 35.0);
        assert_eq!(hit2.len(), 2);
        assert_eq!(hit2.pop(), Some(2));

        let trace_btn2 = find_widget(&control, 2)
            .expect("must return a widget")
            .as_any()
            .downcast_ref::<Button<()>>()
            .expect("must be a button");
        assert_eq!(trace_btn2, &btn2_clone);

        let widget_hit2 = widget_hit_at(&control, &layout_tree, 1.0, 35.0)
            .expect("must hit something")
            .as_any()
            .downcast_ref::<Button<()>>()
            .expect("must be button");

        assert_eq!(widget_hit2, &btn2_clone);
    }
}
