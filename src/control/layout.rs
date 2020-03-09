use super::Control;
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
        let child_hits: Vec<usize> = self
            .children_layout
            .iter()
            .flat_map(|cl| {
                *cur_index += 1;
                cl.at_location(x, y, cur_index)
            })
            .collect();

        let loc = self.layout.location;
        let width = self.layout.size.width;
        let height = self.layout.size.height;
        if x >= loc.x && x <= loc.x + width && y >= loc.y && y <= loc.y + height
        {
            hits.push(*cur_index);
        }
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
        let mut bx = Box::new();
        bx.vertical();
        let mut btn1 = Button::new("Hello");
        btn1.set_size(Some(30.0), Some(34.0));

        bx.add_child(Into::<Control>::into(btn1));

        let mut btn2 = Button::new("world");
        btn2.set_size(Some(20.0), Some(10.0));
        bx.add_child(Into::<Control>::into(btn2));

        let mut control = Control::Box(bx);
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
    }
}
