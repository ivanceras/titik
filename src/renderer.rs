use crate::{
    command, compute_layout, find_layout, find_widget_mut, set_focused_node,
    widget_node_idx_at, Buffer, LayoutTree, Widget,
};
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent},
    execute, queue, style,
    style::{Attribute, Attributes, Color, ContentStyle},
    terminal::{self, ClearType},
    Command, Result,
};
use std::cell::RefCell;
use std::io::Write;
use std::marker::PhantomData;
use std::rc::Rc;
use stretch::{geometry::Size, number::Number};

pub trait Dispatch<MSG> {
    fn dispatch(&self, msg: MSG, root_node: &mut Box<dyn Widget<MSG>>);
}

pub fn render<MSG>(
    write: &mut dyn Write,
    program: Option<&dyn Dispatch<MSG>>,
    root_node: Rc<RefCell<Box<Widget<MSG>>>>,
) -> Result<()> {
    let mut focused_widget_idx: Option<usize> = None;
    command::init(write)?;
    command::reset_top(write)?;
    let (width, height) = terminal::size().expect("must get the terminal size");
    {
        root_node
            .borrow_mut()
            .as_mut()
            .set_size(Some((width) as f32), Some(height as f32));
    }
    let layout_tree = compute_layout(
        root_node.borrow_mut().as_mut(),
        Size {
            width: Number::Defined(width as f32),
            height: Number::Defined(height as f32),
        },
    );

    loop {
        let mut buf = Buffer::new(width as usize, height as usize);
        buf.reset();
        {
            let cmds =
                root_node.borrow_mut().as_mut().draw(&mut buf, &layout_tree);
            buf.render(write)?;
            cmds.iter()
                .for_each(|cmd| cmd.execute(write).expect("must execute"));
        }
        write.flush()?;

        if let Ok(event) = event::read() {
            match event {
                Event::Key(key_event) => {
                    // To quite, press any of the following:
                    //  - CTRL-c
                    //  - CTRL-q
                    //  - CTRL-d
                    //  - CTRL-z
                    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                        match key_event.code {
                            KeyCode::Char(c) => match c {
                                'c' | 'q' | 'd' | 'z' => {
                                    break;
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    } else {
                        // send the keypresses to the focused widget
                        if let Some(idx) = focused_widget_idx.as_ref() {
                            let mut root_node = root_node.borrow_mut();
                            let active_widget: Option<&mut dyn Widget<MSG>> =
                                find_widget_mut(root_node.as_mut(), *idx);
                            let focused_layout =
                                find_layout(&layout_tree, *idx)
                                    .expect("must have a layout tree");
                            if let Some(focused_widget) = active_widget {
                                let msgs = focused_widget.process_event(
                                    event,
                                    &focused_layout.layout,
                                );
                                if let Some(program) = program {
                                    for msg in msgs {
                                        eprintln!("dispatching msg");
                                        program.dispatch(msg, &mut *root_node);
                                    }
                                }
                            }
                        }
                    }
                }
                // mouse clicks sets the focused the widget underneath
                Event::Mouse(MouseEvent::Down(_btn, x, y, _modifier)) => {
                    focused_widget_idx =
                        widget_node_idx_at(&layout_tree, x as f32, y as f32);

                    if let Some(idx) = focused_widget_idx.as_ref() {
                        set_focused_node(root_node.borrow_mut().as_mut(), *idx);
                    }
                }
                Event::Resize(width, height) => {
                    compute_layout(
                        root_node.borrow_mut().as_mut(),
                        Size {
                            width: Number::Defined(width as f32),
                            height: Number::Defined(height as f32),
                        },
                    );
                }
                _ => (),
            }
            // any other activities, such as mouse scroll is
            // sent the widget underneath the location, regardless
            // if it focused or not.
            if let Some((x, y)) = extract_location(&event) {
                let mut hits = layout_tree.hit(x as f32, y as f32);
                let hit = hits.pop().expect("process only 1 for now");
                let mut root_node = root_node.borrow_mut();
                let mut hit_widget: Option<&mut dyn Widget<MSG>> =
                    { find_widget_mut(root_node.as_mut(), hit) };

                let focused_layout = find_layout(&layout_tree, hit)
                    .expect("must have a layout tree");

                if let Some(hit_widget) = &mut hit_widget {
                    let msgs =
                        hit_widget.process_event(event, &focused_layout.layout);
                    if let Some(program) = program {
                        for msg in msgs {
                            eprintln!("dispatching msg");
                            program.dispatch(msg, &mut *root_node);
                        }
                    }
                }
            }
        }
    }
    command::finalize(write)?;
    Ok(())
}

/// extract the x and y location of a mouse event
fn extract_location(event: &Event) -> Option<(u16, u16)> {
    match event {
        Event::Mouse(MouseEvent::Down(_btn, x, y, _modifier)) => Some((*x, *y)),
        Event::Mouse(MouseEvent::Up(_btn, x, y, _modifier)) => Some((*x, *y)),
        Event::Mouse(MouseEvent::Drag(_btn, x, y, _modifier)) => Some((*x, *y)),
        Event::Mouse(MouseEvent::ScrollDown(x, y, _modifier)) => Some((*x, *y)),
        Event::Mouse(MouseEvent::ScrollUp(x, y, _modifier)) => Some((*x, *y)),
        Event::Key(_) => None,
        Event::Resize(_, _) => None,
    }
}
