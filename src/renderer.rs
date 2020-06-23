use crate::{
    command,
    compute_layout,
    find_layout,
    find_widget_mut,
    set_focused_node,
    widget_node_idx_at,
    Buffer,
    LayoutTree,
    Widget,
};
pub use crossterm::{
    cursor,
    event::{
        self,
        Event,
        KeyCode,
        KeyEvent,
        KeyModifiers,
        MouseEvent,
    },
    execute,
    queue,
    style,
    style::{
        Attribute,
        Attributes,
        Color,
        ContentStyle,
    },
    terminal::{
        self,
        ClearType,
    },
    Command,
    Result,
};
use std::{
    io::Write,
    marker::PhantomData,
};
use stretch::{
    geometry::Size,
    number::Number,
};

pub trait Dispatch<MSG> {
    fn dispatch(&self, msg: MSG, root_node: &mut dyn Widget<MSG>);
}

pub struct Renderer<'a, MSG> {
    write: &'a mut dyn Write,
    program: Option<&'a dyn Dispatch<MSG>>,
    root_node: &'a mut dyn Widget<MSG>,
    layout_tree: LayoutTree,
    terminal_size: (u16, u16),
    focused_widget_idx: Option<usize>,
}

impl<'a, MSG> Renderer<'a, MSG> {
    pub fn new(
        write: &'a mut dyn Write,
        program: Option<&'a dyn Dispatch<MSG>>,
        root_node: &'a mut dyn Widget<MSG>,
    ) -> Self {
        let (width, height) =
            terminal::size().expect("must get the terminal size");

        root_node.set_size(Some((width) as f32), Some(height as f32));

        let layout_tree = compute_layout(
            root_node,
            Size {
                width: Number::Defined(width as f32),
                height: Number::Defined(height as f32),
            },
        );
        Renderer {
            write,
            program,
            root_node,
            layout_tree,
            terminal_size: (width, height),
            focused_widget_idx: None,
        }
    }

    fn recompute_layout(&mut self) {
        let (width, height) = self.terminal_size;
        self.layout_tree = compute_layout(
            self.root_node,
            Size {
                width: Number::Defined(width as f32),
                height: Number::Defined(height as f32),
            },
        );
    }

    fn dispatch_widget_event(
        &mut self,
        target_widget: &mut dyn Widget<MSG>,
        event: Event,
    ) {
        let msgs = target_widget.process_event(event);
        eprintln!(
            "done processing event... now processing msgs: {}",
            msgs.len()
        );
    }

    fn dispatch_msg(&mut self, msgs: Vec<MSG>) {
        if let Some(program) = self.program {
            eprintln!("dispatching the msgs to the program");
            for msg in msgs {
                eprintln!("dispatching msg");
                program.dispatch(msg, self.root_node);
            }
        }
        self.recompute_layout();
    }

    pub fn run(&mut self) -> Result<()> {
        command::init(&mut self.write)?;
        command::reset_top(&mut self.write)?;
        let (width, height) = self.terminal_size;

        loop {
            let mut buf = Buffer::new(width as usize, height as usize);
            eprintln!("looping...");

            buf.reset();
            {
                let tty_cmds = self.root_node.draw(&mut buf, &self.layout_tree);
                buf.render(self.write)?;
                tty_cmds.iter().for_each(|cmd| {
                    cmd.execute(self.write).expect("must execute")
                });
            }
            self.write.flush()?;

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
                                KeyCode::Char(c) => {
                                    match c {
                                        'c' | 'q' | 'd' | 'z' => {
                                            break;
                                        }
                                        _ => (),
                                    }
                                }
                                _ => (),
                            }
                        } else {
                            // send the keypresses to the focused widget
                            if let Some(idx) = self.focused_widget_idx.as_ref()
                            {
                                let active_widget: Option<
                                    &mut dyn Widget<MSG>,
                                > = find_widget_mut(self.root_node, *idx);
                                let focused_layout =
                                    find_layout(&self.layout_tree, *idx)
                                        .expect("must have a layout tree");
                                if let Some(focused_widget) = active_widget {
                                    let msgs =
                                        focused_widget.process_event(event);
                                    self.dispatch_msg(msgs);
                                }
                            }
                        }
                    }
                    // mouse clicks sets the focused the widget underneath
                    Event::Mouse(MouseEvent::Down(_btn, x, y, _modifier)) => {
                        self.focused_widget_idx = widget_node_idx_at(
                            &self.layout_tree,
                            x as f32,
                            y as f32,
                        );

                        if let Some(idx) = self.focused_widget_idx.as_ref() {
                            set_focused_node(self.root_node, *idx);
                        }
                    }
                    Event::Resize(width, height) => {
                        self.terminal_size = (width, height);
                        self.recompute_layout();
                    }
                    _ => (),
                }
                // any other activities, such as mouse scroll is
                // sent the widget underneath the location, regardless
                // if it focused or not.
                if let Some((x, y)) = extract_location(&event) {
                    let mut hits = self.layout_tree.hit(x as f32, y as f32);
                    //let hit = hits.pop().expect("process only 1 for now");
                    for hit in hits.iter().rev() {
                        let mut hit_widget: Option<&mut dyn Widget<MSG>> =
                            { find_widget_mut(self.root_node, *hit) };

                        let focused_layout =
                            find_layout(&self.layout_tree, *hit)
                                .expect("must have a layout tree");

                        if let Some(hit_widget) = &mut hit_widget {
                            let msgs = hit_widget.process_event(event);
                            eprintln!(
                            "done processing event... now processing msgs: {}",
                            msgs.len()
                        );
                            self.dispatch_msg(msgs);
                        }
                    }
                }
            }
        }
        command::finalize(self.write)?;
        Ok(())
    }
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
