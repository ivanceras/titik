use stretch::result::Layout;

pub struct LayoutTree {
    pub layout: Layout,
    pub children_layout: Vec<LayoutTree>,
}
