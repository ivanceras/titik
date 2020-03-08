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

#[derive(Debug)]
pub struct LayoutTree {
    pub layout: Layout,
    pub children_layout: Vec<LayoutTree>,
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
