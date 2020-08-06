use crate::Widget;
use mt_dom::attr;
use stretch::geometry::Size;
use stretch::number::Number;
use stretch::Stretch;

/// calculate the layout of the nodes utilizing the styles set on each of the widget
/// and its children widget styles
pub(crate) fn compute_node_layout<MSG>(
    widget_node: &mut dyn Widget<MSG>,
    parent_size: Size<Number>,
) {
    let mut stretch = Stretch::new();
    let stretch_node = build_stretch_node_recursive(&mut stretch, widget_node)
        .expect("must have built a style node");
    stretch
        .compute_layout(stretch_node, parent_size)
        .expect("must compute the layout");
    set_node_layout_from_stretch_node(
        widget_node,
        stretch_node,
        &stretch,
        (0.0, 0.0),
        (0.0, 0.0),
    )
}

pub(crate) fn node_hit_at<MSG>(
    node: &dyn Widget<MSG>,
    x: f32,
    y: f32,
    cur_node_idx: &mut usize,
) -> Vec<usize> {
    let layout = node.layout().expect("must have a layout");
    let loc = layout.location;
    let width = layout.size.width;
    let height = layout.size.height;

    let mut hits = vec![];

    if x >= loc.x && x < loc.x + width && y >= loc.y && y < loc.y + height {
        hits.push(*cur_node_idx);
    }
    if let Some(children) = node.children() {
        for child in children.iter() {
            *cur_node_idx += 1;
            hits.extend(node_hit_at(child.as_ref(), x, y, cur_node_idx));
        }
    }

    hits
}

fn build_stretch_node_recursive<MSG>(
    stretch: &mut Stretch,
    widget_node: &dyn Widget<MSG>,
) -> Option<stretch::node::Node> {
    let children_styles = if let Some(children) = widget_node.children() {
        children
            .iter()
            .filter_map(|c| build_stretch_node_recursive(stretch, c.as_ref()))
            .collect()
    } else {
        vec![]
    };
    let node_style = widget_node.style();
    stretch.new_node(node_style, &children_styles).ok()
}

fn set_node_layout_from_stretch_node<MSG>(
    widget_node: &mut dyn Widget<MSG>,
    stretch_node: stretch::node::Node,
    stretch: &Stretch,
    parent_loc: (f32, f32),
    parent_offset: (f32, f32),
) {
    let mut layout = *stretch.layout(stretch_node).expect("must have layout");

    let (parent_loc_x, parent_loc_y) = parent_loc;
    let (parent_offset_x, parent_offset_y) = parent_offset;
    let (child_offset_x, child_offset_y) = widget_node.get_offset();

    layout.location.x += parent_loc_x + parent_offset_x;
    layout.location.y += parent_loc_y + parent_offset_y;

    layout.size.width -= parent_offset_x;
    layout.size.height -= parent_offset_y;

    let stretch_node_children: Vec<stretch::node::Node> =
        stretch.children(stretch_node).expect("must get children");

    let widget_children = widget_node.children_mut().unwrap_or(&mut []);

    stretch_node_children
        .into_iter()
        .zip(widget_children.iter_mut())
        .for_each(|(stretch_node_child, widget_child)| {
            set_node_layout_from_stretch_node(
                widget_child.as_mut(),
                stretch_node_child,
                stretch,
                (layout.location.x, layout.location.y),
                (child_offset_x, child_offset_y),
            )
        });

    widget_node.set_layout(layout);
}
