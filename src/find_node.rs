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

        let got_btn1 =
            find_widget_by_id(&control, "btn1").expect("must return a widget");

        println!("btn1: {:?}", got_btn1);
        assert_eq!(*got_btn1.get_id(), Some("btn1".to_string()));

        let got_btn2 =
            find_widget_by_id(&control, "btn2").expect("must return a widget");

        println!("btn2: {:?}", got_btn2);
        assert_eq!(*got_btn2.get_id(), Some("btn2".to_string()));
    }
}
