use crate::Widget;

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

/// Get the widget with the node_idx by traversing to through the root_widget specified
pub fn find_widget<MSG>(
    root_widget: &dyn Widget<MSG>,
    node_idx: usize,
) -> Option<&dyn Widget<MSG>> {
    find_node(root_widget, node_idx, &mut 0)
}

/// returns a mutable reference to the widget from the root_widget tree matching the supplied node
/// index
pub fn find_widget_mut<MSG>(
    root_widget: &mut dyn Widget<MSG>,
    node_idx: usize,
) -> Option<&mut dyn Widget<MSG>> {
    find_node_mut(root_widget, node_idx, &mut 0)
}

/// returns a reference to the widget from the root widget tree matching the supplied id
pub fn find_widget_by_id<'a, MSG>(
    root_widget: &'a dyn Widget<MSG>,
    id: &str,
) -> Option<&'a dyn Widget<MSG>> {
    let matched_root = if let Some(node_id) = root_widget.get_id() {
        if node_id == id {
            Some(root_widget)
        } else {
            None
        }
    } else {
        None
    };
    if matched_root.is_some() {
        return matched_root;
    } else if let Some(children) = root_widget.children() {
        children
            .iter()
            .find_map(|child| find_widget_by_id(child.as_ref(), id))
    } else {
        None
    }
}

/// returns a mutable reference to the widget from the root widget tree matching the supplied id
pub fn find_widget_by_id_mut<'a, MSG>(
    root_widget: &'a mut dyn Widget<MSG>,
    id: &str,
) -> Option<&'a mut dyn Widget<MSG>> {
    let matched_root = if let Some(node_id) = root_widget.get_id() {
        if node_id == id {
            true
        } else {
            false
        }
    } else {
        false
    };
    if matched_root {
        return Some(root_widget);
    } else if let Some(children) = root_widget.children_mut() {
        children
            .iter_mut()
            .find_map(|child| find_widget_by_id_mut(child.as_mut(), id))
    } else {
        None
    }
}

/// set the node with idx to be in focused
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

/// remove the widget at this node_idx
pub fn remove_widget<MSG>(
    root_widget: &mut dyn Widget<MSG>,
    node_idx: usize,
) -> bool {
    remove_widget_recursive(root_widget, node_idx, &mut 0)
}

fn remove_widget_recursive<MSG>(
    widget: &mut dyn Widget<MSG>,
    node_idx: usize,
    cur_node_idx: &mut usize,
) -> bool {
    if let Some(children) = widget.children_mut() {
        let mut this_cur_node_idx = *cur_node_idx;
        let mut to_be_remove = None;
        // look ahead for remove
        for (idx, child) in children.iter().enumerate() {
            this_cur_node_idx += 1;
            if node_idx == this_cur_node_idx {
                to_be_remove = Some(idx);
            } else {
                increment_node_idx_to_descendant_count(
                    child.as_ref(),
                    &mut this_cur_node_idx,
                );
            }
        }

        if let Some(remove_idx) = to_be_remove {
            widget
                .take_child(remove_idx)
                .expect("must be able to remove child");
            true
        } else {
            for child in children {
                *cur_node_idx += 1;
                if remove_widget_recursive(
                    child.as_mut(),
                    node_idx,
                    cur_node_idx,
                ) {
                    return true;
                }
            }
            false
        }
    } else {
        false
    }
}

pub fn increment_node_idx_to_descendant_count<MSG>(
    node: &dyn Widget<MSG>,
    cur_node_idx: &mut usize,
) {
    if let Some(children) = node.children() {
        for child in children {
            *cur_node_idx += 1;
            increment_node_idx_to_descendant_count(
                child.as_ref(),
                cur_node_idx,
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn find_by_id() {
        let mut control = FlexBox::<()>::new();
        control.vertical();
        let mut btn1 = Button::<()>::new("Hello");
        btn1.set_size(Some(30.0), Some(34.0));
        btn1.set_id("btn1");
        let btn1_clone = btn1.clone();
        control.add_child(Box::new(btn1));

        let mut btn2 = Button::<()>::new("world");
        btn2.set_id("btn2");
        btn2.set_size(Some(20.0), Some(10.0));
        let btn2_clone = btn2.clone();
        control.add_child(Box::new(btn2));

        let got_btn1 = find_widget_by_id(&control, "btn1")
            .expect("must return a widget")
            .as_any()
            .downcast_ref::<Button<()>>()
            .expect("must be button");
        println!("btn1: {:?}", got_btn1);
        assert_eq!(*got_btn1.get_id(), Some("btn1".to_string()));
        assert_eq!(got_btn1, &btn1_clone);

        let got_btn2 = find_widget_by_id(&control, "btn2")
            .expect("must return a widget")
            .as_any()
            .downcast_ref::<Button<()>>()
            .expect("must be button");
        println!("btn2: {:?}", got_btn2);
        assert_eq!(*got_btn2.get_id(), Some("btn2".to_string()));
        assert_eq!(got_btn2, &btn2_clone);
    }
}
